# Creating notes

Most of your writing revolves around creating new notes. Granted, that's a
simple thing to do with `settle`.

For example:

- `settle sync --create 'This is an interesting note'` would create a note with
    'This is an interesting note' as title, within the main Zettelkasten.

- `settle sync --create 'My second note' --project inbox` would create a note
    with 'My second note' as title, but in the 'inbox' project.

However, based on certain conditions, this operation may have three outcomes:

- if you try to create a new note but one with the same title in the same
    project already exists, then nothing is changed and an error is returned
    (duplicates are forbidden)

- if you try to create a new note but there is a file (on the filesystem) that
    exists with that title and in the same project as the one specified, then
    the file is not overwritten, and its metadata is added to the database

- if a corresponding file doesn't exist and a database entry for it doesn't
    exist, then indeed, a new note is created

### Templates

If you have a template, then its contents are going to be used for the newly
created note. If you don't, an empty file is created. In either case, you'll
have to edit the file on your own.
