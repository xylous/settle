use rusqlite::{Connection, DatabaseName, Error, Result, Row, named_params};

use crate::zettel::Zettel;
use rayon::prelude::*;

/// Construct a Zettel from an entry in the database metadata
fn zettel_metadata(row: &Row) -> Result<Zettel, rusqlite::Error>
{
    let id: String = row.get(0)?;
    let title: String = row.get(1)?;
    let link_str: String = row.get(2)?;
    let links: Vec<String> = crate::str_to_vec(&link_str, ",");
    Ok(Zettel::new(&id, &title, links))
}

pub struct Database
{
    name: String,
    conn: Connection,
}

impl Database
{
    /// Create a `Database` interface to an SQLite database
    pub fn new(name: &str, uri: Option<&str>) -> Result<Self, Error>
    {
        let g_uri = uri.or(Some(name)).unwrap();
        Ok(Database {
            name: name.to_string(),
            conn: Connection::open(g_uri)?,
        })
    }

    /// Create a `Database` interface to a named SQLite database, opened in memory
    pub fn in_memory(name: &str) -> Result<Self, Error>
    {
        let uri = &format!("file:{}?mode=memory&cache=shared", name);
        Database::new(name, Some(uri))
    }

    /// Initialise the current Database with a `zettelkasten` table that holds the properties of
    /// `Zettel`s
    pub fn init(&self) -> Result<(), Error>
    {
        &self.conn.execute(
            "CREATE TABLE IF NOT EXISTS zettelkasten (
                id          TEXT PRIMARY KEY,
                title       TEXT NOT NULL,
                links       TEXT,
                tags        TEXT
            )",
            [])?;
        Ok(())
    }

    /// Save current Database to `path`
    pub fn write_to(&self, path: &str) -> Result<(), Error>
    {
        self.conn.backup(DatabaseName::Main, path, None)?;
        Ok(())
    }

    /// Save the Zettel's metadata to the database
    ///
    /// # Examples
    ///
    /// ```
    /// let conn = rusqlite::Connection::open("zettelkasten.db")?;
    /// let zettel = Zettel::new("-1", "my super interesting note");
    /// zettel.save(&conn)?;
    /// ```
    pub fn save(&self, zettel: &Zettel) -> Result<(), rusqlite::Error>
    {
        let links = crate::vec_to_str(&zettel.links, ",");
        let tags = crate::vec_to_str(&zettel.tags, ",");
        &self.conn.execute(
            "INSERT INTO zettelkasten (id, title, links, tags) values (?1, ?2, ?3, ?4)",
            &[&zettel.id, &zettel.title, &links, &tags])?;
        Ok(())
    }

    /// Remove the Zettel's metadata from the database
    pub fn delete(&self, zettel: &Zettel) -> Result<(), rusqlite::Error>
    {
        &self.conn.execute(
            "DELETE FROM zettelkasten WHERE id = (?1)",
            &[&zettel.id]
        )?;
        Ok(())
    }

    /// Search in the database for the Zettels whose `id` matches `pattern`, and return them
    /// Return an Error if nothing was found
    ///
    /// `pattern` uses SQL pattern syntax, e.g. `%` to match zero or more characters.
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
    pub fn find_by_id(&self, pattern: &str) -> Result<Vec<Zettel>, rusqlite::Error>
    {
        let mut stmt = self.conn.prepare("SELECT * FROM zettelkasten WHERE id LIKE :pattern")?;
        let mut rows = stmt.query(named_params! {":pattern": pattern})?;

        let mut results: Vec<Zettel> = Vec::new();
        while let Some(row) = rows.next()? {
            let zettel = zettel_metadata(row)?;
            results.push(zettel);
        }

        Ok(results)
    }

    /// Search in the database for the Zettels whose `tags` property includes `tag`, and return
    /// them
    /// Return an Error if nothing was found
    ///
    /// `tag` uses SQL pattern syntax, e.g. `%` to match zero or more characters.
    #[allow(dead_code)]
    pub fn find_by_tag(&self, tag: &str) -> Result<Vec<Zettel>, rusqlite::Error>
    {
        let pattern = format!("%{}%", tag);
        let mut stmt = self.conn.prepare("SELECT * FROM zettelkasten WHERE tags LIKE :pattern")?;
        let mut rows = stmt.query(named_params! {":pattern": pattern})?;

        let mut results: Vec<Zettel> = Vec::new();
        while let Some(row) = rows.next()? {
            let zettel = zettel_metadata(row)?;
            results.push(zettel);
        }

        Ok(results)
    }

    /// Search in the database for the Zettels whose `links` property contains `id`, and return
    /// them
    /// Return an Error if nothing was found
    ///
    /// `id` uses SQL pattern syntax, e.g. `%` to match zero or more characters.
    pub fn find_by_links_to(&self, id: &str) -> Result<Vec<Zettel>>
    {
        let pattern = format!("%{}%", id);
        let mut stmt = self.conn.prepare("SELECT * FROM zettelkasten WHERE links LIKE :pattern")?;
        let mut rows = stmt.query(named_params! {":pattern": pattern})?;

        let mut results: Vec<Zettel> = Vec::new();
        while let Some(row) = rows.next()? {
            let zettel = zettel_metadata(row)?;
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
                let mut thread_zettel = Zettel::from_str(&f);
                thread_zettel.update_links();
                thread_db.save(&thread_zettel).unwrap();
            });
    }
}
