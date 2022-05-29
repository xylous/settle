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

For the commands, options, configuration, and setting up autocompletion, read
[the manual](./doc/SETTLE_MANUAL.md)

If you prefer, there's also a groff document inside the `doc/` dirctory which
can be read with `man`. On the command line, of course.

### The note-taking system

`settle` just stores and manages a database of Zettel metadata. That's it.

There are two important things to remember when writing:

- wiki-style links are used to denote the actual links between Zettel. For
    example, a link such as `[[Neurons]]` would be considered as linking to a
    note called *precisely* 'Neurons'. These can appear anywhere in the note.
- every word with a hashtag prefix is treated as a tag. For example,
    `#psychology`. Subtags (hierarchical tags), such as
    `#biology/anatomy/humans` are supported, if you want to stay more organised.
    You can also put these everywhere.

Besides, how do you actually start writing, if there are no explicit commands to
invoke an editor? It'd have been hard trying to write code so that it works
smoothly with vim or emacs and their many quirks. Instead, editor-side code is
written to act as a settle plugin.

I wrote [settle.vim](https://github.com/xylous/settle.vim) since I'm a (neo)vim
user myself. If you write a wrapper around settle, contact me at
`xylous.e@gmail.com` and I'll make a list or something.

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
    - [ ] move notes from project to project
- [ ] rename notes
    - [ ] update all links to the renamed note

##### Meta

- [x] configure `Cargo.toml` properties
- [x] shell autocompletion
    - [x] zsh
    - [x] bash
    - [x] fish
- [x] write a proper `man` page
- [ ] logo (pixel art?)
- [x] publish to crates.io
- [ ] describe the design choices of settle-rs

## Contributing

Pull requests are welcome. For major changes, please open an issue first to
discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](LICENSE)
