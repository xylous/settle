# Creating notes

Most of your writing revolves around creating new notes. Granted, that's a
simple thing to do:

- `settle sync --create 'This is an interesting note'` would create a note with
    'This is an interesting note' as title, within the main Zettelkasten.
- `settle sync --create 'My second note' --project inbox` would create a note
    with 'My second note' as title, but in the 'inbox' project.

If you specify a [project](./projects.md) that doesn't exist yet, then it's
automatically created.

However, note that this operation may fail if a note with the same title in the
Zettelkasten already exists; duplicate titles are forbidden.

### Templates

If you have a [template](./templates.md) file set (see:
[configuration](./configuration.md)), then its contents are going to be used for
the newly created note. If you don't, an empty file is created. In either case,
you'll have to edit the file on your own.
