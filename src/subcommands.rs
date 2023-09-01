use clap::ArgMatches;
use clap_complete::Shell::*;
use regex::Regex;
use rusqlite::Error;

use crate::config::ConfigOptions;
use crate::graph::{vizk, zk_graph_dot_output, zk_graph_json_output};
use crate::zettel::strip_multiple_whitespace;
use crate::Database;
use crate::Zettel;

use crate::cli;
use crate::io::{abs_path, file_exists};

/// A printer that prints. Because it's convenient.
struct Printer
{
    zettel: Vec<Zettel>,
    additional: Vec<String>,
    // Print according to a certain format, replacing the following placeholder tokens
    //
    //  %t - title
    //  %p - project
    //  %P - path
    //  %l - (forward) links
    //  %b - backlinks
    //  %a - contents of the `additional` field (--text flag fills this with the matched pattern)
    format: String,
    link_separator: String,
}

impl Printer
{
    fn set_zettelkasten(&mut self, new_zettel: Vec<Zettel>)
    {
        self.zettel = new_zettel;
    }

    fn set_additional(&mut self, new_additional: Vec<String>)
    {
        self.additional = new_additional;
    }

    fn set_format(&mut self, new_format: String)
    {
        self.format = new_format;
    }

    fn set_link_separator(&mut self, new_separator: String)
    {
        self.link_separator = new_separator;
    }

    fn print_one(&mut self, cfg: &ConfigOptions, zettel: Zettel)
    {
        self.zettel = vec![zettel];
        self.print(cfg);
    }

    /// Abracadabra, yadda yadda. Print everything properly.
    fn print(&mut self, cfg: &ConfigOptions)
    {
        // basically. since there's a `zip()` call, if the `additional` vector is smaller in length
        // than the `zettel` vector, then some things won't get printed.
        // this really only makes sure that everything gets printed
        let len_diff = self.zettel.len() - self.additional.len();
        let mut empty_diff = vec!["".to_string(); len_diff];
        self.additional.append(&mut empty_diff);

        let mut zip = self.zettel.iter().zip(&self.additional).collect::<Vec<_>>();
        zip.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for (z, a) in zip {
            let mut result = self.format.to_string();

            result = result.replace("%t", &z.title);
            result = result.replace("%p", &z.project);
            result = result.replace("%P", &z.filename(cfg));
            result = result.replace("%l", &z.links.join(&self.link_separator));
            result = result.replace("%a", a);
            result = result.replace("%b", &z.backlinks.join(&self.link_separator));

            println!("{}", result);
        }
    }
}

impl Default for Printer
{
    fn default() -> Printer
    {
        Printer { zettel: vec![],
                  additional: vec![],
                  format: "[%p] %t".to_string(),
                  link_separator: "|".to_string() }
    }
}

pub fn sync(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    Database::new(&cfg.db_file())?.init()?;

    let project = realproject(if let Some(p) = matches.get_one::<String>("PROJECT") {
                                  p
                              } else {
                                  ""
                              });
    if let Some(title) = matches.get_one::<String>("CREATE") {
        create(cfg, title, project)?;
    } else if let Some(path) = matches.get_one::<String>("UPDATE") {
        update(cfg, path)?;
    } else if let Some(title) = matches.get_one::<String>("MOVE") {
        mv(cfg, title, project)?;
    } else if matches.contains_id("RENAME") {
        let args = matches.get_many::<String>("RENAME")
                          .unwrap_or_default()
                          .map(|a| a.to_string())
                          .collect::<Vec<String>>();
        rename(cfg, &args[0], &args[1])?;
    } else if matches.get_flag("GENERATE") {
        generate(cfg)?;
    }
    Ok(())
}

