# settle's User Manual

`settle` is a command line note manager for the Zettelkasten method.

- exclusively recognizes wiki-style links (such as `[[Neurons]]`, that redirects
you to a Zettel titled "Neurons")
- looks for hashtag-tags, like `#psychology`. You can also use subtags, such as
`#biology/neuroscience`
- everything is stored locally
- set a custom Zettelkasten directory to contains notes
- [create Zettel from templates](#writing-templates)

Notes are written in Markdown. The filenames have a simple format,
`${TITLE}.md`.

NOTE: you're probably going to want to use (or write) a plugin for your
favourite editor, such as [settle.vim](https://github.com/xylous/settle.vim) for
Vim and Neovim; as of v0.26.0, the program no longer supports starting an editor
on a file.

## Configurability

The config file is located at `~/.config/settle/settle.yaml`. When you first
run the program, it's going to be generated automatically with default values.

You can specify the following options:
- `zettelkasten`: path to Zettelkasten. If you don't specify an absolute path,
e.g. `notes`, it's assumed you want your Zettelkasten to be at `~/notes`. You
can also use paths containing environment variables or paths starting with a
tilde (`~`)
- `db_file`: database file `settle` uses
- `template`: path to Zettel template; if empty, or if the path is invalid, then
templates won't be used. You can use paths containing environment variables, or
a leading tilde (`~`).

### Writing Templates

You can store templates anywhere on the filesystem (see above). But what do
templates actually contain?

Most text in a template is interpreted literally. Except for the following
placeholders, which will be replaced accordingly:

- `${TITLE}`: is replaced with the title of the Zettel upon creation

Thus, a template may look like:

```md
# ${TITLE}



### References


```

## Commands

Under the hood, it uses an SQLite database to keep track of note metadata: the
title and tags of the Zettel and the files it links to. Thus, it's fairly fast
for most operations. Make sure to keep it up to date; all commands rely on the
database!

- `help`: print a help message
- `generate`: create and populate the database with Zettel metadata
- `ls`: list existing files in Zettelkasten, based on database info
- `new`: create a new Zettel and add its metadata to the database, but don't
overwrite; if the file exists and the metadata entry also exists, abort
- `update`: update the metadata for a given path. If the path isn't a file or
doesn't exist, print an error message.
- `query`: return existing Zettel matching the pattern provided as argument
- `not-created`: return a list of Zettel that have links pointing to them, but
haven't been created
- `list-tags`: list all unique tags used in Zettelkasten
- `find`: search for Zettel that have the specified tag
- `links`: print the Zettel that match the query provided and the forward links
they contain.
- `backlinks`: return a list of Zettel that reference the note specified as an
argument
- `search`: return a list of Zettel that contain the specified text
- `zettelkasten`: the absolute path to the directory `settle` uses

### Patterns

Matching literal text gets boring, quick. Fortunately, `settle` supports two
wildcards that'll come in very handy:

- `*`, which matches zero or more characters
- `.`, which matches a single character

If you want a literal `*`, or a literal `.`, you'll need to escape the
character, i.e. `\*` or `\.`. If you want a literal backslash, you're also going
to have to escape it, i.e. `\\`. All other text is matched literally.

## Authors

Written and maintained by xylous \<xylous.e@gmail.com\>. You can also [find me on
GitHub](https://github.com/xylous).
