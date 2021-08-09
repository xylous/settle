use clap::{App, Arg};
use rayon::prelude::*;

mod io;
mod zettel;
mod database;
mod parser;

use crate::io::*;
use crate::zettel::Zettel;
use crate::database::Database;

const FILENAME_SEPARATOR: &str = "::";
const ZETTELKASTEN_DB: &str = "metadata.db";
const LUA_FILTER_SCRIPT: &str = "md_links_to_html.lua";

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

/// Join a vector of `String`s, separated by `sep`
fn vec_to_str(vec: &Vec<String>, sep: &str) -> String
{
    vec.join(sep)
}

/// Split `str` on `sep` and return results as a vector
fn str_to_vec(str: &str, sep: &str) -> Vec<String>
{
    str.split(sep).map(|s| s.to_string()).collect()
}

/// Creates a Lua script that will be used by pandoc to replace links ending in `.md` with links
/// ending in `.html`
fn create_lua_filter()
{
    let lua_script =
r#"-- this script replaces all links ending in `.md` with ones ending in `.html`
-- it will used by pandoc when building the Zettelkasten
function Link(el)
    el.target = string.gsub(el.target, "%.md", ".html")
    return el
end
"#;
    write_to_file(LUA_FILTER_SCRIPT, lua_script);
}

/// Return the value of $EDITOR or $VISUAL, or, if those are empty, return `"vim"`
fn default_system_editor() -> String
{
    std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| "vim".to_string())
}

fn main() -> Result<(), rusqlite::Error>
{
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("xylous, xylous.e@gmail.com")
        .about("CLI tool to manage a digital zettelkasten")
        .subcommand(App::new("new")
            .about("creates a new zettel")
            .arg(Arg::new("TITLE")
                .required(true)
                .about("title of zettel")))
        .subcommand(App::new("edit")
            .about("edit an existing zettel")
            .arg(Arg::new("ID")
                .required(true)
                .about("id of zettel")))
        .subcommand(App::new("build")
            .long_about(
                "compile a Zettel to html \n\
                uses SQL syntax, e.g. `%` to match one or more characters")
            .arg(Arg::new("ID")
                .required(true)
                .about("id of zettel")))
        .subcommand(App::new("find")
            .arg(Arg::new("TAG")
                .required(true)
                .about("tag of zettel"))
            .about("search Zettels by tag"))
        .subcommand(App::new("generate")
            .about("generate the database in the current directory"))
        .subcommand(App::new("backlinks")
            .about("update the backlinks of every file in zettelkasten"))
        .get_matches();

    let db = Database::new(ZETTELKASTEN_DB, None)?;
    db.init()?;

    if let Some(matches) = matches.subcommand_matches("new") {
        let title = matches.value_of("TITLE").unwrap_or_default();
        let mut zettel = Zettel::new(&id_timestamp(), title, vec![]).create();
        zettel.update_links();
        zettel.update_tags();
        db.save(&zettel)?;
    } else if let Some(matches) = matches.subcommand_matches("edit") {
        let id = matches.value_of("ID").unwrap_or_default();
        let results = db.find_by_id(id)?;
        let editor = default_system_editor();
        for mut zettel in results {
            zettel.edit(&editor);
            zettel.update_links();
            zettel.update_tags();
            db.delete(&zettel)?;
            db.save(&zettel)?;
        }
    } else if let Some(matches) = matches.subcommand_matches("build") {
        create_lua_filter();
        let id = matches.value_of("ID").unwrap_or_default();
        let start = chrono::Local::now();

        let results = db.find_by_id(id)?;
        results.par_iter()
            .for_each(|z| {
                z.build();
            });
        delete_file(LUA_FILTER_SCRIPT).unwrap_or_default();

        let end = chrono::Local::now();
        let time = end - start;
        println!("compiled {} files, took {}ms", results.len(), time.num_milliseconds());
    } else if let Some(matches) = matches.subcommand_matches("find") {
        let tag = matches.value_of("TAG").unwrap_or_default();
        let results = db.find_by_tag(tag)?;
        println!("found {} items", results.len());
        results.par_iter()
            .for_each(|z| {
                println!("{}", z.filename());
            });
    } else if matches.subcommand_matches("backlinks").is_some() {
        let all_zettels = db.find_by_id("%")?;
        let start = chrono::Local::now();

        all_zettels.par_iter()
            .for_each(|z| {
                let thread_db = Database::new(ZETTELKASTEN_DB, None).unwrap();
                let links = thread_db.find_by_links_to(&z.id).unwrap();
                z.update_backlinks_section(&links);
            });

        let end = chrono::Local::now();
        let time = end - start;

        println!("updated {} files' backlinks, took {}ms", all_zettels.len(), time.num_milliseconds());
    } else if matches.subcommand_matches("generate").is_some() {
        let start = chrono::Local::now();

        let mem_db = Database::in_memory(ZETTELKASTEN_DB)?;
        mem_db.init()?;
        mem_db.generate();
        mem_db.write_to(ZETTELKASTEN_DB)?;

        let end = chrono::Local::now();
        let time = end - start;
        println!("database generated successfully, took {}ms", time.num_milliseconds());
    }

    Ok(())
}
