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

pub fn sync(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let project = matches.value_of("PROJECT").unwrap_or_default();
    if let Some(title) = matches.value_of("CREATE") {
        new(cfg, title, project)?;
    } else if let Some(path) = matches.value_of("UPDATE") {
        update(cfg, path)?;
    } else if let Some(title) = matches.value_of("MOVE") {
        mv(cfg, title, project)?;
    } else if let Some(args_arr) = matches.values_of("RENAME") {
        rename(cfg, args_arr)?;
    } else if matches.is_present("GENERATE") {
        generate(cfg)?;
    }
    Ok(())
}

/// Query the database, applying various filters if proivded
pub fn query(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;

    let mut zs: Vec<Zettel> = db.all()?;

    if let Some(title) = matches.value_of("TITLE") {
        zs = filter_title(zs, title);
    }
    if let Some(project) = matches.value_of("PROJECT") {
        zs = filter_project(zs, project);
    }
    if let Some(text) = matches.value_of("TEXT_REGEX") {
        zs = filter_text(zs, text, cfg);
    }
    if let Some(tag) = matches.value_of("TAG") {
        zs = filter_tag(zs, tag);
    }
    if let Some(linked_from) = matches.value_of("LINKS") {
        zs = intersect(&zs, &fwlinks(&db.all()?, linked_from));
    }
    if let Some(links_to) = matches.value_of("BACKLINKS") {
        zs = intersect(&zs, &backlinks(&zs, links_to));
    }
    if matches.is_present("LONERS") {
        zs = filter_isolated(zs);
    }

    if let Some(format) = matches.value_of("FORMAT") {
        let link_sep = matches.value_of("LINK_SEP").unwrap_or(" | ");
        zettelkasten_format(cfg,
                            &zs,
                            &replace_literals(format),
                            &replace_literals(link_sep))
    } else {
        zettelkasten_format(cfg, &zs, "[%p] %t", "")
    }
    Ok(())
}

fn replace_literals(s: &str) -> String
{
    s.replace(r"\n", "\n").replace(r"\t", "\t")
}

/// Print things that aren't directly related to notes.
pub fn ls(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;

    let m = matches.value_of("OBJECT").unwrap_or_default();
    // TODO: maybe implement word suggestion? actually, that'd be quite useless
    match m {
        "tags" => print_list_of_strings(&db.list_tags()?),
        "ghosts" => print_list_of_strings(&db.zettel_not_yet_created()?),
        "projects" => print_list_of_strings(&db.list_projects()?),
        "path" => println!("{}", cfg.zettelkasten),
        _ => eprintln!("error: expected one of: 'tags', 'ghosts', 'projects', 'path'; got '{}'",
                       m),
    }
    Ok(())
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

/// Print every given Zettel according to the specified ormat
fn zettelkasten_format(cfg: &ConfigOptions, zs: &[Zettel], fmt: &str, link_sep: &str)
{
    zs.iter().for_each(|z| {
                 zettel_format(cfg, z, fmt, link_sep);
             });
}

// Print according to a certain format, replacing the following placeholder tokens
//
//  %t - title
//  %p - project
//  %P - path
//  %l - (forward) links
//  %b - backlinks
fn zettel_format(cfg: &ConfigOptions, z: &Zettel, fmt: &str, link_sep: &str)
{
    let mut result = fmt.to_string();

    result = result.replace("%t", &z.title);
    result = result.replace("%p", &z.project);
    result = result.replace("%P", &z.filename(cfg));
    result = result.replace("%l", &z.links.join(link_sep));
    // Based on the provided ConfigOptions, we may or may not get the backlinks for the given
    // Zettel, so if we don't, we just consume the `%b` token and move on
    if result.contains("%b") {
        let maybe_get_backlinks = || -> Result<Vec<String>, Error> {
            let all = Database::new(&cfg.db_file())?.all()?;
            let bks = backlinks(&all, &z.title);
            Ok(bks.iter().map(|z| z.title.clone()).collect())
        };
        if let Ok(bks) = maybe_get_backlinks() {
            result = result.replace("%b", &bks.join(link_sep));
        }
    }

    println!("{}", result);
}

/// Print every element in the list of Strings on an individual line
fn print_list_of_strings(elems: &[String])
{
    elems.iter().for_each(|e| {
                    println!("{}", e);
                })
}

/// Keep only those Zettel whose title matches the provided regex
fn filter_title(zs: Vec<Zettel>, pattern: &str) -> Vec<Zettel>
{
    let re = Regex::new(&format!("^{}$", pattern)).unwrap();
    zs.into_iter().filter(|z| re.is_match(&z.title)).collect()
}

/// Keep only those Zettel whose project matches the provided regex
fn filter_project(zs: Vec<Zettel>, pattern: &str) -> Vec<Zettel>
{
    let re = Regex::new(&format!("^{}$", pattern)).unwrap();
    zs.into_iter().filter(|z| re.is_match(&z.project)).collect()
}

/// Keep only those Zettel that contain the pattern in their text
fn filter_text(zs: Vec<Zettel>, pattern: &str, cfg: &ConfigOptions) -> Vec<Zettel>
{
    zs.into_iter()
      .filter(|z| z.has_text(cfg, pattern))
      .collect()
}

/// Keep only those Zettel that have at least one tag that matches the regex
fn filter_tag(zs: Vec<Zettel>, pattern: &str) -> Vec<Zettel>
{
    let re = Regex::new(&format!("^{}$", pattern)).unwrap();
    zs.into_iter()
      .filter(|z| {
          for t in &z.tags {
              if re.is_match(t) {
                  return true;
              }
          }
          false
      })
      .collect()
}

/// Keep only those Zettel that neither link to other notes, nor have links pointing to them
fn filter_isolated(zs: Vec<Zettel>) -> Vec<Zettel>
{
    zs.clone()
      .into_iter()
      .filter(|z| z.links.is_empty() && backlinks(&zs, &z.title).is_empty())
      .collect()
}

/// Keep only the Zettel that are both in A and B
fn intersect<T: Eq + Clone>(a: &[T], b: &[T]) -> Vec<T>
{
    a.to_owned()
     .iter()
     .cloned()
     .filter(|z| b.contains(z))
     .collect::<Vec<_>>()
}

/// Return all the Zettel that are linked to by the pattern-matched zettel
fn fwlinks(all: &[Zettel], linked_from: &str) -> Vec<Zettel>
{
    // first find the Zettel that match the query, then find the notes that have been linked to by
    // them
    let fwlinks = &filter_title(all.to_owned(), linked_from);
    all.iter()
       .cloned()
       .filter(|z| {
           for fw in fwlinks {
               if fw.links.contains(&z.title) {
                   return true;
               }
           }
           false
       })
       .collect()
}

/// Return all the Zettel that link to the given title/pattern, within the provided list of Zettel
fn backlinks(all: &[Zettel], links_to: &str) -> Vec<Zettel>
{
    let re = Regex::new(&format!("^{}$", links_to)).unwrap();
    all.iter()
       .cloned()
       .filter(|z| {
           for l in &z.links {
               if re.is_match(l) {
                   return true;
               }
           }
           false
       })
       .collect()
}

/// Based on the CLI arguments and the config options, *maybe* add a new entry to the database
fn new(cfg: &ConfigOptions, title: &str, project: &str) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;
    db.init()?;

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
        zettel_format(cfg, &zettel, "[%p] %t", "")
    }
    db.save(&zettel)?;

    Ok(())
}

