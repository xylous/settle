use regex::{Regex, Captures};
use rusqlite::{Connection, DatabaseName, Error, Result, Row, named_params};

use crate::{SQL_ARRAY_SEPARATOR, config::ConfigOptions, str_to_vec, zettel::Zettel};
use rayon::prelude::*;

impl Zettel {
    /// Construct a Zettel from an entry in the database metadata
    /// Return an Error if the `row` was invalid
    fn from_db(row: &Row) -> Result<Zettel, rusqlite::Error>
    {
        let title: String = row.get(0)?;
        let project: String = row.get(1)?;
        let links: String = row.get(2)?;
        let tags: String = row.get(3)?;
        let mut z = Zettel::new(&title, &project);
        z.links = str_to_vec(&links);
        z.tags = str_to_vec(&tags);
        Ok(z)
    }
}

pub struct Database
{
    name: String,
    conn: Connection,
}

/// Turn command line input pattern into database request string equivalent
///
/// ### Rules
///
/// [characters in input] -> [characters in output]
///
/// - '\\' -> '\'
/// - '%' -> '\%'
/// - '*' -> '%'
/// - '\*' -> '*'
/// - '_' -> '\_'
/// - '.' -> '_'
/// - '\.' -> '.'
/// - '\' -> ''
fn cli_input_to_db_input(inp: &str) -> String
{
    let re = Regex::new(r"(\\\\|%|\*|\\\*|_|\.|\\\.|\\)").unwrap();
    re.replace_all(inp, |cap: &Captures| {
        match &cap[0] {
            r"\\" => r"\",
            r"%" => r"\%",
            r"*" => r"%",
            r"\*" => r"*",
            r"_" => r"\_",
            r"." => r"_",
            r"\." => r".",
            _ => r"",
        }
    }).to_string()
}

impl Database
{
    /// Create a `Database` interface to an SQLite database
    /// Return an Error if the connection couldn't be made
    pub fn new(name: &str) -> Result<Self, Error>
    {
        Ok(Database {
            name: name.to_string(),
            conn: Connection::open(name)?,
        })
    }

    /// Create a `Database` interface to a named SQLite database, opened in memory
    /// Return an Error if the connection couldn't be made
    pub fn in_memory(name: &str) -> Result<Self, Error>
    {
        let uri = &format!("file:{}?mode=memory&cache=shared", name);
        Ok(Database {
            name: name.to_string(),
            conn: Connection::open(uri)?,
        })
    }

