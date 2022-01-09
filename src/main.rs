mod io;
mod zettel;
mod database;
mod config;
mod subcommands;
mod cli;

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
    let matches = cli::build().get_matches();

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
        "links" => links(cmd_matches, &cfg)?,
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
