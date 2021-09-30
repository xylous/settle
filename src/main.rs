use clap::{App, Arg};
use rayon::prelude::*;

mod io;
mod zettel;
mod database;

use crate::zettel::Zettel;
use crate::database::Database;

const SQL_ARRAY_SEPARATOR: &str = ",";
const ZETTELKASTEN_DB: &str = "metadata.db";

/// Join a vector of `String`s, separated by `sep`
fn vec_to_str(vec: &Vec<String>) -> String
{
    format!(
        "{}{}{}",
        SQL_ARRAY_SEPARATOR,
        vec.join(&SQL_ARRAY_SEPARATOR),
        SQL_ARRAY_SEPARATOR,
    )
}

/// Split `str` on `sep` and return results as a vector
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
        .author("xylous, xylous.e@gmail.com")
        .about("CLI tool to manage a digital zettelkasten")
        .subcommand(App::new("new")
            .about("creates a new zettel")
            .arg(Arg::new("TITLE")
                .required(true)
                .about("title of zettel")))
        .subcommand(App::new("edit")
            .about("edit an existing zettel")
            .arg(Arg::new("TITLE")
                .required(true)
                .about("title of zettel")))
        .subcommand(App::new("find")
            .about("search Zettels by tag")
            .arg(Arg::new("TAG")
                .required(true)
                .about("tag of zettel")))
        .subcommand(App::new("list-tags")
            .about("list all tags registered in the database"))
        .subcommand(App::new("generate")
            .about("generate the database in the current directory"))
        .subcommand(App::new("backlinks")
            .about("list files linking to <TITLE>")
            .arg(Arg::new("TITLE")
                .required(true)
                .about("title of zettel")))
        .get_matches();

    let db = Database::new(ZETTELKASTEN_DB, None)?;
    db.init()?;

    if let Some(matches) = matches.subcommand_matches("new") {
        let title = matches.value_of("TITLE").unwrap();

        let mut zettel = Zettel::new(title).create();
        zettel = Zettel::from_file(&zettel.filename());
        db.save(&zettel)?;
    } else if let Some(matches) = matches.subcommand_matches("edit") {
        let title = matches.value_of("TITLE").unwrap_or_default();
        let editor = default_system_editor();
        for mut zettel in db.find_by_title(&title)? {
            zettel.edit(&editor);
            zettel = Zettel::from_file(&zettel.filename());
            db.delete(&zettel)?;
            db.save(&zettel)?;
        }
    } else if let Some(matches) = matches.subcommand_matches("find") {
        let tag = matches.value_of("TAG").unwrap_or_default();
        let results = db.find_by_tag(tag)?;
        println!("found {} item(s)", results.len());
        results.par_iter()
            .for_each(|z| {
                println!("{}", z.filename());
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

        let db = Database::new(ZETTELKASTEN_DB, None).unwrap();
        let links = db.find_by_links_to(title).unwrap();
        links.par_iter()
            .for_each(|l| {
                println!("{}", l.title);
            });
    } else if matches.subcommand_matches("generate").is_some() {
        let start = chrono::Local::now();

        let mem_db = Database::in_memory(ZETTELKASTEN_DB)?;
        mem_db.init()?;
        mem_db.generate();
        mem_db.write_to(ZETTELKASTEN_DB)?;

        let end = chrono::Local::now();
        let time = end - start;
        println!("database generated successfully, took {}ms", time.num_milliseconds());
    }

    Ok(())
}
