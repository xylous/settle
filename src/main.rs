use clap::{App, Arg};
use std::env;
use rusqlite::{Connection, Result};

mod io;
mod zettel;

use crate::io::*;
use crate::zettel::Zettel;

const FILENAME_SEPARATOR: &str = "::";
const ZETTELKASTEN_DB: &str = ".zettelkasten.db";
const LUA_FILTER_SCRIPT: &str = ".md_links_to_html.lua";

/// Return a timestamp with the format YYYYMMDDhhmmss
/// where YYYY = year,
///         MM = month,
///         DD = day,
///         hh = hour,
///         mm = minute,
///         ss = second
///
/// # Examples
///
/// ```
/// let id = id_timestamp();
/// ```
fn id_timestamp() -> String
{
    let dt = chrono::offset::Local::now();
    dt.format("%Y%m%d%H%M%S").to_string()
}

/// Return the value of $EDITOR or $VISUAL, or, if those are empty, return `"vim"`
fn default_system_editor() -> String
{
    env::var("EDITOR")
        .or(env::var("VISUAL"))
        .unwrap_or("vim".to_string())
}

/// Create table `zettelkasten` in database `conn` if it doesn't exist already
///
/// The table `zettelkasten` has two properties: `id` and `title`, both of type `TEXT`
fn initialize_db(conn: &Connection) -> Result<(), rusqlite::Error>
{
    conn.execute(
        "CREATE TABLE IF NOT EXISTS zettelkasten (
            id      TEXT PRIMARY KEY,
            title   TEXT NOT NULL
        )",
        []).expect("failed to create database");
    Ok(())
}

/// Creates a Lua script that will be used by pandoc to replace links ending in `.md` with links
/// ending in `.html`
fn create_lua_filter() -> ()
{
    if path_exists(LUA_FILTER_SCRIPT) {
        return;
    }
    let lua_script =
r#"-- this script replaces all links ending in `.md` with ones ending in `.html`
-- it will used by pandoc when building the Zettelkasten
function Link(el)
    el.target = string.gsub(el.target, "%.md", ".html")
    return el
end
"#;
    write_to_file(LUA_FILTER_SCRIPT, lua_script);
}

fn main() -> Result<(), rusqlite::Error>
{
    let matches = App::new("settler")
        .version(env!("CARGO_PKG_VERSION"))
        .about("CLI tool to manage a digital zettelkasten")
        .subcommand(App::new("new")
            .about("creates a new zettel")
            .arg(Arg::new("TITLE")
                .required(true)
                .about("title of zettel")))
        .subcommand(App::new("build")
            .about("compile a zettel to html\nuses SQL syntax, e.g. `%` to match one or more characters")
            .arg(Arg::new("ID")
                .required(true)
                .about("id of zettel")))
        .get_matches();

    let conn = Connection::open(ZETTELKASTEN_DB)?;
    initialize_db(&conn).unwrap();

    if let Some(ref matches) = matches.subcommand_matches("new") {
        let title = matches.value_of("TITLE").unwrap_or_default();
        let editor = default_system_editor();
        let zettel = Zettel::new(&id_timestamp(), title);
        zettel.edit(&editor);
        if path_exists(&zettel.filename()) { // user may not have written the file
            zettel.save(&conn)?;
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("build") {
        create_lua_filter();
        let id = matches.value_of("ID").unwrap_or_default();
        let list_of_zettels = Zettel::from_db_by_id(&conn, id)?;
        for zettel in list_of_zettels {
            if path_exists(&zettel.filename()) {
                zettel.build();
            }
        }
    }

    conn.close().unwrap_or_default();
    Ok(())
}