/// Query the database, applying various filters if proivded
pub fn query(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;
    db.init()?;

    let mut zs: Vec<Zettel> = db.all()?;
    let mut printer = Printer::default();

    let exact = matches.get_flag("EXACT_MATCH");

    if let Some(title) = matches.get_one::<String>("TITLE") {
        zs = filter_title(zs, title, exact);
    }
    if let Some(project) = matches.get_one::<String>("PROJECT") {
        zs = filter_project(zs, realproject(project), exact);
    }
    if let Some(tag) = matches.get_one::<String>("TAG") {
        zs = filter_tag(zs, tag, exact);
    }
    if let Some(linked_from) = matches.get_one::<String>("LINKS") {
        zs = intersect(&zs, &fwlinks(&db.all()?, linked_from, exact));
    }
    if let Some(links_to) = matches.get_one::<String>("BACKLINKS") {
        zs = intersect(&zs, &backlinks(&zs, links_to, exact));
    }
    if let Some(text) = matches.get_one::<String>("TEXT_REGEX") {
        let vs = filter_text(zs.clone(), text, cfg);
        let mut texts: Vec<String> = vec![];
        let mut found = vec![];
        // maybe speed this up by using unzip? I digress
        for (z, t) in vs {
            found.push(z);
            texts.push(t);
        }
        printer.set_additional(texts);
        zs = intersect(&zs, &found);
    }

    if matches.get_flag("LONERS") {
        zs = filter_isolated(zs);
    }

    if let Some(graph) = matches.get_one::<String>("GRAPH") {
        match graph.as_str() {
            "vizk" => vizk(&zs),
            "dot" => zk_graph_dot_output(&zs),
            "json" => zk_graph_json_output(&zs),
            _ => {
                eprintln!("error: expected one of 'json', 'dot', 'vizk' (got: '{}')",
                          graph);
            }
        }
        return Ok(());
    }

    if let Some(format) = matches.get_one::<String>("FORMAT") {
        let link_sep = matches.get_one::<&str>("LINK_SEP").unwrap_or(&" | ");
        printer.set_format(replace_literals(format));
        printer.set_link_separator(replace_literals(link_sep));
    }

    printer.set_zettelkasten(zs);

    printer.print(cfg);

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
    db.init()?;

    let obj = if let Some(m) = matches.get_one::<String>("OBJECT") {
        m
    } else {
        ""
    };

    // TODO: maybe implement word suggestion? actually, that'd be quite useless
    match obj {
        "tags" => print_list_of_strings(&db.list_tags()?),
        "ghosts" => print_list_of_strings(&db.zettel_not_yet_created()?),
        "projects" => print_list_of_strings(&db.list_projects()?),
        "path" => println!("{}", cfg.zettelkasten),
        _ => eprintln!("error: expected one of: 'tags', 'ghosts', 'projects', 'path'; got '{}'",
                       obj),
    }
    Ok(())
}

