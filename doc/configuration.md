# Configuration

The configuration file is at either `$XDG_CONFIG_HOME/settle/settle.yaml`, if
`$XDG_CONFIG_HOME` is set, or `~/.config/settle/settle.yaml`, by default,
and is written in YAML Format.

Note that paths specified in configuration may contain environment variables, or
a leading tilde.

Here are the configuration options:

- `zettelkasten` - path to the directory in which the notes are stored at

    If you don't specify an absolute path, e.g. `notes`, it's assumed you want
    your Zettelkasten to be at `~/notes`.

- `template` - path to the [template file for new Zettel](./templates.md)

    If empty, or if the path is invalid, then templates won't be used.

### Example configuration file

The configuration file is automatically created when `settle` is ran, if it
doesn't already exist. They can be as simple as:

```YAML
zettelkasten: ~/docs/zettelkasten
template: ~/.config/settle/template.md
```
