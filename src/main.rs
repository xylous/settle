mod cli;
mod config;
mod database;
mod graph;
mod io;
mod subcommands;
mod zettel;

use crate::config::*;
use crate::database::Database;
use crate::subcommands::*;
use crate::zettel::Zettel;

const SQL_ARRAY_SEPARATOR: &str = "::";

/// Join a vector of `String`s, and return a string starting and ending with `SQL_ARRAY_SEPARATOR`,
/// and with the elements of the vector separated by `SQL_ARRAY_SEPARATOR`
fn vec_to_str(vec: &[String]) -> String
{
    format!("{}{}{}",
            SQL_ARRAY_SEPARATOR,
            vec.join(SQL_ARRAY_SEPARATOR),
            SQL_ARRAY_SEPARATOR,)
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
    let matches = cli::build().get_matches();

    let cfg = ConfigOptions::load();
    io::mkdir(&cfg.zettelkasten);
    io::mkdir(&format!("{}/inbox", &cfg.zettelkasten));

    let cmd = matches.subcommand_name().unwrap_or_default();
    // NOTE: this won't crash on unwrap, because if no subcommand was specified, clap-rs would
    // print the help message
    let cmd_matches = matches.subcommand_matches(cmd).unwrap();

    // If the database isn't initialised, the program would likely panic, complaining that it isn't
    // able to find SQL tables.
    // Yes, it's kind of lazy to put this here, but putting it in every function that accesses the
    // database, just to make sure that the database exists in the first place, is worse.
    Database::new(&cfg.db_file())?.init()?;

    match cmd {
        "sync" => sync(cmd_matches, &cfg)?,
        "query" => query(cmd_matches, &cfg)?,
        "ls" => ls(cmd_matches, &cfg)?,
        "compl" => compl(cmd_matches)?,
        _ => (),
    };

    Ok(())
}
