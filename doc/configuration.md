# Configuration

The configuration options are passed in YAML format. The location of the
configuration file may be influenced by environment variables:

1. if `$SETTLE_CONFIG` is set: `$SETTLE_CONFIG`
2. if `$XDG_CONFIG_HOME` is set: `$XDG_CONFIG_HOME/settle/settle.yaml`
3. default: `$HOME/.config/settle/settle.yaml`

A generic configuration file is automatically created when `settle` is ran with
any command (except `compl`), if it doesn't already exist.

### Configuration option

NOTE: the paths specified in configuration may contain environment variables, or
a leading tilde.

- `zettelkasten` - path to the directory in which the notes are stored at

    If you don't specify an absolute path, e.g. `notes`, it's assumed you want
    your Zettelkasten to be at `~/notes`.

- `template` - path to the [template file for new Zettel](./templates.md)

    If empty, or if the path is invalid, then templates won't be used.

### Example configuration file

```YAML
zettelkasten: ~/docs/zettelkasten
template: ~/.config/settle/template.md
```
