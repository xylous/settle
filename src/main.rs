use clap::{App, Arg};
use std::process::Command;
use std::env;
use rusqlite::{Connection, Result};
use chrono;

mod io;
use io::path_exists;

const FILENAME_SEPARATOR: &str = "::";
const ZETTELKASTEN_DB: &str = ".zettelkasten.db";

struct Zettel
{
    id: String,
    title: String,
}

impl Zettel
{
    /// Create a Zettel with specified `id` and `title`.
    fn new(id: &str, title: &str) -> Self
    {
        Zettel
        {
            id: id.to_string(),
            title: title.to_string(),
        }
    }

    /// Return a string with the format "`id`(FILENAME_SEPARATOR)`title`.md"
    ///
    /// # Examples
    ///
    /// ```
    /// let FILENAME_SEPARATOR = "::";
    /// let zettel = Zettel::new("2021", "structs in rust");
    ///
    /// assert_eq!(zettel.filename(), "2021::structs_in_rust.md");
    /// ```
    fn filename(&self) -> String
    {
        format!("{}{}{}.md", self.id, FILENAME_SEPARATOR, self.title.replace(" ", "_"))
    }

    /// Open `editor` on current Zettel
    ///
    /// # Examples
    ///
    /// ```
    /// let zettel = Zettel::new("1", "my note");
    /// zettel.edit("nvim"); // opens neovim, or panics if it can't find it
    /// zettel.edit("emacs"); // opens emacs, or panics if it can't find it
    /// ```
    fn edit(&self, editor: &str) -> ()
    {
        Command::new(editor)
            .arg(self.filename())
            .status()
            .expect("failed to execute process");
    }

    /// Save the current Zettel's metadata to the database, through `conn`
    ///
    /// # Examples
    ///
    /// ```
    /// let conn = rusqlite::Connection::open("zettelkasten.db")?;
    /// let zettel = Zettel::new("-1", "my super interesting note");
    /// zettel.save(&conn)?;
    /// ```
    fn save(&self, conn: &Connection) -> Result<(), rusqlite::Error>
    {
        conn.execute(
            "INSERT INTO zettelkasten (id, title) values (?1, ?2)",
            &[&self.id, &self.title])?;
        Ok(())
    }
}

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
        .get_matches();

    let conn = Connection::open(ZETTELKASTEN_DB)?;
    initialize_db(&conn).unwrap();

    if let Some(ref matches) = matches.subcommand_matches("new") {
        let title = matches.value_of("TITLE").unwrap_or_default();
        let editor = default_system_editor();
        let zettel = Zettel::new(&id_timestamp(), title);
        zettel.edit(&editor);
        if path_exists(&zettel.filename()) { // user may not have written the file
            zettel.save(&conn).unwrap();
        }
    }

    conn.close().unwrap_or_default();
    Ok(())
}
