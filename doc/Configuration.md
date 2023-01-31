# Configuration

The configuration file is at either `$XDG_CONFIG_HOME/settle/settle.yaml`, if
`$XDG_CONFIG_HOME` is set, either `~/.config/settle/settle.yaml`, by default.

Paths specified in configuration may contain environment variables, or a leading
tilde.

- `zettelkasten` - path to the directory in which the notes are stored at

    If you don't specify an absolute path, e.g. `notes`, it's assumed you want
    your Zettelkasten to be at `~/notes`.

- `template` - path to the template for new Zettel

    If empty, or if the path is invalid, then templates won't be used.
