# settle

`settle` is a CLI tool that helps you manage your digital Zettelkasten.

## Getting started

### Requirements

* cargo/rust toolchain
* SQLite

### Installation

There's a crate on crates.io, so you can simply run:

```
cargo install settle
```

### Overview

- [design principles](./doc/design-principles.md)
- [project history](./doc/history.md)
- [configuration](./doc/configuration.md)
- [full usage manual](./doc/SETTLE_MANUAL.md), contains more technical descriptions
- [creating new notes (with templates as well)](./doc/creating-notes.md)
- [projects](./doc/projects.md)
    - [moving notes between projects](./doc/moving-notes-between-projects.md)
- [renaming notes](./doc/renaming-notes.md)
- [keeping the database up to date](./doc/keeping-the-database-up-to-date.md)
- [tags and subtags](./doc/tags-and-subtags.md)
- [links and backlinks](./doc/links-and-backlinks.md)
- [query, search and filter](./doc/query-search-and-filter.md)
    - [making a graph of your Zettelkasten](./doc/graphs.md)

## Roadmap

#### Before 2023

- [x] generate the database from existing files
- [x] create Zettel
- [x] list Zettel
- [x] tags
    - [x] recognize hashtag-tags (e.g. `#interesting-tag`)
    - [x] search for tags
    - [x] list all tags
- [x] configuration
    - [x] custom Zettelkasten directory
    - [x] custom database file path
    - [x] be able to use templates
- [x] list mentioned but non-existent Zettel ("ghosts")
- [x] search for text inside notes
- [x] update Zettel metadata individually
- [x] projects (subdirectories within the main Zettelkasten folder)
    - [x] add notes to projects
    - [x] create an inbox project by default
    - [x] generate the database with projects included
    - [x] start with an 'inbox' project by default
    - [x] move notes from project to project
- [x] rename notes
    - [x] update all links to the renamed note

#### After/During 2023

- [x] query: filter notes based on various criteria (title, tags, etc.)
    - [x] support regex
    - [x] print according to a format
    - [x] put custom separator between links, both forward and backward
- [x] graph
    - [x] DOT output
    - [ ] ~~render DOT as image~~
- [ ] writing experience (help deal with writer's block)
    - [ ] find related notes
    - [ ] suggest random notes

## Contributing

Pull requests are welcome. For major changes, please open an issue first to
discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](LICENSE)
