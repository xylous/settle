use std::process::Command;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

use crate::io::*;
use crate::{FILENAME_SEPARATOR, LUA_FILTER_SCRIPT};
use crate::parser::{self, *};
use crate::default_system_editor;

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
        let _ = delete_file(file);
        assert_eq!(z.links, vec!["1012"]);
    }
}

/// Return a String containing
///
/// ```md
///
/// ## Backlinks
///
/// ```
fn backlink_header() -> String
{
    "\n\
    ## Backlinks\n\
    \n"
    .to_string()
}

/// Given a markdown paragraph, generate a context line to be put in the Backlinks section of a
/// Zettel
fn backlink_paragraph_format(s: &str) -> String
{
    format!(
        "\t* {}\n",
        s,
    )
}

/// Return a String containing a markdown reference link with the template:
///
/// ```md
/// [<link.title>]
///
/// [<link.title>]: <link.filename()>
/// ```
fn backlink_str(link: &Zettel, contexts: Vec<String>) -> String
{
    format!(
        "* [{}]\n\
        {}\n\
        [{}]: {}\n\
        ",
        link.title,
        contexts.join(""),
        link.title,
        link.filename(),
    )
}

pub struct Zettel
{
    pub id: String,
    pub title: String,
    pub links: Vec<String>,
}

impl Zettel
{
    /// Create a Zettel with specified `id` and `title`.
    pub fn new(id: &str, title: &str, links: Vec<String>) -> Self
    {
        Zettel
        {
            id: id.to_string(),
            title: title.to_string(),
            links
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
        Zettel::new(id, &title, vec![])
    }

    /// Create Zettel as a physical file on the system and open system editor on it
    pub fn create(self) -> Self
    {
        let editor = default_system_editor();
        write_to_file(&self.filename(), &self.yaml_metadata());
        self.edit(&editor);
        self
    }

    /// Generate YAML metadata to put at the top of a newly created Zettel
    fn yaml_metadata(&self) -> String
    {
        format!(
            "---\n\
            title: {}\n\
            tags:\n\
            ---\n",
            self.title,
        )
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
    /// `Zettel.links`
    pub fn update_links(&mut self)
    {
        let filename = &self.filename();
        let contents = file_to_string(filename);
        let re = Regex::new(&format!(r#"\[.*\]\(\.?/?(.*?){}.*?\.md\)"#, FILENAME_SEPARATOR)).unwrap();
        for cap in re.captures_iter(&contents) {
            let id = cap.get(1).map_or("", |m| m.as_str()).to_string();
            self.links.push(id);
        }
    }

    /// Overwrite (or, if it doesn't exist, create) a `## Backlinks` section at the end of the
    /// Zettel, with the links being other Zettels
    pub fn update_backlinks_section(&self, links: &Vec<Zettel>)
    {
        let file = &self.filename();
        let contents = file_to_string(file);
        let re = Regex::new(r#"\n## Backlinks(?s:.*)\z"#).unwrap();

        let mut new_contents = re.replace(&contents, "").to_string();
        let b_header = backlink_header();
        new_contents.push_str(&b_header);

        for link in links {
            let b_contexts = Self::backlink_context(&self, link);
            let b_link = backlink_str(link, b_contexts);
            new_contents.push_str(&b_link);
        }

        write_to_file(file, &new_contents)
    }

    /// Read `link.filename()` and find the paragraphs where the current Zettel is mentioned
    fn backlink_context(&self, link: &Zettel) -> Vec<String>
    {
        let file = &link.filename();
        let contents = file_to_string(file);
        let re = Regex::new(&format!(r#"\(.*?{}?\)"#, &self.filename())).unwrap();
        parser::parse(&contents)
            .into_par_iter()
            .map(|e| {
                match e {
                    Block::Paragraph(s) => backlink_paragraph_format(&s),
                }
            })
            .filter(|e| {
                re.is_match(e)
            })
            .map(|s| {
                re.replace_all(&s, "").to_string()
            })
            .collect()
    }
}
