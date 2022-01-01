use rusqlite::Error;
use clap::ArgMatches;
use rayon::prelude::*;

use crate::Database;
use crate::Zettel;
use crate::config::ConfigOptions;

use crate::io::file_exists;

/// Print all Zettel in the given vector with the format: `[i] <TITLE>` if in inbox, and `[p]
/// <TITLE>` if in outbox.
fn print_zettel_info(zettel: &Vec<Zettel>)
{
    zettel.par_iter()
        .for_each(|z| {
            let mut location = String::from("p");
            if z.inbox {
                location = String::from("i");
            }
            println!("[{}] {}", location, z.title);
        })
}

/// Based on the CLI arguments and the config options, *maybe* add a new entry to the database
pub fn new(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file(), None)?;
    db.init()?;

    let title = matches.value_of("TITLE").unwrap();
    let is_inbox = matches.is_present("inbox");

    let zettel = Zettel::new(title, is_inbox);

    let exists_in_fs = file_exists(&zettel.filename(cfg));
    let exists_in_db = db.all().unwrap().into_par_iter().any(|z| z.clone() == zettel);

    // If the corresponding file exists and there's an entry in the database, abort.
    // If there's a file but there's no entry in the database, create an entry.
    // Otherwise, create a new file from template and add a database entry.
    if exists_in_fs && exists_in_db {
        eprintln!("couldn't create new Zettel: one with the same title already exists");
        return Ok(());
    } else if exists_in_fs {
        println!("file exists in filesystem but not in database; added entry"); // saved right after
    } else {
        zettel.create(cfg);
        print_zettel_info(&vec![zettel.clone()]); // confirm that the Zettel was created
    }
    db.save(&zettel)?;

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
    print_zettel_info(&result);

    Ok(())
}

pub fn find(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file(), None)?;

    let input = matches.value_of("TAG").unwrap_or_default();

    let mut zettel = db.find_by_tag(input)?;
    let mut zettel_with_subtag = db.find_by_tag(&format!("{}/*", input))?;
    zettel.append(&mut zettel_with_subtag);
    zettel.par_sort();
    zettel.dedup();

    print_zettel_info(&zettel);

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
    print_zettel_info(&links);

    Ok(())
}

pub fn search(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let text = matches.value_of("TEXT").unwrap();

    let db = Database::new(&cfg.db_file(), None)?;
    let results = db.search_text(cfg, text)?;
    print_zettel_info(&results);

    Ok(())
}

pub fn generate(cfg: &ConfigOptions) -> Result<(), Error>
{
    let start = std::time::Instant::now();

    let mem_db = Database::in_memory(&cfg.db_file())?;
    mem_db.init()?;
    mem_db.generate(cfg);
    mem_db.write_to(&cfg.db_file())?;

    println!("database generated successfully, took {}ms", start.elapsed().as_millis());

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

    print_zettel_info(&results);

    Ok(())
}

pub fn zettelkasten_dir(cfg: &ConfigOptions) -> Result<(), Error>
{
    println!("{}", cfg.zettelkasten);
    Ok(())
}
