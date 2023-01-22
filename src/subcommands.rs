use clap::ArgMatches;
use clap_complete::Shell::*;
use rayon::prelude::*;
use regex::Regex;
use rusqlite::Error;

use crate::config::ConfigOptions;
use crate::Database;
use crate::Zettel;

use crate::cli;
use crate::io::file_exists;

/// Print `[<PROJECT>] <TITLE>` for every given zettel.
fn print_zettel_info(zettel: &[Zettel])
{
    zettel.iter().for_each(|z| {
                     println!("[{}] {}", z.project, z.title);
                 })
}

/// Print every element in the list of Strings on an individual line
fn print_list_of_strings(elems: &Vec<String>)
{
    elems.iter().for_each(|e| {
                    println!("{}", e);
                })
}

/// Generate completions for a shell
pub fn compl(matches: &ArgMatches) -> Result<(), Error>
{
    let shell = matches.value_of("SHELL").unwrap_or_default();

    let sh = match shell {
        "zsh" => Some(Zsh),
        "bash" => Some(Bash),
        "fish" => Some(Fish),
        _ => None,
    };

    if let Some(sh) = sh {
        let app = &mut cli::build();
        clap_complete::generate(sh, app, app.get_name().to_string(), &mut std::io::stdout());
    } else {
        eprintln!("error: '{}' isn't a (supported) shell", shell);
    }

    Ok(())
}

/// Based on the CLI arguments and the config options, *maybe* add a new entry to the database
pub fn new(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;
    db.init()?;

    let title = matches.value_of("TITLE").unwrap();
    let project = matches.value_of("PROJECT").unwrap_or_default();

    let zettel = Zettel::new(title, project);

    let exists_in_fs = file_exists(&zettel.filename(cfg));
    let exists_in_db = db.all().unwrap().into_par_iter().any(|z| z == zettel);

    // If the corresponding file exists and there's an entry in the database, abort.
    // If there's a file but there's no entry in the database, create an entry.
    // Otherwise, create a new file from template and add a database entry.
    if exists_in_fs && exists_in_db {
        eprintln!("error: couldn't create new Zettel: one with the same title already exists");
        return Ok(());
    } else if exists_in_fs {
        println!("file exists in the filesystem but not in the database; added entry");
        // saved outside of the loop
    } else {
        zettel.create(cfg);
        print_zettel_info(&[zettel.clone()]); // confirm that the Zettel was created
    }
    db.save(&zettel)?;

    Ok(())
}

/// Rename a note, but keep it in the same project
pub fn rename(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;
    let old_title = matches.value_of("TITLE").unwrap();
    let new_title = matches.value_of("NEW_TITLE").unwrap();

    let results = db.find_by_title(old_title)?;

    // check if there's already a note with this title
    let overwrite_failsafe = db.find_by_title(new_title)?;
    if overwrite_failsafe.first().is_some() {
        eprintln!("error: a note with the new title already exists: won't overwrite");
        return Ok(());
    }

    let old_zettel = if results.first().is_none() {
        eprintln!("error: no Zettel with that title");
        return Ok(());
    } else {
        results.first().unwrap()
    };
    let new_zettel = Zettel::new(new_title, &old_zettel.project);

    let mut dial = dialoguer::Confirm::new();
    let prompt = dial.with_prompt(format!("{} --> {}", old_title, new_title));

    // If the user confirms, change the note's title, and update the links to this Zettel
    if prompt.interact().unwrap_or_default() {
        crate::io::rename(&old_zettel.filename(cfg), &new_zettel.filename(cfg));
        db.change_title(old_zettel, new_title).unwrap();
        // It's not enough that we renamed the file. We need to update all references to it!
        let backlinks = db.find_by_links_to(old_title)?;
        backlinks.iter().for_each(|bl| {
                            let contents = crate::io::file_to_string(&bl.filename(cfg));
                            // The link might span over multiple lines. We must account for that
                            let regex_string =
                                &format!(r"\[\[{}\]\]", old_title).replace(" ", r"[\n\t ]");
                            let old_title_reg = Regex::new(regex_string).unwrap();
                            let new_contents =
                                old_title_reg.replace_all(&contents, format!(r"[[{}]]", new_title));
                            crate::io::write_to_file(&bl.filename(cfg), &new_contents);
                            db.update(cfg, bl).unwrap();
                        })
    }

    Ok(())
}

/// Move all matching notes into a project
pub fn mv(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;
    let pattern = matches.value_of("PATTERN").unwrap();
    let project = matches.value_of("PROJECT").unwrap();

    let notes = db.find_by_title(pattern)?;

    print_zettel_info(&notes);

    let mut dial = dialoguer::Confirm::new();
    let prompt =
        dial.with_prompt(format!(">> These notes will be transferred to the {}. Proceed?",
                                 if project.is_empty() {
                                     "main zettelkasten".to_string()
                                 } else {
                                     format!("'{}' project", project)
                                 }));

    // If the user confirms, change the notes' projects, both the system path and in database
    if prompt.interact().unwrap_or_default() {
        crate::io::mkdir(&format!("{}/{}", cfg.zettelkasten, project));
        let new_notes = notes.iter().map(|z| Zettel { title: z.title.clone(),
                                                      project: project.to_string(),
                                                      links: z.links.clone(),
                                                      tags: z.tags.clone() });
        let pairs = notes.iter().zip(new_notes);
        pairs.for_each(|(old, new)| {
                 crate::io::rename(&old.filename(cfg), &new.filename(cfg));
                 db.change_project(old, project).unwrap();
             });
    }

    Ok(())
}

