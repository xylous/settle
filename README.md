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

### Usage

Read [the manual](./doc/settle.1.md)

## Roadmap

- [x] generate database from existing files
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
- [x] list mentioned but non-existent Zettel
- [x] ~~use an inbox~~
- [x] search for text inside notes
- [x] support matching titles with wildcards
- [x] update Zettel metadata individually
- [x] print links
    - [x] forward links (from a Zettel to other Zettel)
    - [x] backlinks (to a Zettel from other Zettel)
- [x] projects (subdirectories within the main Zettelkasten folder)
    - [x] add notes to projects
    - [x] generate the database with projects included
    - [x] start with an 'inbox' project by default

##### Meta

- [x] configure `Cargo.toml` properties
- [x] shell autocompletion
    - [x] zsh
    - [x] bash
    - [x] fish
- [x] write a proper `man` page
- [ ] logo (pixel art?)
- [x] publish to crates.io

## Contributing

Pull requests are welcome. For major changes, please open an issue first to
discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](LICENSE)
