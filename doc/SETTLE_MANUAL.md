# settle

Settle is a Zettelkasten note manager. All notes are written in Markdown and are
stored locally.

As of v0.26.0, Settle no longer supports editing a file directly.
Therefore, you're probably going to want to use (or write) a plugin for your
favourite editor, such as [settle.vim](https://github.com/xylous/settle.vim) for
Vim and Neovim.

## Synopsis

```
settle [--help | -h | --version | -v]
settle {sync | -S} [-p | -c | -u | -g | -m | -n]
settle {query | -Q} [-t | -p | -g | -x | -l | -b | -o]
settle ls ['tags' | 'projects' | 'ghosts' | 'path']
```

## Options

- `-h`, `--help` - Print usage information

- `-V`, `--version` - Print version information

## Commands

- `help` - Print usage information broadly or, if a subcommand is given, usage
    information for said subcommand

- `compl <SHELL>` - Generate autocompletion file for a certain shell (currently
    supported: zsh, bash, fish) (see: section on autocompletion)

- `ls <OBJECT>` - list things that are not directly related to notes. The only
    accepted `<OBJECT>` values are `tags`, `projects`, `ghosts`, and `path`. The
    first prints all unique tags, the second prints all projects in your
    Zettelkasten, the third prints all Zettel that have links pointing to them
    but have not yet been created, and the last one prints the path to the
    Zettelkasten (as per your configuration options).

### Options for `query` (`-Q`)

Note that the various options listed here all compound - that is to say, every
option acts as a criteria for querying. `settle query --title "Foo.*" --tag
"bar"` will only return notes whose titles starts with `Foo` AND have the tag
`bar`, not notes whose titles start with `Foo` OR have the tag `bar`. By
default, when no filter parameter is applied (that is to say, `settle query` is
ran without options), the entire Zettelkasten is returned.

- `-t | --title <REGEX>` - keep Zettel whose title matches `<REGEX>`

- `-p | --project <REGEX>` - keep Zettel that are in projects that match `<REGEX>`

- `-g | --tag <REGEX>` - keep Zettel that have at least one tag that matches `<REGEX>`

- `-x | --text <REGEX>` - keep Zettel whose text contents match `<REGEX>`

- `-l | --links <REGEX>` - for the Zettel whose title matches `<REGEX>`,
    keep all the Zettel that they have a link pointing to

- `-b | --backlinks <REGEX>` - for the Zettel whose title matches `<REGEX>`,
    keep all Zettel that have a link pointing to them

- `-o | --loners` - keep Zettel that have no links pointing to other zettel AND
    have no links pointing to them.

- `-f | --format <FORMAT>` - every Zettel is printed according to `<FORMAT>`.
    Several formatting flags are supported, such as:
    - `%t` - replaced with the title
    - `%p` - replaced with the project name
    - `%P` - replaced with the absolute path to the Zettel
    - `%l` - replaced with the (forward) links of the Zettel
    - `%b` - replaced with the backlinks of the Zettel; note that since `settle`
        only stores forward links in the database, fetching backlinks is a
        little bit more time consuming

- `-s | --link_sep <SEPARATOR>` - can only be used together with `--format`, in
    which case it specifies the separator used between both forward links and
    backlinks.

Here are a few concrete examples:

- `settle query --text "sample" --loners` returns all notes that contain `sample`
    in their text and that aren't linked with any other note in the
    Zettelkasten.

- `settle query --project "" --title ".*word.*"` returns all notes that are in
    the main Zettelkasten (the empty-string project) and have the word `word`
    within their title.

- `settle query --formatting "[%p] %t" --link_sep " | "` is the same as the
    default format. Note that, since no links are printed, the separator is
    actually never used for this format.

- `settle query --tag "literature" --links "Neurons"` returns all notes that
    have the `literature` tag and link to a note called *precisely* `Neurons`
    (note the absence of regex wildcards)

- `settle query --format "[%P]\t%l" --link_sep "\t" --title "Note.*"` takes
    every Zettel whose title starts with `Note`, printing their absolute path
    between square brackets, separating links with tabs.

### Options for `sync` (`-S`)

Note that, unlike the query command, the options that do take arguments here
don't work with regex (except `--move`). Matches here need to be exact, since
we're dealing with more or less precise database changes. Also, unless
specified otherwise, most/all options are mutually exclusive.

- `-p | --project <PROJECT>` - this option actually doesn't do anything on its
    own, instead being used as a helper option to `--create` and `--move`

- `-c | --create <TITLE>` - create a new Zettel with the provided title. If the
    `--project` flag is provided, then make it part of that project

- `-u | --update <PATH>` - update a note's metadata, given its path on the
    filesystem

- `-g | --generate` - (re)generate the entire database based on what's in the
    Zettelkasten directory

- `-m | --move <REGEX>` - this option requires the `--project` option; all
    Zettel whose title matches `<REGEX>` are moved to the specified project

- `-n | --rename <ARGS...>` - this option accepts multiple values; however, it
    only renames the first Zettel whose title it can find in the database, with
    the name specified by the last argument in the list. If the names coincide,
    or if there's no valid Zettel title in the list, or if by renaming it would
    overwrite some files, then it aborts. NOTE: the project of the renamed
    Zettel is not changed. Also note that all links pointing to the previous
    Zettel's title are changed, so that the links point to the same file.

## Configuration

The configuration file is at either `$XDG_CONFIG_HOME/settle/settle.yaml`, if
`$XDG_CONFIG_HOME` is set, either `~/.config/settle/settle.yaml`, by default.

- `zettelkasten` - directory in which the notes are stored at

    If you don't specify an absolute path, e.g. `notes`, it's assumed you want
    your Zettelkasten to be at `~/notes`. You can also use paths containing
    environment variables or paths starting with a tilde (`~`)

- `template` - Path to Zettel template

    If empty, or if the path is invalid, then templates won't be used. You can
    use paths containing environment variables, or a leading tilde (`~`).

## Templates

Template files are used when creating new Zettel. The text they contain gets put
inside said new note, replacing variables.

### Placeholders

- `${TITLE}` - placeholder for the actual title
- `${DATE}` - replaced with the output of `date +%Y-%m-%d`

### Example template

```md
# ${TITLE}



### References


```

## Autocompletion

Shell completions can be generated by the user at runtime, by using the `compl`
command. In most cases, you'll need to create a directory for user-defined
completions, then add `settle`'s output to it.

If you want XDG compliance, you probably know what you're doing, so just replace
a few things here and there.

### bash

Add the following text to the `~/.bash_completion` file:

```bash
for comp_file in ~/.bash_completion.d/*; do
    [[ -f "${comp_file}" ]] && . "${comp_file}"
done
```

And then run the following commands:

```bash
mkdir ~/.bash_completion.d
settle compl bash >~/.bash_completion.d/settle
```

### zsh

In your terminal, run:

```zsh
mkdir ~/.zsh_completion.d
settle compl zsh >~/zsh_completion.d/_settle
```

Then add this line in your zshrc:

```zsh
fpath=(${HOME}/.zsh_completion.d $fpath)
```

### fish

Run the following commands:

```fish
mkdir -p ~/.config/fish/completions
settle compl fish >~/.config/fish/completions/settle.fish
```

## License & Credits

[Licensed under MIT](../LICENSE)

Written by xylous \<xylous.e@gmail.com\>
