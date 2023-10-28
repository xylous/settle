# Changelog

## v0.40.1 - 2023-10-28

- fix bug where not all tags would be added to the database and therefore `query
    --tag` results would have been incomplete
- fix bug where `sync --update` would keep old links and tags, even if they
    changed or were removed
- fix panics in `settle sync --rename` caused by trying to rename a file that
    didn't exist
- fix overwrites in `settle sync --rename` when there was a Zettel with the new
    title on the filesystem but didn't have an entry in the database
- `query --graph json`: specify `"ghost"` as edge weight if the target link does
    not exist
- `vizk`:
    - make nonexistent Zettel slightly darker in color and less opaque, to
        distinguish them from those that do exist
    - make drag distance detection dependent on node size, so that there isn't a
        visible mismatch between large nodes and their selection radii
    - change default parameters so that the notes don't cluster in the center
        when opening the visualisation
    - fix bug where link force greatly affected link distance at lower values
        for the former
    - add button to toggle displaying arrows at the end of each link
- `settle compl`: add completion for `Nushell` (through `nu`, `nushell`
    arguments)
- change `query` and `ls`: always print output alphabetically sorted
- change: don't enforce having an `inbox` project, since the user should have
    control over this

## v0.40.0 - 2023-08-31

- `query`: change `--graph` to take a single value insteaed of printing a DOT graph:
    - specifying `dot` as an argument prints the DOT graph
    - specifying `json` as an argument prints the JSON string of the graph
    - specifying `vizk` as an argument returns HTML code using `d3.js` under the
        hood, through which the Zettelkasten can be interactively visualised
- fix: make `sync --rename` take precisely two arguments instead of a variable
    amount
- fix: get default configuration file variables at runtime instead of compile
    time

## v0.39.11 - 2023-08-15

- add: make configuration file location flexible by allowing using the
    `SETTLE_CONFIG` environment option
- fix: check for environment variables at runtime, not at compile time
- fix: don't use environment variables if they are set but empty

## v0.39.10 - 2023-08-15

- change: make titles be unique *globally*, i.e. *a single unique title per
    Zettelkasten*, instead of having them be unique per-project basis.
- refactor: rework the database architecture entirely, but keep (roughly) the
    same functionality
- fix: make finding backlinks just as fast as finding forward links
- fix: make `sync --create` return the proper error
- fix: prevent `compl` from creating configuration files
- fix: make `compl` recognise its input properly
- fix: prevent `query` from returning a capacity overflow error when using the
    `--text` option.
- fix: don't insert duplicate links into the database
- fix: include more information in I/O panics, to make debugging easier

## v0.39.9 - 2023-06-10

- fix: automatically rename Zettel with multiple consecutive whitespace, so that
    words in them may be separated only by a single space character
- fix `query`: return backlinks properly, instead of just a subset, or none (!)
- clarify documentation
- update dependencies to latest versions

## v0.39.8 - 2023-05-21

- add `main` as an alias for the main Zettelkasten project, i.e. the empty
    string project, for all commands that may reference it
- fix `settle sync --move`: use regular regex instead of SQL regex to match
    notes

## v0.39.7 - 2023-05-19

- `settle query`: fix bad output of `--graph` command, which (due to the v0.39.6
    update) printed not only its intended output, but also the regular query
    output.

## v0.39.6 - 2023-05-14

- `settle query`: add `%a` format flag, which, when used in combination with the
    `--text` option, prints the patterns matched by the latter option. This is
    especially useful for giving context to a search, since enclosing it with a
    `.*` at the start and end (such as `.*search.*`) returns an entire line.

## v0.39.5 - 2023-01-30

- add `--exact` option to `settle query`, for disabling regex and at the same
    time avoiding any possible [internal] errors due to regex-specific
    characters in Zettel titles
- re-write documentation entirely, add a bunch of files to `doc/` and link to
    them in the README.md "Overview" section

