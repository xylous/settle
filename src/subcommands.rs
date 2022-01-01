use rusqlite::Error;
use clap::ArgMatches;
use rayon::prelude::*;

use crate::Database;
use crate::Zettel;
use crate::config::ConfigOptions;

use crate::io::file_exists;

/// Print all Zettel in the given vector with the format: `[i] <TITLE>` if in inbox, and `[p]
/// <TITLE>` if in outbox.
fn print_zettel_info(zettel: &[Zettel])
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
    let db = Database::new(&cfg.db_file())?;
    db.init()?;

    let title = matches.value_of("TITLE").unwrap();
    let is_inbox = matches.is_present("inbox");

    let zettel = Zettel::new(title, is_inbox);

    let exists_in_fs = file_exists(&zettel.filename(cfg));
    let exists_in_db = db.all().unwrap().into_par_iter().any(|z| z == zettel);

    // If the corresponding file exists and there's an entry in the database, abort.
    // If there's a file but there's no entry in the database, create an entry.
    // Otherwise, create a new file from template and add a database entry.
    if exists_in_fs && exists_in_db {
        eprintln!("error: couldn't create new Zettel: one with the same title already exists");
        return Ok(());
    } else if exists_in_fs {
        println!("file exists in the filesystem but not in the database; added entry");
        // saved outside of the loop
    } else {
        zettel.create(cfg);
        print_zettel_info(&[zettel.clone()]); // confirm that the Zettel was created
    }
    db.save(&zettel)?;

    Ok(())
}

/// Update the metadata of a file
pub fn update(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;

    let path = matches.value_of("FILENAME").unwrap();
    if file_exists(path) {
        let zettel = Zettel::from_file(path);
        db.update(cfg, &zettel)?;
    } else {
        eprintln!("error: provided path isn't a file");
    }

    Ok(())
}

/// Print all Zettel matching the pattern from the CLI
pub fn query(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;

    let pattern = matches.value_of("PATTERN").unwrap_or_default();
    let result = db.find_by_title(pattern)?;
    print_zettel_info(&result);

    Ok(())
}

/// Print all Zettel whose tags contain the pattern specified in the CLI args
pub fn find(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;

    let input = matches.value_of("TAG").unwrap_or_default();

    let mut zettel = db.find_by_tag(input)?;
    let mut zettel_with_subtag = db.find_by_tag(&format!("{}/*", input))?;
    zettel.append(&mut zettel_with_subtag);
    zettel.par_sort();
    zettel.dedup();

    print_zettel_info(&zettel);

    Ok(())
}

/// Print all tags used inside the Zettelkasten
pub fn list_tags(cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;

    let tags = db.list_tags()?;
    tags.into_par_iter()
        .for_each(|t|
            if !t.is_empty() {
                println!("{}", t)
            }
        );

    Ok(())
}

/// Print the titles of the Zettel matching the pattern provided in the CLi arguments and the other
/// Zettel it links to under the following format:
///
/// ```
/// [<i>] <TITLE>
///     | <LINK_1>
///     | <LINK_2>
///     | ...
///     | <LINK_N>
/// ```
///
/// ...where `<i>` is the inbox status, `<TITLE>` is the title of the Zettel matching the pattern,
/// and `LINK_N` is the title of a Zettel linked to by `<TITLE>`
pub fn links(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let title = matches.value_of("TITLE").unwrap_or_default();
    let db = Database::new(&cfg.db_file())?;

    let zettel = db.find_by_title(title)?;
    for z in zettel {
        print_zettel_info(&[z.clone()]);
        for link in &z.links {
            println!("    | {}", link);
        }
    }
    Ok(())
}

/// Print all Zettel that match the one specified in the CLI argument matches
pub fn backlinks(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let title = matches.value_of("TITLE").unwrap_or_default();
    let db = Database::new(&cfg.db_file())?;

    let zettel = db.find_by_title(title)?;
    for z in zettel {
        print_zettel_info(&[z.clone()]);
        let res = db.find_by_links_to(&z.title)?;
        for blink in res {
            print!("    | ");
            print_zettel_info(&[blink]);
        }
    }
    Ok(())
}

/// Print all Zettel that contain the text provided in the CLI argument matches
pub fn search(matches: &ArgMatches, cfg: &ConfigOptions) -> Result<(), Error>
{
    let text = matches.value_of("TEXT").unwrap();

    let db = Database::new(&cfg.db_file())?;
    let results = db.search_text(cfg, text)?;
    print_zettel_info(&results);

    Ok(())
}

/// (Re)generate the database file
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

/// Print a list of Zettel that haven't yet been created
pub fn not_created(cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;

    let results = db.zettel_not_yet_created()?;
    for title in results {
        println!("{}", title);
    }

    Ok(())
}

/// List all files in the Zettelkasten
pub fn ls(cfg: &ConfigOptions) -> Result<(), Error>
{
    let db = Database::new(&cfg.db_file())?;
    let results = db.all()?;

    print_zettel_info(&results);

    Ok(())
}

/// Print the directory used as Zettelkasten
pub fn zettelkasten_dir(cfg: &ConfigOptions) -> Result<(), Error>
{
    println!("{}", cfg.zettelkasten);
    Ok(())
}
