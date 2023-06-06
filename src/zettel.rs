use chrono::prelude::*;
use rayon::iter::{ParallelBridge, ParallelIterator};
use regex::Regex;

use crate::config::ConfigOptions;
use crate::io::*;

/// Find and return wiki-style links inside of `contents` string
/// wiki-style links are of the form `[[LINK]]`
fn find_links(contents: &str) -> Vec<String>
{
    let re = Regex::new(r#"\[\[((?s).*?)\]\]"#).unwrap();
    re.captures_iter(contents)
      .par_bridge()
      .map(|cap| {
          cap.get(1)
             .map_or("".to_string(), |m| strip_multiple_whitespace(m.as_str()))
      })
      .collect()
}

/// Find tags inside of `contents` string and return them
///
/// Tags are hashtag-tags, e.g. `#gardening`, `#note-taking`, but they MUST be delimited by any
/// kind of whitespace
fn find_tags(contents: &str) -> Vec<String>
{
    let re = Regex::new(r"\s#([\w/_-]+?)\s").unwrap();
    re.captures_iter(contents)
      .par_bridge()
      .map(|cap| {
          let tag = cap.get(1).map_or("", |m| m.as_str()).to_string();
          tag
      })
      .collect()
}

/// Replace all multiple consecutive whitespace with a single space character.
pub fn strip_multiple_whitespace(s: &str) -> String
{
    let re = Regex::new(r#"[\n\t ]+"#).unwrap();
    re.replace_all(s, " ").to_string()
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
        Zettel { title: title.to_string(),
                 project: project.to_string(),
                 links: vec![],
                 tags: vec![] }
    }

    /// Create a Zettel from a file, provided the ABSOLUTE path to the Zettel
    pub fn from_file(cfg: &ConfigOptions, path: &str) -> Self
    {
        let mut title = basename(&replace_extension(path, ""));
        let contents = file_to_string(path);

        let fixed_ws = strip_multiple_whitespace(&title);
        if fixed_ws != title {
            eprintln!("warning: multiple consecutive whitespaces are not allowed; '{}' was renamed",
                      title);
            let newpath = format!("{}/{}.md", dirname(path), fixed_ws);
            rename(path, &newpath);
            title = fixed_ws;
        }

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

    /// Return the absolute path to the Zettel
    pub fn filename(&self, cfg: &ConfigOptions) -> String
    {
        let dir = format!("{}/{}",
                          cfg.zettelkasten,
                          // if the project is empty then it's the main Zettelkasten project, and
                          // so we don't want to introduce two forward slashes one after the other
                          if self.project.is_empty() {
                              self.project.clone()
                          } else {
                              format!("{}/", &self.project)
                          });
        format!("{}{}.md", dir, &self.title)
    }

    /// Return an empty string if the given Zettel doesn't contain the given pattern, otherwise
    /// return the first match
    pub fn find_pattern(&self, cfg: &ConfigOptions, pattern: &str) -> String
    {
        let contents = file_to_string(&self.filename(cfg));
        let re = Regex::new(&format!(r"(?i){}", pattern)).unwrap();

        if let Some(first) = re.captures(&contents) {
            if let Some(second) = first.get(0) {
                second.as_str()
            } else {
                ""
            }
        } else {
            ""
        }.to_string()
    }

    /// Given the contents of a template file, replace all placeholders with their proper value
    fn replace_template_placeholders(&self, contents: &str) -> String
    {
        let re_title = Regex::new(r"\$\{TITLE\}").unwrap();
        let c1 = re_title.replace_all(contents, &self.title).to_string();
        let re_date = Regex::new(r"\$\{DATE\}").unwrap();
        re_date.replace_all(&c1, Utc::now().format("%Y-%m-%d").to_string())
               .to_string()
    }
}