## v0.39.4 - 2023-01-25

- fix: `settle sync --create` no longer permits empty titles and titles starting
    with a dot (which would be, as per regular Unix behaviour, hidden files)
- fix: `settle sync --generate` now ignores files starting with a dot, and
    likewise, empty titles
- fix: `settle sync --update` no longer panics when given certain relative
    paths, notably when the current working directory is the main Zettelkasten
    directory
- fix: `settle sync --update` returns an error on files that are not in the
    Zettelkasten
- fix: if the database file doesn't exist or the database hasn't been
    initialised, `settle` no longer panics complaining that it can't find SQL
    tables

## v0.39.3 - 2023-01-25

- fix: detected tags must be delimited by whitespace on both sides, so that for
    example it doesn't think that a reference in a link is a tag (e.g.
    `https://example.com/hello#About`; formerly, it would take `#About` as a
    tag)
- fix: `query --tag <TAG>` also returns the subtags of `<TAG>`
- fix: `sync --rename` would never work since it never found the correct title

## v0.39.2 - 2023-01-24

- add `--graph` option to `query`, for making a graph out of the results

## v0.39.1 - 2023-01-24

- add to `query`:
    - add `--format` option, which supports printing the title, the project, the
        path, the links and the backlinks for any Zettel.
    - add `--link_sep` option, which specifies how both links and backlinks
        should be separated in formatting
- remove groff/man document inside `doc/`

## v0.39.0 - 2023-01-22

- change command line interface entirely:
    - remove all listing-related commands
    - add `query` command for querying the database, with the ability to to
        apply several filter parameters, at your choice
    - remove all commands that were related to changing the database
    - add `sync` command, which deals with everything related to changing the
        database
    - replace misc commands (like `zk`, `tags`) with `ls`
- [internal] instead of using SQL queries to get stuff, load the entire
    database into memory and then apply various filters

## v0.38.1 - 2022-08-11

- fix panics on a completely fresh environment (credit: irandms)

## v0.38.0 - 2022-08-01

- add `isolated` command, for returning all Zettel that have no links pointing
    to or from them

## v0.37.2 - 2022-06-12

- fix: when moving a note from a project to another, create the project
    directory if it doesn't exist

## v0.37.1 - 2022-06-12

- remove `db_file` as a configuration option
- add a 'core principles' section to the README
- briefly describe the project's history in the README

## v0.37.0 - 2022-06-01

- add `mv` command, for moving Zettel between projects
- add `rename` command, for renaming Zettel obviously
    - after a file is renamed, all links to it are updated
- fix: properly recognise links spanning multiple lines

## v0.36.5 - 2022-05-21

- fix: print things synchronously instead of asynchronously to stdout

## v0.36.4 - 2022-05-12

- reformulate a vague section in the README file
- fix: don't panic when compiling if `XDG_CONFIG_HOME` isn't set

## v0.36.3 - 2022-04-11

- add the 'projects' command for listing existing projects
- add `buildman.sh` in `docs/`, a script that builds the man documentation
    automatically

## v0.36.2 - 2022-04-03

- add a readable manual in markdown (which works on e.g. GitHub)
- use backticks in the 'Wildcards' section in the markdown manual
- update the markdown manual's formatting
- move a paragraph to its proper position in the manuals
- add a section on how to actually take notes with it

## v0.36.1 - 2022-03-07

- process projects in parallel
- use the latest versions of dependencies

## v0.36.0 - 2022-03-06

- implement projects (subdirectories within the main Zettelkasten) to hold notes

## v0.35.0 - 2022-02-07

- add `${DATE}` placeholder

## v0.34.0 - 2022-01-17

- rename all commands: use at most one word

## v0.33.1 - 2022-01-15

- follow XDG base specifications for configuration files
- add a proper man page that uses groff

## v0.33.0 - 2022-01-09

- add command to generate autocompletion files on the go
