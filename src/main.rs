use clap::{App, Arg};

mod io;
mod zettel;
mod database;
mod config;
mod subcommands;

use crate::zettel::Zettel;
use crate::database::Database;
use crate::config::*;
use crate::subcommands::*;

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
fn vec_to_str(vec: &[String]) -> String
{
    format!(
        "{}{}{}",
        SQL_ARRAY_SEPARATOR,
        vec.join(SQL_ARRAY_SEPARATOR),
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

fn main() -> Result<(), rusqlite::Error>
{
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("xylous <xylous.e@gmail.com>")
        .about("CLI tool to manage a digital Zettelkasten")
        .subcommand(App::new("new")
            .about("create a new Zettel and print its inbox status and title")
            .arg(Arg::new("inbox")
                .short('i')
                .long("inbox")
                .takes_value(false)
                .about("create the new Zettel in the inbox"))
            .arg(Arg::new("TITLE")
                .required(true)
                .about("title of Zettel")))
        .subcommand(App::new("update")
            .about("update the metadata of a Zettel")
            .arg(Arg::new("FILENAME")
                .required(true)
                .about("path to Zettel")))
        .subcommand(App::new("query")
            .about("return a list of Zettel whose title matches the text")
            .arg(Arg::new("PATTERN")
                .required(true)
                .about("title of Zettel")))
        .subcommand(App::new("find")
            .about("search Zettels by tag")
            .arg(Arg::new("TAG")
                .required(true)
                .about("tag of Zettel")))
        .subcommand(App::new("backlinks")
            .about("list files linking to <TITLE>")
            .arg(Arg::new("TITLE")
                .required(true)
                .about("title of Zettel")))
        .subcommand(App::new("search")
            .about("list titles of Zettel that contain provided text")
            .arg(Arg::new("TEXT")
                .required(true)
                .about("text to be searched")))
        .subcommand(App::new("list-tags")
            .about("list all tags registered in the database"))
        .subcommand(App::new("generate")
            .about("(re)generate the database"))
        .subcommand(App::new("not-created")
            .about("list Zettel linked to, but not yet created"))
        .subcommand(App::new("ls")
            .about("list all existing Zettel"))
        .subcommand(App::new("zettelkasten")
            .about("return the path to the Zettelkasten"))
        .get_matches();

    let cfg = ConfigOptions::load();
    io::mkdir(&cfg.zettelkasten);
    io::mkdir(&format!("{}/inbox", &cfg.zettelkasten));

    let cmd = matches.subcommand_name().unwrap_or_default();
    let cmd_matches;

    // If no subcommand was specified, quit
    if ! cmd.is_empty() {
        cmd_matches = matches.subcommand_matches(cmd).unwrap();
    } else {
        return Ok(());
    }

    match cmd {
        "new" => new(cmd_matches, &cfg)?,
        "update" => update(cmd_matches, &cfg)?,
        "query" => query(cmd_matches, &cfg)?,
        "find" => find(cmd_matches, &cfg)?,
        "backlinks" => backlinks(cmd_matches, &cfg)?,
        "search" => search(cmd_matches, &cfg)?,
        "list-tags" => list_tags(&cfg)?,
        "generate" => generate(&cfg)?,
        "not-created" => not_created(&cfg)?,
        "ls" => ls(&cfg)?,
        "zettelkasten" => zettelkasten_dir(&cfg)?,
        _ => (),
    };

    Ok(())
}