    /// Initialise the current Database with a `zettelkasten` table that holds the properties of
    /// `Zettel`s, if it doesn't exist already
    /// Return an Error if this wasn't possible
    pub fn init(&self) -> Result<(), Error>
    {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS zettelkasten (
                title       TEXT NOT NULL,
                project     TEXT,
                links       TEXT,
                tags        TEXT,
                UNIQUE(title, project)
            )",
            [])?;
        Ok(())
    }

    /// Save current Database to `path`
    /// Return an Error if this wasn't possible
    pub fn write_to(&self, path: &str) -> Result<(), Error>
    {
        self.conn.backup(DatabaseName::Main, path, None)?;
        Ok(())
    }

    /// Save a Zettel's metadata to the database
    pub fn save(&self, zettel: &Zettel) -> Result<(), Error>
    {
        let links = crate::vec_to_str(&zettel.links);
        let tags = crate::vec_to_str(&zettel.tags);
        self.conn.execute(
            "INSERT INTO zettelkasten (title, project, links, tags) values (?1, ?2, ?3, ?4)",
            &[ &zettel.title, &zettel.project, &links, &tags ])?;
        Ok(())
    }

    /// Delete a Zettel's metadata from the database
    pub fn delete(&self, zettel: &Zettel) -> Result<(), Error>
    {
        self.conn.execute(
            "DELETE FROM zettelkasten WHERE title=?1 AND project=?2",
            &[&zettel.title, &zettel.project ])?;
        Ok(())
    }

    /// Return all Zettel in the database
    /// Return an Error if the data in a row couldn't be accessed or if the database was
    /// unreachable
    pub fn all(&self) -> Result<Vec<Zettel>, Error>
    {
        let mut stmt = self.conn.prepare("SELECT * FROM zettelkasten")?;
        let mut rows = stmt.query([])?;

        let mut results: Vec<Zettel> = Vec::new();
        while let Some(row) = rows.next()? {
            let zettel = Zettel::from_db(row)?;
            results.push(zettel);
        }

        Ok(results)
    }

    /// Search in the database for the Zettels whose `title` property matches `pattern`, and return
    /// them
    /// Return an Error if the databases was unreachable.
    ///
    /// `pattern` uses SQL pattern syntax, e.g. `%` to match zero or more characters.
    pub fn find_by_title(&self, pattern: &str) -> Result<Vec<Zettel>, Error>
    {
        let req_pattern = cli_input_to_db_input(pattern);
        let mut stmt = self.conn.prepare("SELECT * FROM zettelkasten WHERE title LIKE :req_pattern")?;
        let mut rows = stmt.query(named_params! {":req_pattern": req_pattern})?;

        let mut results: Vec<Zettel> = Vec::new();
        while let Some(row) = rows.next()? {
            let zettel = Zettel::from_db(row)?;
            results.push(zettel);
        }

        Ok(results)
    }

    /// Search in the database for the Zettels whose `tags` property includes `pattern`, and return
    /// them
    /// Return an Error if the database was unreachable
    ///
    /// `tag` uses SQL pattern syntax, e.g. `%` to match zero or more characters.
    pub fn find_by_tag(&self, pattern: &str) -> Result<Vec<Zettel>, Error>
    {
        let req_pattern = format!(
            "%{}{}{}%",
            SQL_ARRAY_SEPARATOR,
            cli_input_to_db_input(pattern),
            SQL_ARRAY_SEPARATOR,
        );
        let mut stmt = self.conn.prepare("SELECT * FROM zettelkasten WHERE tags LIKE :req_pattern")?;
        let mut rows = stmt.query(named_params! {":req_pattern": req_pattern})?;

        let mut results: Vec<Zettel> = Vec::new();
        while let Some(row) = rows.next()? {
            let zettel = Zettel::from_db(row)?;
            results.push(zettel);
        }

        Ok(results)
    }

    /// Return a list of all unique tags found in the database
    ///
    /// Return an Error if the database was unreachable
    pub fn list_tags(&self) -> Result<Vec<String>, Error>
    {
        let mut stmt = self.conn.prepare("SELECT tags FROM zettelkasten")?;
        let mut rows = stmt.query([])?;

        let mut results: Vec<String> = Vec::new();
        while let Some(row) = rows.next()? {
            let tags: String = row.get(0)?;
            for tag in str_to_vec(&tags) {
                results.push(tag);
            }
        }
        results.par_sort();
        results.dedup();
        Ok(results)
    }

    /// Return a list of all unique project names found in the database
    ///
    /// Return an Error if the database was unreachable
    pub fn list_projects(&self) -> Result<Vec<String>, Error>
    {
        let mut stmt = self.conn.prepare("SELECT project FROM zettelkasten")?;
        let mut rows = stmt.query([])?;

        let mut results: Vec<String> = Vec::new();
        while let Some(row) = rows.next()? {
            let project: String = row.get(0)?;
            if ! project.is_empty() {
                results.push(project);
            }
        }
        results.par_sort();
        results.dedup();
        Ok(results)
    }

    /// Search in the database for the Zettel whose `links` property includes `pattern`, and
    /// return them
    /// Return an Error if the database was unreachable
    ///
    /// `title` uses SQL pattern syntax, e.g. `%` to match zero or more characters.
    pub fn find_by_links_to(&self, pattern: &str) -> Result<Vec<Zettel>>
    {
        let req_pattern = format!(
            "%{}{}{}%",
            SQL_ARRAY_SEPARATOR,
            cli_input_to_db_input(pattern),
            SQL_ARRAY_SEPARATOR,
        );
        let mut stmt = self.conn.prepare("SELECT * FROM zettelkasten WHERE links LIKE :req_pattern")?;
        let mut rows = stmt.query(named_params! {":req_pattern": req_pattern})?;

        let mut results: Vec<Zettel> = Vec::new();
        while let Some(row) = rows.next()? {
            let zettel = Zettel::from_db(row)?;
            results.push(zettel);
        }

        Ok(results)
    }

    /// Search in the database for Zettel that have been linked to, but don't yet exist
    /// Return an Error if the database was unreachable or if the data in a Row couldn't have been
    /// accessed
    pub fn zettel_not_yet_created(&self) -> Result<Vec<String>>
    {
        let mut stmt = self.conn.prepare("SELECT links FROM zettelkasten")?;
        let mut rows = stmt.query([])?;

        let mut unique_links: Vec<String> = Vec::new();
        while let Some(row) = rows.next()? {
            let links_str: String = row.get(0)?;
            let links = str_to_vec(&links_str);
            unique_links.extend(links);
        }

        unique_links.par_sort();
        unique_links.dedup();

        Ok(unique_links.into_iter()
            .filter(|link| {
                // if the response was empty, then nothing has been found, meaning it doesn't exist
                // in the database
                self.find_by_title(link).unwrap().is_empty()
            })
            .collect())
    }

    /// Look for Markdown files in the Zettelkasten directory and populate the database with their
    /// metadata
    pub fn generate(&self, cfg: &ConfigOptions)
    {
        let db_name = &self.name;

        let mut directories = crate::io::list_subdirectories(&cfg.zettelkasten);
        directories.push(cfg.zettelkasten.clone());
        directories.par_iter()
            .for_each(|dir| {
                let notes = crate::io::list_md_files(dir);
                notes.par_iter()
                    .for_each(|note| {
                        let thread_db = Self::in_memory(db_name).unwrap();
                        let thread_zettel = Zettel::from_file(cfg, note);
                        thread_db.save(&thread_zettel).unwrap();
                    });
            });
    }

    /// Update the metadata for a given Zettel. The specified path *must* exist
    /// Not practical for a bunch of Zettel. Use `generate` instead.
    pub fn update(&self, cfg: &ConfigOptions, zettel: &Zettel) -> Result<(), Error>
    {
        self.delete(zettel)?;
        let z = &Zettel::from_file(cfg, &zettel.filename(cfg));
        self.save(z)?;
        Ok(())
    }

    /// Return titles of Zettel that contain `text`
    pub fn search_text(&self, cfg: &ConfigOptions, text: &str) -> Result<Vec<Zettel>, Error>
    {
        let zettel = self.all()?;
        Ok(zettel.par_iter()
            .filter(|z|
                z.has_text(cfg, text)
            ).map(|z| z.clone())
            .collect())
    }

    /// Change the project of the given Zettel within the database
    pub fn change_project(&self, zettel: &Zettel, project: &str) -> Result<(), Error>
    {
        self.conn.execute(
            "UPDATE zettelkasten SET project=?1 WHERE title=?2 AND project=?3",
            &[ project, &zettel.title, &zettel.project ])?;
        Ok(())
    }
}
