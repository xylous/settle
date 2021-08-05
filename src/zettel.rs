use rusqlite::{Connection, Result, named_params};
use std::process::Command;
use regex::Regex;

use crate::io::*;
use crate::{FILENAME_SEPARATOR, LUA_FILTER_SCRIPT};

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn from_str_filename_with_no_extension()
    {
        let ans = Zettel::from_str("100b23e::some_title");

        assert_eq!(ans.id, "100b23e");
        assert_eq!(ans.title, "some title");
    }

    #[test]
    fn from_str_filename_with_md_extension()
    {
        let ans = Zettel::from_str("100b23e::some_title.md");

        assert_eq!(ans.id, "100b23e");
        assert_eq!(ans.title, "some title");
    }

    #[test]
    fn zettel_update_links_one_link()
    {
        let file = "1011::test.md";
        write_to_file(file, "[test](1012::interesting_note.md)");
        let mut z = Zettel::from_str(file);
        z.update_links();
        let _ = remove_file(file);
        assert_eq!(z.links_to, vec!["1012"]);
    }
}

pub struct Zettel
{
    pub id: String,
    pub title: String,
    pub links_to: Vec<String>,
}

impl Zettel
{
    /// Create a Zettel with specified `id` and `title`.
    pub fn new(id: &str, title: &str) -> Self
    {
        Zettel
        {
            id: id.to_string(),
            title: title.to_string(),
            links_to: vec![],
        }
    }

    /// Create a Zettel from a string representing a filename
    ///
    /// # Examples
    ///
    /// ```
    /// const FILENAME_SEPARATOR = "::";
    /// let ans = Zettel::from_str("100b23e::some_title");
    ///
    /// assert_eq!(ans.id, "100b23e");
    /// assert_eq!(ans.title, "some title");
    /// ```
    pub fn from_str(s: &str) -> Self
    {
        let extensionless = replace_extension(s, "");
        let split = extensionless.split(FILENAME_SEPARATOR);
        let vec: Vec<&str> = split.collect();
        let id = vec[0];
        let title = vec[1].replace("_", " "); // in the filename, spaces are replaced with underscores
        Zettel::new(id, &title)
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
    pub fn from_db_by_id(conn: &Connection, id_pattern: &str) -> Result<Vec<Self>, rusqlite::Error>
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
    pub fn filename(&self) -> String
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
    pub fn edit(&self, editor: &str)
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
    pub fn save(&self, conn: &Connection) -> Result<(), rusqlite::Error>
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
    pub fn build(&self)
    {
        let filename = self.filename();
        let out_file = replace_extension(&filename, "html");

        println!("compiling {}...", &filename);
        Command::new("pandoc")
            .arg("--standalone")
            .arg(&filename)
            .arg("--output")
            .arg(&out_file)
            .arg(format!("--lua-filter={}", LUA_FILTER_SCRIPT))
            .arg(format!("--metadata=title:{}", &self.title))
            .status()
            .expect("failed to execute process");
    }

    /// Look into the file corresponding to the `Zettel`, extract links from it and put them in
    /// `Zettel.links_to`
    pub fn update_links(&mut self)
    {
        let filename = &self.filename();
        let contents = file_to_string(filename);
        let re = Regex::new(&format!(r#"\[.*\]\(\.?/?(.*?){}.*?\.md\)"#, FILENAME_SEPARATOR)).unwrap();
        for cap in re.captures_iter(&contents) {
            let id = cap.get(1).map_or("", |m| m.as_str()).to_string();
            self.links_to.push(id);
        }
    }
}
