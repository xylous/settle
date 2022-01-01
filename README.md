# settle

`settle` is a CLI tool that helps you manage your digital Zettelkasten.

## Getting started

### Requirements

* cargo/rust toolchain
* SQLite

### Installation

Clone this repository locally, for example:

```
git clone https://github.com/xylous/settle settle
```

And then build:

```
cd settle/
cargo build
```

### Usage

Read [the User Manual](./USER-MANUAL.md)

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
- [x] use an inbox
- [x] search for text inside notes
- [x] support matching titles with wildcards
- [x] update Zettel metadata individually
- [x] print links
    - [x] forward links (from a Zettel to other Zettel)
    - [x] backlinks (to a Zettel from other Zettel)
- [ ] list "lonely" Zettel (with zero connections)

##### Meta

- [ ] configure `Cargo.toml` properties
- [ ] shell autocompletion
    - [ ] zsh
    - [ ] bash
- [ ] write a proper `man` page
- [ ] logo (pixel art?)
- [ ] publish to crates.io

## Contributing

Pull requests are welcome. For major changes, please open an issue first to
discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](LICENSE)
