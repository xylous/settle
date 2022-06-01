# Changelog

NOTE: This Changelog is incomplete.

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
