use clap::{App, Arg};
use std::process::Command;
use std::env;
use chrono;

const FILENAME_SEPARATOR: &str = "::";

#[derive(Debug, PartialEq)]
struct Zettelkasten
{
    zettels: Option<Vec<Zettel>>,
}

#[derive(Debug, PartialEq)]
struct Zettel
{
    filename: String,
    id: String,
    title: String,
}

impl Zettel
{
    /// Create a Zettel with specified `id` and `title`.
    fn new(id: &str, title: &str) -> Self
    {
        Zettel {
            filename: format!("{}{}{}", id, FILENAME_SEPARATOR, title.replace(" ", "_")),
            id: id.to_string(),
            title:  title.to_string(),
        }
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
    fn edit(&self, editor: &str) -> ()
    {
        Command::new(editor)
            .arg(&self.filename)
            .status()
            .expect("failed to execute process");
    }
}

/// Return a timestamp with the format YYYYMMDDhhmmss
/// where YYYY = year,
///         MM = month,
///         DD = day,
///         hh = hour,
///         mm = minute,
///         ss = second
///
/// # Examples
///
/// ```
/// let id = id_timestamp();
/// ```
fn id_timestamp() -> String
{
    let dt = chrono::offset::Local::now();
    dt.format("%Y%m%d%H%M%S").to_string()
}

/// Return the value of $EDITOR or $VISUAL, or, if those are empty, return `"vim"`
fn default_system_editor() -> String
{
    env::var("EDITOR")
        .or(env::var("VISUAL"))
        .unwrap_or("vim".to_string())
}

fn main() -> ()
{
    let matches = App::new("settler")
        .version(env!("CARGO_PKG_VERSION"))
        .about("CLI tool to manage a digital zettelkasten")
        .subcommand(App::new("new")
            .about("creates a new zettel")
            .arg(Arg::new("TITLE")
                .required(true)
                .about("title of zettel")))
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("new") {
        let title = matches.value_of("TITLE").unwrap_or_default();
        let editor = default_system_editor();
        let zettel = Zettel::new(&id_timestamp(), title);
        zettel.edit(&editor);
    }
}
