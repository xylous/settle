use crate::{config::ConfigOptions, zettel::Zettel};
use rayon::prelude::*;
use rusqlite::{
    named_params, Connection, DatabaseName, Error, Result, Row, Transaction, TransactionBehavior,
};
use std::sync::{mpsc, Arc, Mutex, MutexGuard};
use std::thread;

impl Zettel
{
    /// Construct a Zettel from an entry in the database metadata
    /// Return an Error if the `row` was invalid
    fn from_db(conn_lock: &MutexGuard<Connection>, row: &Row) -> Result<Zettel, rusqlite::Error>
    {
        let title: String = row.get(0)?;
        let project: String = row.get(1)?;
        let mut z = Zettel::new(&title, &project);

        let mut stmt = conn_lock.prepare("SELECT link_id FROM links WHERE zettel_id = ?1")?;
        let mut links = stmt.query([&z.title])?;
        while let Some(link_row) = links.next()? {
            z.links.push(link_row.get(0)?);
        }
        let mut stmt = conn_lock.prepare("SELECT tag FROM tags WHERE zettel_id = ?1")?;
        let mut tags = stmt.query([&z.title])?;
        while let Some(tag_row) = tags.next()? {
            z.tags.push(tag_row.get(0)?);
        }
        Ok(z)
    }
}

pub struct Database
{
    conn: Arc<Mutex<Connection>>,
}

impl Database
{
    /// Create a `Database` interface to an SQLite database
    /// Return an Error if the connection couldn't be made
    pub fn new(uri: &str) -> Result<Self, Error>
    {
        Ok(Database { conn: Arc::new(Mutex::new(Connection::open(uri)?)) })
    }

    /// Create a `Database` interface to a named SQLite database, opened in memory
    /// Return an Error if the connection couldn't be made
    pub fn new_in_memory(filename: &str) -> Result<Self, Error>
    {
        let uri = &format!("file:{}?mode=memory&cache=shared", filename);
        Database::new(uri)
    }

