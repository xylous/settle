# Graphs

Since a Zettelkasten is a collection of notes and the links between said notes,
the perfect representation is a [directed
graph](https://en.wikipedia.org/wiki/Directed_graph). Graphs help a lot in
understanding your Zettelkasten, to give you a quick and intuitive overview of
everything that is going on.

`settle` provides the `--graph` option for the `query` command, which turns the
query results into a format of your choice. There are currently three available
formats, namely:

- [an interactive visualisation for the web-browser](./vizk.md), whose code is
    printed when the `--graph` option is given `vizk` as a value (check the link
    for usage instructions)
- [DOT (.gv)](https://en.wikipedia.org/wiki/DOT_(graph_description_language)),
    obtained by passing `dot` as the value to the `--graph` option. This format
    is used by many graph renderers, such as `xdot` or `graphviz` (and its many
    sub-tools).
- providing `json` as a value to the `--graph` option returns the query results
    as a [JSON object with several properties](#json-format-specification).

## Visualising a DOT graph

You may use, for example, `xdot` to explore the DOT graph interactively:

```
$ settle query --graph dot >zk.gv
$ xdot zk.gv
```

Or, you may create a JPG image using `circo` ([on Arch Linux,] comes with the
`graphivz` package):

```
$ settle query --graph >zk.gv
$ circo -Tjpg zk.gv >graph.jpg
```

## JSON format specification

There are four properties in total:
    - `nodes`: an array containing all the notes' titles
    - `edges`: an array of arrays where the first element [in the sub-array] is
        the index of the source Zettel [in the `nodes` array], the second is the
        index of the target Zettel, and the third is the weight of the link
        (which is always empty string (`""`))
    - `node_holes`: always empty array (`[]`)
    - `edge_property`: always `"directed"`

A minimal example of the format is this:

```json
{
    "nodes": [
        "My first super interesting note",
        "My second, albeit less interesting note",
        "My third note, which is unrelated"
    ],
    "node_holes": [],
    "edge_property": "directed",
    "edges": [
        [
            0,
            1,
            ""
        ]
    ]
}
```

This minimal example describes a Zettelkasten where `My first super interesting
note` (indexed with `0`) links to `My second, albeit less interesting note`
(indexed with `1`), and how `My third note, which is unrelated` has no links to
or from it.
