use rusqlite::{Connection, DatabaseName, Error, Result, named_params};

use crate::zettel::Zettel;

pub struct Database
{
    conn: Connection,
}

impl Database
{
    /// Create a `Database` interface to an SQLite database
    pub fn new(name: &str) -> Result<Self, Error>
    {
        Ok(Database {
            conn: Connection::open(name)?,
        })
    }

    /// Create a `Database` interface to a named SQLite database, opened in memory
    pub fn in_memory(name: &str) -> Result<Self, Error>
    {
        let uri = &format!("file:{}?mode=memory&cache=shared", name);
        Database::new(uri)
    }

    /// Initialise the current Database with a `zettelkasten` table that holds the properties of
    /// `Zettel`s
    pub fn init(&self) -> Result<(), Error>
    {
        &self.conn.execute(
            "CREATE TABLE IF NOT EXISTS zettelkasten (
                id          TEXT PRIMARY KEY,
                title       TEXT NOT NULL,
                links       TEXT
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
    pub fn save(&self, zettel: Zettel) -> Result<(), rusqlite::Error>
    {
        let links = crate::vec_to_str(&zettel.links, ",");
        &self.conn.execute(
            "INSERT INTO zettelkasten (id, title, links) values (?1, ?2, ?3)",
            &[&zettel.id, &zettel.title, &links])?;
        Ok(())
    }

    /// Search in the database for the Zettels whose `id` matches `pattern`, and return them
    /// Return an Error if nothing was found
    ///
    /// `id_pattern` uses SQL pattern syntax, e.g. `%` to match zero or more characters.
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
            let id: String = row.get(0)?;
            let title: String = row.get(1)?;
            let link_str: String = row.get(2)?;
            let links: Vec<String> = crate::str_to_vec(&link_str, ",");
            let zettel = Zettel::new(&id, &title, links);
            results.push(zettel);
        }

        Ok(results)
    }
}
