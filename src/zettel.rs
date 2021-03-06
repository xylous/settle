use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;
use chrono::prelude::*;

use crate::config::ConfigOptions;
use crate::io::*;

// Find and return wiki-style links inside of `contents` string
// wiki-style links are of the form `[[LINK]]`
fn find_links(contents: &str) -> Vec<String>
{
    let re = Regex::new(r#"\[\[((?s).*?)\]\]"#).unwrap();
    re.captures_iter(contents).par_bridge()
        .map(|cap| {
            let title = cap.get(1)
                .map_or("", |m| m.as_str())
                .to_string()
                .replace("\n", " ")
                .replace("\t", " ");
            title
        })
        .collect()
}

// Find tags inside of `contents` string and return them
// Tags are hashtag-tags, e.g. `#gardening`, `#note-taking`
fn find_tags(contents: &str) -> Vec<String>
{
    let re = Regex::new(r"#([\w/_-]+?)\s+").unwrap();
    re.captures_iter(contents).par_bridge()
        .map(|cap| {
            let tag = cap.get(1).map_or("", |m| m.as_str()).to_string();
            tag
        })
        .collect()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Zettel
{
    pub title: String,
    pub project: String,
    pub links: Vec<String>,
    pub tags: Vec<String>,
}

impl Zettel
{
    /// Create a Zettel with specified `title`
    pub fn new(title: &str, project: &str) -> Self
    {
        Zettel
        {
            title: title.to_string(),
            project: project.to_string(),
            links: vec![],
            tags: vec![],
        }
    }

    /// Create a Zettel from a file, provided a path
    pub fn from_file(cfg: &ConfigOptions, path: &str) -> Self
    {
        let title = basename(&replace_extension(path, ""));
        let contents = file_to_string(path);

        let project = if dirname(path) == cfg.zettelkasten {
            "".to_string()
        } else {
            let segments: Vec<&str> = path.split('/').collect();
            segments[segments.len() - 2].to_string()
        };

        let mut zettel = Zettel::new(&title, &project);
        zettel.links = find_links(&contents);
        zettel.tags = find_tags(&contents);
        zettel
    }

    /// If `cfg.template` is set and a file, then replace placeholders and use it. Otherwise create
    /// a blank file.
    pub fn create(&self, cfg: &ConfigOptions)
    {
        mkdir(&format!("{}/{}", &cfg.zettelkasten, &self.project));
        if file_exists(&cfg.template) {
            let template_contents = file_to_string(&cfg.template);
            let new_zettel_contents = self.replace_template_placeholders(&template_contents);
            write_to_file(&self.filename(cfg), &new_zettel_contents);
        } else {
            write_to_file(&self.filename(cfg), "");
        }
    }

    /// Return a string with the format "`Zettel.title`.md"
    pub fn filename(&self, cfg: &ConfigOptions) -> String
    {
        let dir = format!("{}/{}", cfg.zettelkasten, &self.project);
        format!("{}/{}.md", dir, &self.title)
    }

    /// Check if the current Zettel file contains `text`
    pub fn has_text(&self, cfg: &ConfigOptions, text: &str) -> bool
    {
        let contents = file_to_string(&self.filename(cfg));
        let re = Regex::new(&format!(r"(?i){}", text)).unwrap();

        re.is_match(&contents)
    }

    /// Given the contents of a template file, replace all placeholders with their proper value
    fn replace_template_placeholders(&self, contents: &str) -> String
    {
        let re_title = Regex::new(r"\$\{TITLE\}").unwrap();
        let c1 = re_title.replace_all(contents, &self.title).to_string();
        let re_date = Regex::new(r"\$\{DATE\}").unwrap();
        re_date.replace_all(&c1, Utc::today().format("%Y-%m-%d").to_string()).to_string()
    }
}
