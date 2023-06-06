# Overview

`settle` was designed with the Zettelkasten method in mind. It works primarily
with Markdown.

However, there are a few differences from most other Zettelkasten apps, and that
stems from the design principles:

- `settle` is [made for humans](./made-for-humans.md), for a pleasant
    note-taking experience
- `settle` only [manages notes, not editors](./manage-notes-not-editors.md):
    settle is a mere note-taking assistant, it doesn't handle file editing
- what you see is exactly what you get: all metadata is stored in the notes
    themselves, and none may be added or removed by using commands, which makes
    the database a mere convenience - all metadata is inferred from the
    filesystem.
- [projects](./projects.md) are only used to formally separate Zettel, but there
    isn't any hard boundary - any note may reference any other note
- [links between Zettel](./links-and-backlinks.md) are wiki-style links, which
    are extremely straightforward.
