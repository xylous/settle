# Graphs

Since a Zettelkasten is a collection of notes and the links between said notes,
the perfect visual representation is a directed graph.

Graphs help a lot in understanding your Zettelkasten, to give you a quick and
intuitive overview of everything that is going on.

Luckily, `settle` can generate [DOT
(.gv)](https://en.wikipedia.org/wiki/DOT_(graph_description_language)) output of
your Zettelkasten, a language used for describing graphs. You can then use a
dedicated graph renderer, such as `xdot` or `graphviz` (and its many sub-tools)
to view the graph.

So, for example, using `xdot` to explore the graph interactively:

```
$ settle query --graph >zk.gv
$ xdot zk.gv
```

Or, to create a JPG image using `circo` ([on Arch Linux,] comes with the
`graphivz` package):

```
$ settle query --graph >zk.gv
$ circo -Tjpg zk.gv >graph.jpg
```
