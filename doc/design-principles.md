# Overview

`settle` was designed with the Zettelkasten method in mind. It works primarily
with Markdown.

However, there are a few differences from most other Zettelkasten apps, and that
stems from the design principles:

- `settle` is [made for humans](./made-for-humans.md), for a pleasant
    note-taking experience
- `settle` only [manages notes, not editors](./manage-notes-not-editors.md):
    settle is a mere note-taking assistant, it doesn't handle file editing
- [only the minimum metadata necessary is stored](./minimum-metadata.md)
- no extra metadata may be added or removed by using commands, making the
    database a mere convenience - all metadata is inferred from the filesystem
- [projects](./projects.md) are only used to formally separate Zettel, but there
    isn't any hard boundary - any note may reference any other note
- [links between Zettel](./links-and-backlinks.md) are exclusively wiki-style
    links, which are convenient, but you must write the exact title of the note
