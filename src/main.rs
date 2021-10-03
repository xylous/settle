use clap::{App, Arg};
use rayon::prelude::*;

mod io;
mod zettel;
mod database;
mod config;

use crate::zettel::Zettel;
use crate::database::Database;
use crate::config::*;

const SQL_ARRAY_SEPARATOR: &str = "::";

/// Join a vector of `String`s, and return a string starting and ending with `SQL_ARRAY_SEPARATOR`,
/// and with the elements of the vector separated by `SQL_ARRAY_SEPARATOR`
///
/// # Examples
///
/// ```
/// let v = vec!["foo", "bar", "baz"];
/// assert_eq!(vec_to_str(v), String::from(",foo,bar,baz,"));
/// ```
fn vec_to_str(vec: &Vec<String>) -> String
{
    format!(
        "{}{}{}",
        SQL_ARRAY_SEPARATOR,
        vec.join(&SQL_ARRAY_SEPARATOR),
        SQL_ARRAY_SEPARATOR,
    )
}

/// Split `str` on `SQL_ARRAY_SEPARATOR` and return non-empty results as a vector
fn str_to_vec(str: &str) -> Vec<String>
{
    str.split(SQL_ARRAY_SEPARATOR)
        .filter(|s| s != &"")
        .map(|s| s.to_string())
        .collect()
}

/// Return the value of $EDITOR or $VISUAL, or, if those are empty, return `"vim"`
fn default_system_editor() -> String
{
    std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| "vim".to_string())
}

fn main() -> Result<(), rusqlite::Error>
{
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("xylous <xylous.e@gmail.com>")
        .about("CLI tool to manage a digital Zettelkasten")
        .subcommand(App::new("new")
            .about("creates a new Zettel")
            .arg(Arg::new("TITLE")
                .required(true)
                .about("title of Zettel")))
        .subcommand(App::new("edit")
            .about("edit an existing Zettel")
            .arg(Arg::new("TITLE")
                .required(true)
                .about("title of Zettel")))
        .subcommand(App::new("find")
            .about("search Zettels by tag")
            .arg(Arg::new("TAG")
                .required(true)
                .about("tag of Zettel")))
        .subcommand(App::new("list-tags")
            .about("list all tags registered in the database"))
        .subcommand(App::new("generate")
            .about("(re)generate the database"))
        .subcommand(App::new("backlinks")
            .about("list files linking to <TITLE>")
            .arg(Arg::new("TITLE")
                .required(true)
                .about("title of Zettel")))
        .subcommand(App::new("not-created")
            .about("list Zettel linked to, but not yet created"))
        .subcommand(App::new("ls")
            .about("list all existing Zettel"))
        .get_matches();

    let cfg = ConfigOptions::load();
    let zettelkasten_db = format!("{}/metadata.sql", cfg.zettelkasten);

    let db = Database::new(&zettelkasten_db, None)?;
    db.init()?;

    if let Some(matches) = matches.subcommand_matches("new") {
        let title = matches.value_of("TITLE").unwrap();

        let mut zettel = Zettel::new(title).create(&cfg);
        zettel = Zettel::from_file(&zettel.filename(&cfg));
        db.save(&zettel)?;
    } else if let Some(matches) = matches.subcommand_matches("edit") {
        let title = matches.value_of("TITLE").unwrap_or_default();
        let editor = default_system_editor();
        for mut zettel in db.find_by_title(&title)? {
            zettel.edit(&editor, &cfg);
            zettel = Zettel::from_file(&zettel.filename(&cfg));
            db.delete(&zettel)?;
            db.save(&zettel)?;
        }
    } else if let Some(matches) = matches.subcommand_matches("find") {
        let tag = matches.value_of("TAG").unwrap_or_default();
        let results = db.find_by_tag(tag)?;
        println!("found {} item(s)", results.len());
        results.par_iter()
            .for_each(|z| {
                println!("{}", z.title);
            });
    } else if matches.subcommand_matches("list-tags").is_some() {
        let tags = db.list_tags()?;
        tags.into_par_iter()
            .for_each(|t|
                if !t.is_empty() {
                    println!("{}", t)
                }
            )
    } else if let Some(matches) = matches.subcommand_matches("backlinks") {
        let title = matches.value_of("TITLE").unwrap_or_default();

        let db = Database::new(&zettelkasten_db, None).unwrap();
        let links = db.find_by_links_to(title).unwrap();
        links.par_iter()
            .for_each(|l| {
                println!("{}", l.title);
            });
    } else if matches.subcommand_matches("generate").is_some() {
        let start = chrono::Local::now();

        let mem_db = Database::in_memory(&zettelkasten_db)?;
        mem_db.init()?;
        mem_db.generate(&cfg);
        mem_db.write_to(&zettelkasten_db)?;

        let end = chrono::Local::now();
        let time = end - start;
        println!("database generated successfully, took {}ms", time.num_milliseconds());
    } else if matches.subcommand_matches("not-created").is_some() {
        let results = db.zettel_not_yet_created()?;
        for title in results {
            println!("{}", title);
        }
    } else if matches.subcommand_matches("ls").is_some() {
        let results = db.all()?;
        for zettel in results {
            println!("{}", zettel.title);
        }
    }

    Ok(())
}
