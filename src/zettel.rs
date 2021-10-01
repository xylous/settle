use std::process::Command;
use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;

use crate::io::*;
use crate::default_system_editor;

// Find and return wiki-style links inside of `contents` string
// wiki-style links are of the form `[[LINK]]`
fn find_links(contents: &str) -> Vec<String>
{
    let re = Regex::new(&format!(r#"\[\[(.*?)\]\]"#)).unwrap();
    re.captures_iter(&contents).par_bridge()
        .map(|cap| {
            let title = cap.get(1).map_or("", |m| m.as_str()).to_string();
            title
        })
        .collect()
}

// Find tags inside of `contents` string and return them
// Tags are hashtag-tags, e.g. `#gardening`, `#note-taking`
fn find_tags(contents: &str) -> Vec<String>
{
    let re = Regex::new(r"#([\w/_-]+?)\s+").unwrap();
    re.captures_iter(&contents).par_bridge()
        .map(|cap| {
            let tag = cap.get(1).map_or("", |m| m.as_str()).to_string();
            tag
        })
        .collect()
}

pub struct Zettel
{
    pub title: String,
    pub links: Vec<String>,
    pub tags: Vec<String>,
}

impl Zettel
{
    /// Create a Zettel with specified `title`
    pub fn new(title: &str) -> Self
    {
        Zettel
        {
            title: title.to_string(),
            links: vec![],
            tags: vec![],
        }
    }

    /// Create a Zettel from a file, provided a path
    pub fn from_file(path: &str) -> Self
    {
        let title = replace_extension(path, "");
        let mut zettel = Zettel::new(&title);
        let contents = file_to_string(&zettel.filename());

        zettel.links = find_links(&contents);
        zettel.tags = find_tags(&contents);
        zettel
    }

    /// Create Zettel as a physical file on the system and open system editor on it
    pub fn create(self) -> Self
    {
        let editor = default_system_editor();
        write_to_file(&self.filename(), &self.title_header());
        self.edit(&editor);
        self
    }

    /// Generate title header line for physical file
    fn title_header(&self) -> String
    {
        format!(
            "# {}\n",
            &self.title,
        )
    }

    /// Return a string with the format "`Zettel.title`.md"
    ///
    /// # Examples
    ///
    /// ```
    /// let zettel = Zettel::new("Structs in rust");
    /// assert_eq!(zettel.filename(), "Structs in rust.md");
    /// ```
    pub fn filename(&self) -> String
    {
        format!("{}.md", self.title)
    }

    /// Open `editor` on current Zettel
    pub fn edit(&self, editor: &str)
    {
        Command::new(editor)
            .arg(self.filename())
            .status()
            .expect("failed to execute process");
    }
}
