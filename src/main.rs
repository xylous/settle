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

fn main() -> Result<(), rusqlite::Error>
{
    let matches = cli::build().get_matches();

    let cmd = matches.subcommand_name().unwrap_or_default();
    // NOTE: this won't crash on unwrap, because if no subcommand was specified, clap-rs would
    // print the help message
    let cmd_matches = matches.subcommand_matches(cmd).unwrap();

    match cmd {
        "sync" => sync(cmd_matches, &ConfigOptions::load())?,
        "query" => query(cmd_matches, &ConfigOptions::load())?,
        "ls" => ls(cmd_matches, &ConfigOptions::load())?,
        "compl" => compl(cmd_matches)?,
        _ => (),
    };

    Ok(())
}
