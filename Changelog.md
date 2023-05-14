# Changelog

NOTE: This Changelog is partially incomplete.

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
