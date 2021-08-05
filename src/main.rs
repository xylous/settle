use clap::{App, Arg};
use rusqlite::DatabaseName;
use std::env;
use rusqlite::{Connection, Result};
use rayon::prelude::*;

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
        .or_else(|_| env::var("VISUAL"))
        .unwrap_or_else(|_| "vim".to_string())
}

/// Create table `zettelkasten` in database `conn` if it doesn't exist already
///
/// The table `zettelkasten` has two properties: `id` and `title`, both of type `TEXT`
fn initialize_db(conn: &Connection) -> Result<(), rusqlite::Error>
{
    conn.execute(
        "CREATE TABLE IF NOT EXISTS zettelkasten (
            id          TEXT PRIMARY KEY,
            title       TEXT NOT NULL,
            links       TEXT
        )",
        []).expect("failed to create database");
    Ok(())
}

/// Join a vector of `String`s, separated by `sep`
fn vec_to_str(vec: &Vec<String>, sep: &str) -> String
{
    vec.join(sep)
}

/// Split `str` on `sep` and return results as a vector
fn str_to_vec(str: &str, sep: &str) -> Vec<String>
{
    str.split(sep).map(|s| s.to_string()).collect()
}

/// Creates a Lua script that will be used by pandoc to replace links ending in `.md` with links
/// ending in `.html`
fn create_lua_filter()
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
        .subcommand(App::new("generate")
            .about("generate the database in the current directory"))
        .get_matches();

    let conn = Connection::open(ZETTELKASTEN_DB)?;
    initialize_db(&conn)?;

    if let Some(matches) = matches.subcommand_matches("new") {
        let title = matches.value_of("TITLE").unwrap_or_default();
        let editor = default_system_editor();
        let zettel = Zettel::new(&id_timestamp(), title, vec![]);
        zettel.edit(&editor);
        if path_exists(&zettel.filename()) { // user may not have written the file
            zettel.save(&conn)?;
        }
    }

    if let Some(matches) = matches.subcommand_matches("build") {
        create_lua_filter();
        let id = matches.value_of("ID").unwrap_or_default();
        let list_of_zettels = Zettel::from_db_by_id(&conn, id)?;
        for zettel in list_of_zettels {
            if path_exists(&zettel.filename()) {
                zettel.build();
            }
        }
    }

    if matches.subcommand_matches("generate").is_some() {
        let db_param = format!("file:{}?mode=memory&cache=shared", ZETTELKASTEN_DB);
        let start = chrono::Local::now();

        let m_conn = Connection::open(&db_param)?;
        initialize_db(&m_conn)?;
        let files = list_md_files();
        files.par_iter()
            .for_each(|f| {
                let t_conn = Connection::open(&db_param).unwrap();
                let mut t_zet = Zettel::from_str(&f);
                t_zet.update_links();
                t_zet.save(&t_conn).expect("failed to save zettel");
                t_conn.close().unwrap_or_default();
            });
        m_conn.backup(DatabaseName::Main, ZETTELKASTEN_DB, None)?;

        let end = chrono::Local::now();
        let time = end - start;

        println!("database generated successfully, took {}ms", time.num_milliseconds());
    }

    conn.close().unwrap_or_default();
    Ok(())
}