/// Rename a note, but keep it in the same project
fn rename(cfg: &ConfigOptions, arr: clap::Values<'_>) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;

    // basically, look thru the values provided by clap, and extract the first Zettel title that
    // exists and is different from the new title
    let old_title = arr.clone()
                       .into_iter()
                       .find(|x| db.find_by_title(x).unwrap_or_default().is_empty())
                       .unwrap_or("");
    let new_title = arr.clone().next_back().unwrap_or_default();

    if old_title == new_title {
        eprintln!("error: first match is the same as the new title ('{}'), so no rename",
                  old_title);
        return Ok(());
    }

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
        let backlinks = backlinks(&db.all()?, old_title);
        // for some reason rustfmt has absolutely cursed formatting here. this is not my fault, I
        // swear
        backlinks.iter().for_each(|bl| {
                            let contents = crate::io::file_to_string(&bl.filename(cfg));
                            // The link might span over multiple lines. We must account for that
                            let regex_string =
                                &format!(r"\[\[{}\]\]", old_title).replace(' ', r"[\n\t ]");
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
fn mv(cfg: &ConfigOptions, pattern: &str, project: &str) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;

    let zs = db.find_by_title(pattern)?;

    zettelkasten_format(cfg, &zs, "[%p] %t", "");

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
        let new_notes = zs.iter().map(|z| Zettel { title: z.title.clone(),
                                                   project: project.to_string(),
                                                   links: z.links.clone(),
                                                   tags: z.tags.clone() });
        let pairs = zs.iter().zip(new_notes);
        pairs.for_each(|(old, new)| {
                 crate::io::rename(&old.filename(cfg), &new.filename(cfg));
                 db.change_project(old, project).unwrap();
             });
    }

    Ok(())
}

/// Update the metadata of a file
fn update(cfg: &ConfigOptions, path: &str) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;

    if file_exists(path) {
        let zettel = Zettel::from_file(cfg, path);
        db.update(cfg, &zettel)?;
    } else {
        eprintln!("error: provided path isn't a file");
    }

    Ok(())
}

/// (Re)generate the database file
fn generate(cfg: &ConfigOptions) -> Result<(), Error>
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
