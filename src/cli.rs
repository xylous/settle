use clap::{Arg, Command};

/// Generate the clap App by using a builer pattern
pub fn build() -> Command<'static>
{
    Command::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author("xylous <xylous.e@gmail.com>")
    .about("CLI tool to manage a digital Zettelkasten")
    .subcommand(
        Command::new("compl")
            .arg(Arg::new("SHELL").required(true))
            .about("generate completion file for a given shell"),
    )
    .subcommand(
        Command::new("sync")
            .about("sync the database")
            .subcommand(
                Command::new("create")
                    .about("create a new Zettel")
                    .arg(
                        Arg::new("PROJECT")
                            .short('p')
                            .long("project")
                            .takes_value(true)
                            .help("specify project"),
                    )
                    .arg(Arg::new("TITLE").required(true).help("title of Zettel")),
            )
            .subcommand(
                Command::new("update")
                    .about("update the metadata of a Zettel")
                    .arg(Arg::new("FILENAME").required(true).help("path to Zettel")),
            )
            .subcommand(
                Command::new("mv")
                    .about("move all matches into the given project")
                    .arg(
                        Arg::new("PATTERN")
                            .required(true)
                            .help("a pattern/regex for the Zettel titles"),
                    )
                    .arg(
                        Arg::new("PROJECT")
                            .required(true)
                            .help("the project into which notes are put"),
                    ),
            )
            .subcommand(
                Command::new("rename")
                    .about("rename a Zettel")
                    .arg(Arg::new("TITLE").required(true))
                    .arg(Arg::new("NEW_TITLE").required(true)),
            )
            .subcommand(Command::new("generate").about("generate the database"))
    )
    .subcommand(
        Command::new("query")
            .about("apply filter parameters and return matching Zettel")
            .arg(
                Arg::new("TITLE")
                    .short('t')
                    .long("title")
                    .takes_value(true)
                    .help("keep Zettel with a matching title"),
            )
            .arg(
                Arg::new("TAG")
                    .short('g')
                    .long("tag")
                    .takes_value(true)
                    .help("keep Zettel that have a matching tag name"),
            )
            .arg(
                Arg::new("TEXT")
                    .short('s')
                    .long("text")
                    .takes_value(true)
                    .help("keep Zettel that contain some text"),
            )
            .arg(
                Arg::new("LINKS")
                    .short('l')
                    .long("links")
                    .takes_value(true)
                    .help("keep Zettel that have links to the matching Zettel"),
            )
            .arg(
                Arg::new("BACKLINKS")
                    .short('b')
                    .long("backlinks")
                    .takes_value(true)
                    .help("keep Zettel that have links from the matching Zettel"),
            )
            .arg(
                Arg::new("PROJECT")
                    .short('p')
                    .long("project")
                    .takes_value(true)
                    .help("keep Zettel that are in the matching projects"),
            )
            .arg(
                Arg::new("ISOLATED")
                    .short('i')
                    .long("isolated")
                    .takes_value(false)
                    .help("keep Zettel that don't have any links to and fro"),
            ))
    .subcommand(
        Command::new("ls")
            .about("list things in the database")
            .arg(
                Arg::new("OBJECT")
                    .required(true)
                    .help("object to list (tags, projects, ghosts, path)"),
    ))
}