/// Update the metadata of a file
pub fn update(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;

    let path = matches.value_of("FILENAME").unwrap();
    if file_exists(path) {
        let zettel = Zettel::from_file(cfg, path);
        db.update(cfg, &zettel)?;
    } else {
        eprintln!("error: provided path isn't a file");
    }

    Ok(())
}

/// Query various things in the database, based on the provided ArgMatches arguments.
/// The queries for `--title`, `--text`, `--isolated`, and `--by_tag` compound - that is to say,
/// the Zettel filtered for have to pass all the provided criteria in order to pass. All other
/// queries have their own use case, printing different things
pub fn query(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;
    if matches.is_present("PROJECTS") {
        let query = db.list_projects()?;
        print_list_of_strings(&query);
        return Ok(());
    } else if matches.is_present("TAGS") {
        let query = db.list_tags()?;
        print_list_of_strings(&query);
        return Ok(());
    }
    if matches.is_present("GHOSTS") {
        let query = db.zettel_not_yet_created()?;
        print_list_of_strings(&query);
        return Ok(());
    }

    if matches.is_present("FWLINKS") {
        let query = matches.value_of("FWLINKS").unwrap_or("*");
        fwlinks(&db, query)?;
        return Ok(());
    }
    if matches.is_present("BACKLINKS") {
        let query = matches.value_of("BACKLINKS").unwrap_or("*");
        backlinks(&db, query)?;
        return Ok(());
    }

    let mut zs: Vec<Zettel> = db.all()?;

    if matches.is_present("TITLE") {
        let query = matches.value_of("TITLE").unwrap_or("*");
        let crit = db.find_by_title(query)?;
        zs = filter(zs, crit);
    }
    if matches.is_present("TEXT") {
        let query = matches.value_of("TEXT").unwrap_or("*");
        let crit = db.search_text(cfg, query)?;
        zs = filter(zs, crit);
    }
    if matches.is_present("BY_TAG") {
        let query = matches.value_of("BY_TAG").unwrap_or("*");
        let crit = by_tag(&db, query)?;
        zs = filter(zs, crit);
    }
    if matches.is_present("ISOLATED") {
        zs = filter(zs, isolated(&db)?);
    }
    print_zettel_info(&zs);
    Ok(())
}

/// Keep only those elements of `old` that are also in `criteria`
fn filter(old: Vec<Zettel>, criteria: Vec<Zettel>) -> Vec<Zettel>
{
    old.into_iter().filter(|z| criteria.contains(z)).collect()
}

/// Print all Zettel whose tags contain the pattern specified in the CLI args
pub fn by_tag(db: &Database, query: &str) -> Result<Vec<Zettel>, Error>
{
    let mut zettel = db.find_by_tag(query)?;
    let mut zettel_with_subtag = db.find_by_tag(&format!("{}/*", query))?;
    zettel.append(&mut zettel_with_subtag);
    zettel.par_sort();
    zettel.dedup();

    Ok(zettel)
}

/// Print the titles of the Zettel matching the pattern provided in the CLI arguments and the other
/// Zettel it links to under the following format:
///
/// ```
/// [<PROJECT>] <TITLE>
///     | <LINK_1>
///     | <LINK_2>
///     | ...
///     | <LINK_N>
/// ```
pub fn fwlinks(db: &Database, query: &str) -> Result<(), Error>
{
    let zettel = db.find_by_title(query)?;
    for z in zettel {
        print_zettel_info(&[z.clone()]);
        for link in &z.links {
            println!("    | {}", link);
        }
    }
    Ok(())
}

/// Print all Zettel that match the one specified in the CLI argument matches
pub fn backlinks(db: &Database, query: &str) -> Result<(), Error>
{
    let zettel = db.find_by_title(query)?;
    for z in zettel {
        print_zettel_info(&[z.clone()]);
        let res = db.find_by_links_to(&z.title)?;
        for blink in res {
            print!("    | ");
            print_zettel_info(&[blink]);
        }
    }
    Ok(())
}

/// Print the list of Zettel IN THE MAIN ZETTELKASTEN that aren't linked with other notes
pub fn isolated(db: &Database) -> Result<Vec<Zettel>, Error>
{
    let all = db.find_by_title("*")?;
    let isolated = all.iter()
                      .filter(|z| {
                          // skip finding backlinks if the given Zettel isn't in the main Zettelkasten project or
                          // it has "forward" links
                          if z.project != "" || z.links.len() != 0 {
                              return false;
                          }
                          let backlinks = db.find_by_links_to(&z.title).unwrap_or_default();
                          backlinks.len() == 0
                      })
                      .cloned()
                      .collect::<Vec<Zettel>>();
    Ok(isolated)
}

/// (Re)generate the database file
pub fn generate(cfg: &ConfigOptions) -> Result<(), Error>
{
    let start = std::time::Instant::now();

    let mem_db = Database::in_memory(&cfg.db_file())?;
    mem_db.init()?;
    mem_db.generate(cfg);
    mem_db.write_to(&cfg.db_file())?;

    println!("database generated successfully, took {}ms",
             start.elapsed().as_millis());

    Ok(())
}

pub fn zk(cfg: &ConfigOptions) -> Result<(), Error>
{
    println!("{}", cfg.zettelkasten);
    Ok(())
}
