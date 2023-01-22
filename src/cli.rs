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
        Command::new("new")
            .about("create a new Zettel and print its inbox status and title")
            .arg(
                Arg::new("PROJECT")
                    .short('p')
                    .long("project")
                    .takes_value(true)
                    .help("create the new Zettel in a specified project"),
            )
            .arg(Arg::new("TITLE").required(true).help("title of Zettel")),
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
    .subcommand(
        Command::new("update")
            .about("update the metadata of a Zettel")
            .arg(Arg::new("FILENAME").required(true).help("path to Zettel")),
    )
    .subcommand(
        Command::new("query")
            .about("query (filter/list) various things in the database")
            .arg(
                Arg::new("TITLE")
                    .short('t')
                    .long("title")
                    .takes_value(true)
                    .help("FILTER Zettel with a matching title"),
            )
            .arg(
                Arg::new("BY_TAG")
                    .short('g')
                    .long("tag")
                    .takes_value(true)
                    .help("FILTER Zettel that have a specific tag"),
            )
            .arg(
                Arg::new("TEXT")
                    .short('s')
                    .long("text")
                    .takes_value(true)
                    .help("FILTER Zettel that contain a specific text"),
            )
            .arg(
                Arg::new("ISOLATED")
                    .short('i')
                    .long("isolated")
                    .takes_value(false)
                    .help("FILTER Zettel that don't have any links to and fro"),
            )
            .arg(
                Arg::new("FWLINKS")
                    .short('l')
                    .long("links")
                    .takes_value(true)
                    .help("list forward links"),
            )
            .arg(
                Arg::new("BACKLINKS")
                    .short('b')
                    .long("backlinks")
                    .takes_value(true)
                    .help("list backlinks"),
            )
            .arg(
                Arg::new("PROJECTS")
                    .long("projects")
                    .takes_value(false)
                    .help("list all projects"),
            )
            .arg(
                Arg::new("TAGS")
                    .long("tags")
                    .takes_value(false)
                    .help("list all tags"),
            )
            .arg(
                Arg::new("GHOSTS")
                    .long("ghosts")
                    .takes_value(false)
                    .help("list 'Ghost' Zettel, that have been mentioned, but not created"),
            )
    )
    .subcommand(Command::new("generate").about("(re)generate the database"))
    .subcommand(Command::new("zk").about("return the path to the Zettelkasten"))
}
