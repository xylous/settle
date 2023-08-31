# Renaming notes

Renaming a note through `settle` comes with the benefit that *all links to the
former title are updated*, which means that links (almost) never break.

You can rename a note by using the `settle sync --rename <FROM> <TO>`. Titles
here are matched exactly, as using regex could match multiple notes.

Here are two examples:

- `settle sync --rename "PSYCHOLOGY" "Psychology"` renames "PSYCHOLOGY" to
    "Psychology"
- `settle sync --rename "Neuroology" "Neurology"` renames the note called
    "Neuroology" to "Neurology"

### Errors

- if the specified old title doesn't exist, then an error is returned
- if there is already a note with the new title provided, then an error is
    returned and the overwrite is prevented.
