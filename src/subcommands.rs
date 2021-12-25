use rusqlite::Error;
use clap::ArgMatches;
use rayon::prelude::*;

use crate::Database;
use crate::Zettel;
use crate::config::ConfigOptions;

use crate::io::file_exists;

fn print_zettel_info(zettel: Vec<&Zettel>)
{
    zettel.par_iter()
        .for_each(|&z| {
            let mut location = String::from("p");
            if z.inbox {
                location = String::from("i");
            }
            println!("[{}] {}", location, z.title);
        })
}

pub fn new(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file(), None)?;
    db.init()?;

    let title = matches.value_of("TITLE").unwrap();
    let is_inbox = matches.is_present("inbox");

    let zettel = Zettel::new(title, is_inbox);

    if file_exists(&zettel.filename(cfg)) {
        eprintln!("couldn't create new Zettel: one with the same title already exists");
        return Ok(());
    } else {
        zettel.create(cfg);
        print_zettel_info(vec![&zettel]);
    }

    // User may not have actually written to the file
    if file_exists(&zettel.filename(cfg)) {
        let updated_metadata = Zettel::from_file(&zettel.filename(cfg));
        db.save(&updated_metadata)?;
    }

    Ok(())
}

pub fn update(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file(), None)?;

    let path = matches.value_of("FILENAME").unwrap();
    if file_exists(path) {
        let zettel = Zettel::from_file(path);
        db.update(cfg, &zettel)?;
    }

    Ok(())
}

pub fn query(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file(), None)?;

    let pattern = matches.value_of("PATTERN").unwrap_or_default();
    let result = db.find_by_title(pattern)?;
    print_zettel_info(result.iter().collect());

    Ok(())
}

pub fn find(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file(), None)?;

    let input = matches.value_of("TAG").unwrap_or_default();

    let mut tags = db.find_by_tag(input)?;
    let mut subtags = db.find_by_tag(&format!("{}/*", input))?;
    tags.append(&mut subtags);
    tags.par_sort();
    tags.dedup();

    tags.par_iter()
        .for_each(|z| {
            println!("{}", z.title);
        });

    Ok(())
}

pub fn list_tags(cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file(), None)?;

    let tags = db.list_tags()?;
    tags.into_par_iter()
        .for_each(|t|
            if !t.is_empty() {
                println!("{}", t)
            }
        );

    Ok(())
}

pub fn backlinks(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let title = matches.value_of("TITLE").unwrap_or_default();

    let db = Database::new(&cfg.db_file(), None)?;
    let links = db.find_by_links_to(title)?;
    print_zettel_info(links.iter().collect());

    Ok(())
}

pub fn search(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let text = matches.value_of("TEXT").unwrap();

    let db = Database::new(&cfg.db_file(), None)?;
    let results = db.search_text(cfg, text)?;
    print_zettel_info(results.iter().collect());

    Ok(())
}

pub fn generate(cfg: &ConfigOptions) -> Result<(), Error>
{
    let start = chrono::Local::now();

    let mem_db = Database::in_memory(&cfg.db_file())?;
    mem_db.init()?;
    mem_db.generate(cfg);
    mem_db.write_to(&cfg.db_file())?;

    let end = chrono::Local::now();
    let time = end - start;
    println!("database generated successfully, took {}ms", time.num_milliseconds());

    Ok(())
}

pub fn not_created(cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file(), None)?;

    let results = db.zettel_not_yet_created()?;
    for title in results {
        println!("{}", title);
    }

    Ok(())
}

pub fn ls(cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file(), None)?;
    let results = db.all()?;

    print_zettel_info(results.iter().collect());

    Ok(())
}

pub fn zettelkasten_dir(cfg: &ConfigOptions) -> Result<(), Error>
{
    println!("{}", cfg.zettelkasten);
    Ok(())
}
