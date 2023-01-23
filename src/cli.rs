use clap::{Arg, Command};

/// Generate the clap App by using a builer pattern
pub fn build() -> Command<'static>
{
    Command::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author("xylous <xylous.e@gmail.com>")
    .about("CLI tool to manage a digital Zettelkasten")

    .subcommand_required(true)
    .arg_required_else_help(true)

    .subcommand(Command::new("compl")
        .arg(Arg::new("SHELL").required(true))
        .about("generate completion file for a given shell")
    )

    .subcommand(Command::new("sync")
        .short_flag('S')
        .about("sync the database")
        .arg(Arg::new("PROJECT")
            .conflicts_with_all(&["UPDATE", "RENAME", "GENERATE"])
            .short('p')
            .long("project")
            .takes_value(true)
            .help("specify project")
        )
        .arg(Arg::new("CREATE")
            .conflicts_with_all(&["UPDATE", "MOVE", "RENAME", "GENERATE"])
            .short('c')
            .long("create")
            .takes_value(true)
            .help("create a new Zettel")
        )
        .arg(Arg::new("UPDATE")
            .conflicts_with_all(&["MOVE", "RENAME", "GENERATE"])
            .short('u')
            .long("--update")
            .takes_value(true)
            .help("update a note's metadata based on its path")
        )
        .arg(Arg::new("MOVE")
            .conflicts_with_all(&["RENAME", "GENERATE"])
            .short('m')
            .long("move")
            .takes_value(true)
            .requires("PROJECT")
            .help("move the matching Zettel to a project; requires --project")
        )
        .arg(Arg::new("RENAME")
            .conflicts_with("GENERATE")
            .short('n')
            .long("rename")
            .min_values(2)
            .help("rename first argument to last argument, preserving project")
        )
        .arg(Arg::new("GENERATE")
            .short('g')
            .long("--generate")
            .takes_value(false)
            .help("(re)generate the database")
        )
    )

    .subcommand(Command::new("query")
        .short_flag('Q')
        .about("apply filter parameters and return matching Zettel")
        .arg(Arg::new("TITLE")
            .short('t')
            .long("title")
            .takes_value(true)
            .help("keep Zettel with a matching title")
        )
        .arg(Arg::new("PROJECT")
            .short('p')
            .long("project")
            .takes_value(true)
            .help("keep Zettel that are in the matching projects")
        )
        .arg(Arg::new("TAG")
            .short('g')
            .long("tag")
            .takes_value(true)
            .help("keep Zettel that have a matching tag name")
        )
        .arg(Arg::new("TEXT")
            .short('s')
            .long("text")
            .takes_value(true)
            .help("keep Zettel that contain some text")
        )
        .arg(Arg::new("LINKS")
            .short('l')
            .long("links")
            .takes_value(true)
            .help("keep Zettel that have links to the matching Zettel")
        )
        .arg(Arg::new("BACKLINKS")
            .short('b')
            .long("backlinks")
            .takes_value(true)
            .help("keep Zettel that have links from the matching Zettel")
        )
        .arg(Arg::new("ISOLATED")
            .short('i')
            .long("isolated")
            .takes_value(false)
            .help("keep Zettel that don't have any links to and fro")
        )
    )

    .subcommand(Command::new("ls")
        .about("list things in the database")
        .arg(Arg::new("OBJECT")
            .required(true)
            .help("object to list (tags, projects, ghosts, path)"
        )
    ))
}