    /// Initialise the current Database with a `zettelkasten` table that holds the properties of
    /// `Zettel`s, if it doesn't exist already
    /// Return an Error if this wasn't possible
    pub fn init(&self) -> Result<(), Error>
    {
        let conn_lock = self.conn.lock().unwrap();
        conn_lock.execute("CREATE TABLE IF NOT EXISTS zettelkasten (
                                                title       TEXT NOT NULL,
                                                project     TEXT,
                                                UNIQUE(title)
                                            )",
                          [])?;
        conn_lock.execute("CREATE TABLE IF NOT EXISTS links (
                                                zettel_id   TEXT,
                                                link_id     TEXT,
                                                FOREIGN KEY (zettel_id) REFERENCES zettelkasten (title)
                                            )",
                          [])?;
        conn_lock.execute("CREATE TABLE IF NOT EXISTS tags (
                                                zettel_id   TEXT,
                                                tag         TEXT,
                                                FOREIGN KEY (zettel_id) REFERENCES zettelkasten (title)
                                            )",
                          [])?;
        Ok(())
    }

    /// Save current Database to `path`
    /// Return an Error if this wasn't possible
    pub fn write_to(&self, path: &str) -> Result<(), Error>
    {
        self.conn
            .lock()
            .unwrap()
            .backup(DatabaseName::Main, path, None)?;
        Ok(())
    }

    /// Save a Zettel's metadata to the database
    pub fn save(&self, zettel: &Zettel) -> Result<(), Error>
    {
        let conn_lock = self.conn.lock().unwrap();
        let tsx = Transaction::new_unchecked(&conn_lock, TransactionBehavior::Immediate).unwrap();
        Self::save_tsx(&tsx, zettel)?;
        tsx.commit()?;
        Ok(())
    }

    /// Save a Zettel's metadata in the given transaction
    pub fn save_tsx(tsx: &Transaction, zettel: &Zettel) -> Result<(), Error>
    {
        tsx.execute("INSERT INTO zettelkasten (title, project) values (?1, ?2)",
                    [&zettel.title, &zettel.project])?;
        for link in &zettel.links {
            tsx.execute("INSERT INTO links (zettel_id, link_id) values (?1, ?2)",
                        [&zettel.title, link])?;
        }
        for tag in &zettel.tags {
            tsx.execute("INSERT INTO tags (zettel_id, tag) values (?1, ?2)",
                        [&zettel.title, tag])?;
        }
        Ok(())
    }

    /// Delete a Zettel's metadata from the database
    pub fn delete(&self, zettel: &Zettel) -> Result<(), Error>
    {
        self.conn
            .lock()
            .unwrap()
            .execute("DELETE FROM zettelkasten WHERE title=?1", [&zettel.title])?;
        Ok(())
    }

    /// Return all Zettel in the database
    /// Return an Error if the data in a row couldn't be accessed or if the database was
    /// unreachable
    pub fn all(&self) -> Result<Vec<Zettel>, Error>
    {
        let conn_lock = self.conn.lock().unwrap();
        let mut stmt = conn_lock.prepare("SELECT * FROM zettelkasten")?;
        let mut rows = stmt.query([])?;

        let mut results: Vec<Zettel> = Vec::new();
        while let Some(row) = rows.next()? {
            let zettel = Zettel::from_db(&conn_lock, row)?;
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
        let conn_lock = self.conn.lock().unwrap();
        let mut stmt = conn_lock.prepare("SELECT * FROM zettelkasten WHERE title LIKE :pattern")?;
        let mut rows = stmt.query(named_params! {":pattern": pattern})?;

        let mut results: Vec<Zettel> = Vec::new();
        while let Some(row) = rows.next()? {
            let zettel = Zettel::from_db(&conn_lock, row)?;
            results.push(zettel);
        }

        Ok(results)
    }

    /// Return a list of all unique tags found in the database
    ///
    /// Return an Error if the database was unreachable
    pub fn list_tags(&self) -> Result<Vec<String>, Error>
    {
        let conn_lock = self.conn.lock().unwrap();
        let mut stmt = conn_lock.prepare("SELECT DISTINCT tag FROM tags")?;
        let mut rows = stmt.query([])?;

        let mut results: Vec<String> = Vec::new();
        while let Some(row) = rows.next()? {
            results.push(row.get(0)?);
        }
        Ok(results)
    }

    /// Return a list of all unique project names found in the database
    ///
    /// Return an Error if the database was unreachable
    pub fn list_projects(&self) -> Result<Vec<String>, Error>
    {
        let conn_lock = self.conn.lock().unwrap();
        let mut stmt = conn_lock.prepare("SELECT DISTINCT project FROM zettelkasten")?;
        let mut rows = stmt.query([])?;

        let mut results: Vec<String> = Vec::new();
        while let Some(row) = rows.next()? {
            let project: String = row.get(0)?;
            if !project.is_empty() {
                results.push(project);
            }
        }
        Ok(results)
    }

    /// Search in the database for Zettel that have been linked to, but don't yet exist
    ///
    /// Return an Error if the database was unreachable or if the data in a Row couldn't have been
    /// accessed
    pub fn zettel_not_yet_created(&self) -> Result<Vec<String>, Error>
    {
        let conn_lock = self.conn.lock().unwrap();
        let mut stmt = conn_lock.prepare("SELECT DISTINCT link_id FROM links WHERE link_id NOT IN (SELECT title FROM zettelkasten)")?;
        let mut rows = stmt.query([])?;

        let mut ghosts: Vec<String> = Vec::new();
        while let Some(row) = rows.next()? {
            ghosts.push(row.get(0)?);
        }

        ghosts.sort();
        ghosts.dedup();

        Ok(ghosts)
    }

    /// Look for Markdown files in the Zettelkasten directory and populate the database with their
    /// metadata
    pub fn generate(&self, cfg: &ConfigOptions) -> Result<(), Error>
    {
        let mut directories = crate::io::list_subdirectories(&cfg.zettelkasten);

        let (tx, rx) = mpsc::sync_channel::<Zettel>(1);
        let conn = self.conn.clone();

        // Add a separate thread to handle transactioning everything at once
        thread::spawn(move || {
            let conn_lock = conn.lock().unwrap();
            let tsx =
                Transaction::new_unchecked(&conn_lock, TransactionBehavior::Immediate).unwrap();
            loop {
                let data = rx.recv();
                match data {
                    Ok(zettel) => match Database::save_tsx(&tsx, &zettel) {
                        Ok(_) => continue,
                        Err(_) => {
                            eprintln!("Warning: couldn't add Zettel '{}' to the '{}' project; there is another note with that title, and titles must be unique",
                                      &zettel.title,
                                      if zettel.project.is_empty() {
                                          "main"
                                      } else {
                                          &zettel.project
                                      },
                                    )
                        }
                    },
                    // If we get a RecvError, then we know we've encountered the end
                    Err(mpsc::RecvError) => {
                        tsx.commit().unwrap();
                        return;
                    }
                }
            }
        });

        directories.push(cfg.zettelkasten.clone());
        directories.par_iter().for_each(|dir| {
                                    let paths: Vec<String> =
                                        // don't add markdown file that starts with a dot (which
                                        // includes the empty title file, the '.md')
                                        crate::io::list_md_files(dir).into_iter()
                                                                    .filter(|f| {
                                                                        !crate::io::basename(f).starts_with('.')
                                                                    })
                                                                    .collect();
                                    paths.par_iter().for_each(|path| {
                                                    let zettel = Zettel::from_file(cfg, path);
                                                    tx.send(zettel).unwrap();
                                    });
        });
        // Send RecvError to the thread
        drop(tx);

        Ok(())
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

    /// Change the project of the given Zettel within the database
    pub fn change_project(&self, zettel: &Zettel, new_project: &str) -> Result<(), Error>
    {
        self.conn
            .lock()
            .unwrap()
            .execute("UPDATE zettelkasten SET project=?1 WHERE title=?2",
                     [new_project, &zettel.title])?;
        Ok(())
    }

    /// Change the title of the given Zettel within the database
    pub fn change_title(&self, zettel: &Zettel, new_title: &str) -> Result<(), Error>
    {
        self.conn
            .lock()
            .unwrap()
            .execute("UPDATE zettelkasten SET title=?1 WHERE title=?2",
                     [new_title, &zettel.title])?;
        Ok(())
    }
}
