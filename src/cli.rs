//use clap::{Arg, Command};
use clap::builder::{Arg, ArgAction, Command};

/// Generate the clap App by using a builer pattern
pub fn build() -> Command
{
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("xylous <xylous.e@gmail.com>")
        .about("CLI tool to manage a digital Zettelkasten")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("sync")
                .display_order(1)
                .short_flag('S')
                .about("sync the database")
                .arg(
                    Arg::new("PROJECT")
                        .display_order(1)
                        .conflicts_with_all(["UPDATE", "RENAME", "GENERATE"])
                        .short('p')
                        .long("project")
                        .num_args(1)
                        .help("helper option to --create and --move; specify working project"),
                )
                .arg(
                    Arg::new("CREATE")
                        .display_order(2)
                        .conflicts_with_all(["UPDATE", "MOVE", "RENAME", "GENERATE"])
                        .short('c')
                        .long("create")
                        .num_args(1)
                        .value_name("TITLE")
                        .help("create a new Zettel"),
                )
                .arg(
                    Arg::new("UPDATE")
                        .display_order(3)
                        .conflicts_with_all(["MOVE", "RENAME", "GENERATE"])
                        .short('u')
                        .long("update")
                        .num_args(1)
                        .value_name("PATH")
                        .help("update a note's metadata, given its path"),
                )
                .arg(
                    Arg::new("GENERATE")
                        .display_order(4)
                        .short('g')
                        .long("generate")
                        .num_args(0)
                        .help("(re)generate the database"),
                )
                .arg(
                    Arg::new("MOVE")
                        .display_order(5)
                        .conflicts_with_all(["RENAME", "GENERATE"])
                        .short('m')
                        .long("move")
                        .num_args(1)
                        .requires("PROJECT")
                        .value_name("REGEX")
                        .help("move the matching Zettel to a project; requires --project"),
                )
                .arg(
                    Arg::new("RENAME")
                        .display_order(6)
                        .conflicts_with("GENERATE")
                        .short('n')
                        .long("rename")
                        .num_args(2)
                        .value_name("TITLE")
                        .help("rename a note, preserving project and updating backlinks"),
                ),
        )
        .subcommand(
            Command::new("query")
                .display_order(2)
                .short_flag('Q')
                .about("query the database")
                .arg(
                    Arg::new("TITLE")
                        .display_order(1)
                        .short('t')
                        .long("title")
                        .num_args(1)
                        .value_name("REGEX")
                        .help("keep Zettel with a matching title"),
                )
                .arg(
                    Arg::new("PROJECT")
                        .display_order(2)
                        .short('p')
                        .long("project")
                        .num_args(1)
                        .value_name("REGEX")
                        .help("keep Zettel that are in the matching projects"),
                )
                .arg(
                    Arg::new("TAG")
                        .display_order(3)
                        .short('g')
                        .long("tag")
                        .num_args(1)
                        .value_name("REGEX")
                        .help("keep Zettel that have a matching tag name"),
                )
                .arg(
                    Arg::new("TEXT_REGEX")
                        .display_order(4)
                        .short('x')
                        .long("text")
                        .num_args(1)
                        .value_name("REGEX")
                        .help("keep Zettel that contain some text"),
                )
                .arg(
                    Arg::new("LINKS")
                        .display_order(5)
                        .short('l')
                        .long("links")
                        .num_args(1)
                        .value_name("REGEX")
                        .help("keep Zettel that have links to the matching Zettel"),
                )
                .arg(
                    Arg::new("BACKLINKS")
                        .display_order(6)
                        .short('b')
                        .long("backlinks")
                        .num_args(1)
                        .value_name("REGEX")
                        .help("keep Zettel that have links from the matching Zettel"),
                )
                .arg(
                    Arg::new("LONERS")
                        .display_order(7)
                        .short('o')
                        .long("loners")
                        .num_args(0)
                        .action(ArgAction::SetTrue)
                        .help("keep Zettel that don't have any links to and fro"),
                )
                .arg(
                    Arg::new("FORMAT")
                        .display_order(8)
                        .short('f')
                        .long("format")
                        .num_args(1)
                        .help("print formatted"),
                )
                .arg(
                    Arg::new("LINK_SEP")
                        .display_order(9)
                        .short('s')
                        .long("link_sep")
                        .num_args(1)
                        .value_name("SEPARATOR")
                        .requires("FORMAT")
                        .help("specify separator for links and backlinks in formatted output"),
                )
                .arg(
                    Arg::new("GRAPH")
                        .conflicts_with_all(["FORMAT", "LINK_SEP"])
                        .display_order(10)
                        .long("graph")
                        .num_args(1)
                        //.action(ArgAction::SetTrue)
                        .help("turn the query results into a graph: 'dot', 'json' or 'vizk'"),
                )
                .arg(
                    Arg::new("EXACT_MATCH")
                        .display_order(11)
                        .short('e')
                        .long("exact")
                        .num_args(0)
                        .action(ArgAction::SetTrue)
                        .help("match everything exactly, disabling regex"),
                ),
        )
        .subcommand(
            Command::new("ls")
                .display_order(3)
                .about("list things not related to notes")
                .arg(
                    Arg::new("OBJECT")
                        .required(true)
                        .help("object to list (tags, projects, ghosts, path)"),
                ),
        )
        .subcommand(
            Command::new("compl")
                .display_order(5)
                .arg(Arg::new("SHELL").required(true))
                .about("generate completion file for a given shell"),
        )
}
