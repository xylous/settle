# Keeping the database up to date

Since [note editing isn't done with `settle`](./manage-notes-not-editors.md),
the database needs to be updated quite frequently to always have the correct
metadata.

Fortunately, updating the database is fairly straightforward:

- `settle sync --generate` updates every database entry, based on what files are
    on the filesystem. For example, if you delete a note from update to update,
    then its entry won't appear in the new one.

- `settle sync --update <PATH>` takes the path (absolute or relative) to a note
    file, whose corresponding database entry is updated. Keep in mind that if
    the provided path is outside of the Zettelkasten, or it doesn't exist, then
    an error is returned.
