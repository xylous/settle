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

## Configurability

The config file is located at `~/.config/settle/settle.yaml`. When you first
run the program, it's going to be generated automatically with default values.

You can specify the following options:
- `zettelkasten`: path to Zettelkasten. If you don't specify an absolute path,
e.g. `notes`, it's assumed you want your Zettelkasten to be at `~/notes`. You
can also use paths with environment variables or paths starting with tilde (`~`)
- `db_file`: the path relative to your Zettelkasten directory the program is
going to use for storing metadata
- `template`: path to Zettel template; if empty, or if the path is invalid, then
templates won't be used. You can use paths with environment variables, or tilde
(`~`).

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
- `new`: create a new Zettel and add its metadata to the database
- `edit`: edit a currently existing Zettel and update database entry for it
afterwards
- `not-created`: return a list of Zettel that have links pointing to them, but
haven't been created
- `list-tags`: list all unique tags used in Zettelkasten
- `find`: search for Zettel that have the specified tag
- `backlinks`: return a list of Zettel that reference the note specified as an
argument
- `search`: return a list of Zettel that contain the specified text

### Patterns

Matching literal text is tedious. Fortunately, `settle` supports two wildcards
that'll come in very handy:

- `*`, which matches zero or more characters
- `.`, which matches a single character

If you want a literal `*`, or a literal `.`, you'll need to escape the
character, i.e. `\*` or `\.`. If you want a literal backslash, you're also going
to have to escape it, i.e. `\\`. All other text is matched literally.

## Authors

Written and maintained by xylous \<xylous.e@gmail.com\>. You can also [find me on
GitHub](https://github.com/xylous).
