use clap::{App, Arg};
use std::process::Command;
use std::env;
use rusqlite::{Connection, Result, named_params};
use chrono;

mod io;
use io::{path_exists, replace_extension};

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

    /// Search in the database, connected through `conn`, for the Zettels whose `id` matches
    /// `id_pattern`, and return them in a vector
    /// Return an Error if nothing was found
    ///
    /// `id_pattern` uses SQL pattern syntax, e.g. `%` to match one or more characters.
    ///
    /// # Examples
    ///
    /// ```
    /// let conn = rusqlite::Connection::open("test.db")?;
    /// initialize_db(&conn)?;
    /// let zet_1 = &Zettel::new("my_id", "some title");
    /// zet_1.save(&conn)?;
    /// let zet_2 = &Zettel::from_db_by_id(&conn, "my_id")?[0];
    /// assert_eq!(zet_1, zet_2);
    /// ```
    fn from_db_by_id(conn: &Connection, id_pattern: &str) -> Result<Vec<Self>, rusqlite::Error>
    {
        let mut stmt = conn.prepare("SELECT * FROM zettelkasten WHERE id LIKE :pattern")?;
        let mut rows = stmt.query(named_params! {":pattern": id_pattern})?;

        let mut list_of_zettels: Vec<Self> = Vec::new();
        while let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            let title: String = row.get(1)?;
            let zettel = Zettel::new(&id, &title);
            list_of_zettels.push(zettel);
        }

        Ok(list_of_zettels)
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

    /// Compile Zettel, from Markdown to HTML
    /// Requires Pandoc installed
    ///
    /// # Examples
    ///
    /// ```
    /// let zettel = Zettel::new("1a3b", "why do we take notes?");
    /// zettel.edit("nvim"); // add some content to file first
    /// zettel.build();
    /// ```
    fn build(&self) -> ()
    {
        let filename = self.filename();
        println!("compiling {}...", &filename);
        Command::new("pandoc")
            .arg("--standalone")
            .arg(&filename)
            .arg("--output")
            .arg(replace_extension(&filename, "html"))
            .arg(format!("--metadata=title:{}", &self.title))
            .status()
            .expect("failed to execute process");
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
