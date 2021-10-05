use rusqlite::Error;
use clap::ArgMatches;
use rayon::prelude::*;

use crate::Database;
use crate::Zettel;
use crate::config::ConfigOptions;
use crate::default_system_editor;

pub fn new(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file(), None)?;
    db.init()?;

    let title = matches.value_of("TITLE").unwrap();
    let is_inbox = matches.is_present("inbox");

    let mut zettel = Zettel::new(title, is_inbox).create(cfg);
    zettel = Zettel::from_file(&zettel.filename(cfg));
    db.save(&zettel)?;

    Ok(())
}

pub fn edit(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file(), None)?;

    let title = matches.value_of("TITLE").unwrap_or_default();
    let editor = default_system_editor();
    for mut zettel in db.find_by_title(&title)? {
        zettel.edit(&editor, &cfg);
        zettel = Zettel::from_file(&zettel.filename(cfg));
        db.delete(&zettel)?;
        db.save(&zettel)?;
    }

    Ok(())
}

pub fn find(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file(), None)?;

    let tag = matches.value_of("TAG").unwrap_or_default();
    let results = db.find_by_tag(tag)?;
    println!("found {} item(s)", results.len());
    results.par_iter()
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
    links.par_iter()
        .for_each(|l| {
            println!("{}", l.title);
        });

    Ok(())
}

pub fn generate(cfg: &ConfigOptions) -> Result<(), Error>
{
    let start = chrono::Local::now();

    let mem_db = Database::in_memory(&cfg.db_file())?;
    mem_db.init()?;
    mem_db.generate(&cfg);
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

    for zettel in results {
        if zettel.inbox {
            print!("inbox: ");
        } else {
            print!("permanent: ");
        }
        println!("{}", zettel.title);
    }

    Ok(())
}
