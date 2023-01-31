# Templates

Template files are used solely when creating new Zettel. They contain text that
gets put inside a new note, plus some placeholders that get replaced with
certain data.

### Placeholders

- `${TITLE}` - replaced with the title of the note
- `${DATE}` - replaced with the output of `date +%Y-%m-%d`

### Example template

```md
# ${TITLE}



### References

- ${DATE}: ?
```
