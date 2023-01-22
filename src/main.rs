mod cli;
mod config;
mod database;
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
    // If no subcommand was specified, quit
    let cmd_matches = if cmd.is_empty() {
        return Ok(());
    } else {
        matches.subcommand_matches(cmd).unwrap()
    };

    match cmd {
        "compl" => compl(cmd_matches)?,
        "query" => query(cmd_matches, &cfg)?,
        "new" => new(cmd_matches, &cfg)?,
        "update" => update(cmd_matches, &cfg)?,
        "mv" => mv(cmd_matches, &cfg)?,
        "rename" => rename(cmd_matches, &cfg)?,
        "generate" => generate(&cfg)?,
        "zk" => zk(&cfg)?,
        _ => (),
    };

    Ok(())
}