/// Generate completions for a shell
pub fn compl(matches: &ArgMatches) -> Result<(), Error>
{
    let shell = matches.get_one::<String>("SHELL").unwrap();

    let sh = match shell.as_str() {
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

/// Print every element in the list of Strings on an individual line
fn print_list_of_strings(elems: &[String])
{
    let mut sorted = elems.to_vec();
    sorted.sort();
    sorted.iter().for_each(|e| {
                     println!("{}", e);
                 })
}

/// Keep only those Zettel whose title matches the provided regex
fn filter_title(zs: Vec<Zettel>, pattern: &str, exact: bool) -> Vec<Zettel>
{
    let re = Regex::new(&format!("^{}$", pattern)).unwrap();
    zs.into_iter()
      .filter(|z| {
          if exact {
              pattern == z.title
          } else {
              re.is_match(&z.title)
          }
      })
      .collect()
}

/// Keep only those Zettel whose project matches the provided regex
fn filter_project(zs: Vec<Zettel>, pattern: &str, exact: bool) -> Vec<Zettel>
{
    let re = Regex::new(&format!("^{}$", pattern)).unwrap();
    zs.into_iter()
      .filter(|z| {
          if exact {
              pattern == z.project
          } else {
              re.is_match(&z.project)
          }
      })
      .collect()
}

/// Keep only those Zettel that contain the pattern in their text
fn filter_text(zs: Vec<Zettel>, pattern: &str, cfg: &ConfigOptions) -> Vec<(Zettel, String)>
{
    zs.into_iter()
      .map(|z| (z.clone(), z.find_pattern(cfg, pattern)))
      .filter(|(_, t)| !t.is_empty())
      .collect()
}

/// Keep only those Zettel that have at least one tag (or subtag) that matches the regex
fn filter_tag(zs: Vec<Zettel>, pattern: &str, exact: bool) -> Vec<Zettel>
{
    let re = Regex::new(&format!("^{}(/.*)?$", pattern)).unwrap();
    zs.into_iter()
      .filter(|z| {
          for tag in &z.tags {
              if pattern == tag || (!exact && re.is_match(tag)) {
                  return true;
              };
          }
          false
      })
      .collect()
}

/// Keep only those Zettel that neither link to other notes, nor have links pointing to them
fn filter_isolated(zs: Vec<Zettel>) -> Vec<Zettel>
{
    zs.into_iter()
      .filter(|z| z.links.is_empty() && z.backlinks.is_empty())
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
fn fwlinks(all: &[Zettel], linked_from: &str, exact: bool) -> Vec<Zettel>
{
    // first find the Zettel that match the query, then find the notes that have been linked to by
    // them
    let fwlinks = &filter_title(all.to_owned(), linked_from, exact);
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
fn backlinks(all: &[Zettel], links_to: &str, exact: bool) -> Vec<Zettel>
{
    let re = Regex::new(&format!("^{}$", links_to)).unwrap();
    all.iter()
       .cloned()
       .filter(|z| {
           for l in z.links.clone() {
               if links_to == l || (!exact && re.is_match(&l)) {
                   return true;
               };
           }
           false
       })
       .collect()
}

/// Based on the CLI arguments and the config options, *maybe* add a new entry to the database
fn create(cfg: &ConfigOptions, title: &str, project: &str) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;

    let actual_title = strip_multiple_whitespace(title);
    let zettel = Zettel::new(&actual_title, project);

    if actual_title != title {
        eprintln!("warning: truncating newlines, tabs and/or multiple consecutive whitespaces");
    }

    // reject bad formats
    if zettel.title.is_empty() || zettel.title.starts_with('.') {
        eprintln!("error: empty/dotfile titles are not accepted");
        return Ok(());
    }

    let exists_in_fs = file_exists(&zettel.filename(cfg));
    let exists_in_db = db.find_by_title(&zettel.title)?.len() == 1;

    // If the corresponding file exists and there's an entry in the database, abort.
    // If there's an entry in the database but no corresponding file, replace the database entry
    // If there's a file but there's no entry in the database, create an entry.
    // Otherwise, create a new file from template and add a database entry.
    if exists_in_fs && exists_in_db {
        eprintln!("error: couldn't create new Zettel: one with the same title already exists");
        return Ok(());
    } else if !exists_in_fs && exists_in_db {
        eprintln!("Zettel exists in the database but not on the filesystem; replaced entry");
        db.delete(&zettel)?;
        // saved outside of the loop
    } else if exists_in_fs && !exists_in_db {
        eprintln!("Zettel exists on the filesystem but not in the database; added entry");
        // saved outside of the loop
    } else {
        zettel.create(cfg);
        Printer::default().print_one(cfg, zettel.clone());
    }
    db.save(&zettel)?;

    Ok(())
}

/// Rename a note, but keep it in the same project
fn rename(cfg: &ConfigOptions, old_title: &str, new_title: &str) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;

    // basically, look thru the values provided by clap, and extract the first Zettel title that
    // exists and is different from the new title

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
        let backlinks = backlinks(&db.all()?, old_title, true);
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

    let re = Regex::new(pattern).unwrap();
    let zs: Vec<Zettel> = db.all()?
                            .into_iter()
                            .filter(|z| re.is_match(&z.title))
                            .collect();

    let mut printer = Printer::default();
    printer.set_zettelkasten(zs.clone());
    printer.print(cfg);

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
                                                   backlinks: z.backlinks.clone(),
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

    let path_abs = abs_path(path);

    if file_exists(&path_abs) {
        let zettel = Zettel::from_file(cfg, &path_abs);
        if file_exists(&zettel.filename(cfg)) {
            db.update(cfg, &zettel)?;
        } else {
            eprintln!("error: file is not in the Zettelkasten");
        }
    } else {
        eprintln!("error: provided path isn't a file");
    }

    Ok(())
}

/// (Re)generate the database file
fn generate(cfg: &ConfigOptions) -> Result<(), Error>
{
    let start = std::time::Instant::now();

    let mem_db = Database::new_in_memory(&cfg.db_file())?;
    mem_db.init()?;
    mem_db.generate(cfg)?;
    mem_db.write_to(&cfg.db_file())?;

    println!("database generated successfully, took {}ms",
             start.elapsed().as_millis());

    Ok(())
}

/// Given a project name-alias, return the real project's name
/// Note how `main` is an alias for the main Zettelkasten, whose real project is `` (empty string)
fn realproject(project: &str) -> &str
{
    match project {
        "main" | "" => "", // main project
        any => any,        // any other project
    }
}
