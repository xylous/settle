# settle

`settle` is a CLI tool that helps you manage your digital Zettelkasten.

First, a little bit of history. I learned about the Zettelkasten method back in
the summer of 2021. I looked at a few programs for it, and I settled on Obsidian
MD. But I didn't like the experience: I was an avid Vim user, and the vim
compatibility mode wasn't usable in the least. I had alreay written quite a few
notes, and I didn't want to change them to make the links and tags work with
other programs.

So there, in early August, I had the idea of writing a CLI program that I could
easily use with vim (or any editor, for that matter), and at the same time use
Obsidian-style links and tags. In the meantime, I've read Sonke Ahrens's *How to
take smart notes* and have added many features to settle. Personally, I'm
content.

There are several core principles in the design:

- ***plain and simple***: notes are stored locally and written in markdown

- ***manage notes, not editors***: it's the same Zettelkasten everywhere you go.
    The editor you use doesn't matter. Integration is done through (editor)
    plugins (e.g. settle.vim).

- ***database mirrors notes***: metadata is determined by what's on the file
    system, not by commands. The only way to add or remove links and tags is to
    write, then tell `settle` to update the note(s).

- ***you can use projects, but take care***: instead of using tags and putting
    things in the main Zettelkasten, completely unrelated notes like those
    containing games' lore or chapters of a book you're writing can be put in a
    *project*. Actually, the root of your Zettelkasten is a project, and your
    inbox is also a project, and any other sub-"box" (like one for
    literature/book notes) you'd want could be shaped as a project.

    However, the separation into projects is only formal, since links can
    reference notes in any project. It's really easy to misuse them:
    from what I understood in Sonke Ahrens' book, they're meant to be discarded
    after the project is done, or incorporated into the main Zettelkasten.
    Either way, they're *not* supposed to be permanent.

- ***add, change, but never remove***: notes may be created, but never destroyed
    by the program. At most, they can be renamed or moved from project to
    project.

- ***made for humans***: zero YAML metadata. Titles are plain text. Links and tags
    are directly embedded within your note's contents, allowing you to give
    context to connections between ideas.

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

### The note-taking system

`settle` just stores and manages a database of Zettel metadata. A simple record
of the note's title, the project it's in, what other notes it links to, and what
tags it has. That's all.

There are two important things to remember when writing:

- wiki-style links are used to denote the actual links between Zettel. For
    example, a link such as `[[Neurons]]` would be considered as linking to a
    note called *precisely* 'Neurons'. These can appear anywhere in the note.
- every word with a hashtag prefix is treated as a tag. For example,
    `#psychology`. Subtags (hierarchical tags), such as
    `#biology/anatomy/humans` are supported, if you want to stay more organised.
    You can also put these everywhere.

`settle` wasn't designed to work with any editor in particular. Which means that
you can use it practically everywhere! I wrote
[settle.vim](https://github.com/xylous/settle.vim) since I'm a (neo)vim user
myself. Who knows? If I ever switch editors, I may write another.

Although plugins aren't necessary, they make things easier, by automating some
functions, such as telling `settle` to update a note's metadata.

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
- [x] list mentioned but non-existent Zettel ("ghosts")
- [x] search for text inside notes
- [x] update Zettel metadata individually
- [x] print links
    - [x] forward links (from a Zettel to other Zettel)
    - [x] backlinks (to a Zettel from other Zettel)
- [x] projects (subdirectories within the main Zettelkasten folder)
    - [x] add notes to projects
    - [x] create an inbox project by default
    - [x] generate the database with projects included
    - [x] start with an 'inbox' project by default
    - [x] move notes from project to project
- [x] rename notes
    - [x] update all links to the renamed note
(early 2023)
- [x] query: filter notes based on various criteria (title, tags, etc.)
    - [x] support regex
    - [x] print according to a format
    - [x] put custom separator between links, both forward and backward
- [ ] suggest random notes for you to read, to help deal with writer's block
- [ ] generate an image of a directed graph representing all links between notes

## Contributing

Pull requests are welcome. For major changes, please open an issue first to
discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](LICENSE)
