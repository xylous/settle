# settle

`settle` is a CLI tool that helps you manage your digital Zettelkasten.

## Getting started

### Requirements

* cargo/rust toolchain

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

```
$ settle [SUBCOMMAND] [...params]`
```

Consult `settle help` for subcommands.

Example usage:

```
$ settle new 'A super interesting note!'
```

## Roadmap

- [x] ~~compile to HTML; requires `pandoc`~~
- [x] ~~update backlinks automatically~~
- [x] ~~tags; specifically, search by tags~~
- [x] ~~transient/draft notes~~
- [ ] full text search
- [ ] look for the context of a link when generating backlinks
    - [x] ~~paragraphs~~
    - [ ] lists

## Contributing

Pull requests are welcome. For major changes, please open an issue first to
discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](LICENSE)
