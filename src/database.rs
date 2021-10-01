use rusqlite::{Connection, DatabaseName, Error, Result, Row, named_params};

use crate::{SQL_ARRAY_SEPARATOR, str_to_vec, zettel::Zettel};
use rayon::prelude::*;

impl Zettel {
    /// Construct a Zettel from an entry in the database metadata
    /// Return an Error if the `row` was invalid
    fn from_db(row: &Row) -> Result<Zettel, rusqlite::Error>
    {
        let title: String = row.get(0)?;
        Ok(Zettel::new(&title))
    }
}

pub struct Database
{
    name: String,
    conn: Connection,
}

impl Database
{
    /// Create a `Database` interface to an SQLite database
    /// Return an Error if the connection couldn't be made
    pub fn new(name: &str, uri: Option<&str>) -> Result<Self, Error>
    {
        let g_uri = uri.or(Some(name)).unwrap();
        Ok(Database {
            name: name.to_string(),
            conn: Connection::open(g_uri)?,
        })
    }

    /// Create a `Database` interface to a named SQLite database, opened in memory
    /// Return an Error if the connection couldn't be made
    pub fn in_memory(name: &str) -> Result<Self, Error>
    {
        let uri = &format!("file:{}?mode=memory&cache=shared", name);
        Database::new(name, Some(uri))
    }

    /// Initialise the current Database with a `zettelkasten` table that holds the properties of
    /// `Zettel`s, if it doesn't exist already
    /// Return an Error if this wasn't possible
    pub fn init(&self) -> Result<(), Error>
    {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS zettelkasten (
                title       TEXT PRIMARY KEY NOT NULL,
                links       TEXT,
                tags        TEXT
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
    ///
    /// # Examples
    ///
    /// ```
    /// let db = Database::in_memory("some_db_name");
    /// let zettel = Zettel::new("my super interesting note");
    /// db.save(zettel)?;
    /// ```
    pub fn save(&self, zettel: &Zettel) -> Result<(), Error>
    {
        let links = crate::vec_to_str(&zettel.links);
        let tags = crate::vec_to_str(&zettel.tags);
        self.conn.execute(
            "INSERT INTO zettelkasten (title, links, tags) values (?1, ?2, ?3)",
            &[&zettel.title, &links, &tags])?;
        Ok(())
    }

    /// Remove a Zettel's metadata from the database
    /// Return an Error if this wasn't possible
    pub fn delete(&self, zettel: &Zettel) -> Result<(), Error>
    {
        self.conn.execute(
            "DELETE FROM zettelkasten WHERE title = (?1)",
            &[&zettel.title]
        )?;
        Ok(())
    }

    /// Search in the database for the Zettels whose `title` property matches `title`, and return
    /// them
    /// Return an Error if the databases was unreachable.
    ///
    /// `pattern` uses SQL pattern syntax, e.g. `%` to match zero or more characters.
    pub fn find_by_title(&self, title: &str) -> Result<Vec<Zettel>, Error>
    {
        let mut stmt = self.conn.prepare("SELECT * FROM zettelkasten WHERE title LIKE :title")?;
        let mut rows = stmt.query(named_params! {":title": title})?;

        let mut results: Vec<Zettel> = Vec::new();
        while let Some(row) = rows.next()? {
            let zettel = Zettel::from_db(row)?;
            results.push(zettel);
        }

        Ok(results)
    }

    /// Search in the database for the Zettels whose `tags` property includes `tag`, and return
    /// them
    /// Return an Error if the database was unreachable
    ///
    /// `tag` uses SQL pattern syntax, e.g. `%` to match zero or more characters.
    pub fn find_by_tag(&self, tag: &str) -> Result<Vec<Zettel>, Error>
    {
        let pattern = format!(
            "%{}{}{}%",
            SQL_ARRAY_SEPARATOR,
            tag,
            SQL_ARRAY_SEPARATOR,
        );
        let mut stmt = self.conn.prepare("SELECT * FROM zettelkasten WHERE tags LIKE :pattern")?;
        let mut rows = stmt.query(named_params! {":pattern": pattern})?;

        let mut results: Vec<Zettel> = Vec::new();
        while let Some(row) = rows.next()? {
            let zettel = Zettel::from_db(row)?;
            results.push(zettel);
        }

        Ok(results)
    }

    /// Return a list of all unique tags found in the database
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

    /// Search in the database for the Zettels whose `links` property contains `title`, and
    /// return them
    /// Return an Error if the database was unreachable
    ///
    /// `title` uses SQL pattern syntax, e.g. `%` to match zero or more characters.
    pub fn find_by_links_to(&self, title: &str) -> Result<Vec<Zettel>>
    {
        let pattern = format!(
            "%{}{}{}%",
            SQL_ARRAY_SEPARATOR,
            title,
            SQL_ARRAY_SEPARATOR,
        );
        let mut stmt = self.conn.prepare("SELECT * FROM zettelkasten WHERE links LIKE :pattern")?;
        let mut rows = stmt.query(named_params! {":pattern": pattern})?;

        let mut results: Vec<Zettel> = Vec::new();
        while let Some(row) = rows.next()? {
            let zettel = Zettel::from_db(row)?;
            results.push(zettel);
        }

        Ok(results)
    }

    /// Look for Markdown files in the current directory and populate the database with their
    /// metadata
    pub fn generate(&self)
    {
        let files = crate::io::list_md_files();
        let name = &self.name;
        files.par_iter()
            .for_each(|f| {
                let thread_db = Self::in_memory(name).unwrap();
                let thread_zettel = Zettel::from_file(&f);
                thread_db.save(&thread_zettel).unwrap();
            });
    }
}
