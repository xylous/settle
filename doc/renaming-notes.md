# Renaming notes

Renaming a note through `settle` comes with the benefit that *all links to the
former title are updated*, which means that links (almost) never break.

You can rename a note by using the `settle sync --rename <ARGS>` command;
`<ARGS>` is a list of titles, the minimum amount being two. Titles here are
matched exactly, as using regex could match multiple notes.

The technical behaviour of this command is a bit tough to describe: it checks
through the list of titles incrementally, from left to right, and the first
title that it finds a note for (in the database) is renamed with the title of
the last note. All other titles are discarded.

This seems like weird behaviour. Indeed, it is, but its technical specification
doesn't really concern.

Here are a few examples:

- `settle sync --rename "Foo-bar" "Bar-foo"` renames "Foo-bar" to "Bar-foo", if
    there is a note with the former string as title.

- `settle sync --rename "This" "That" "The other" "My special note"` renames
    the first note it finds with that title to `My special note`. If a note
    called `This` exists, then that's the one matched; if a note called `This`
    doesn't exist but one called `That` exists, then that one is used. And
    finally, if no note called `This` or `That` exist, then `The other` note is
    the one used.

### Errors

- if none of the titles specified exist, then an error is returned
- if there is already a note with the last title provided and in the same
    project, then, to avoid overwriting, nothing is done, and thus an error is
    returned
