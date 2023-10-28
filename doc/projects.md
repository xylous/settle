# Projects

`settle` understands your Zettelkasten in terms of *projects*. A project is any
directory that contains at least one note, i.e. at least one Markdown file.

The catch is that only the root of your Zettelkasten directory and only the
*direct* subdirectories it contains can count as projects. That is to say, if
the root of your Zettelkasten is at `~/docs/zettelkasten`, then
`~/docs/zettelkasten/myproject` can be a project, but
`~/docs/zettelkasten/myproject/mysubproject` cannot. Likewise, since `~/docs`
isn't a subdirectory of your Zettelkasten's root, it can't count as a project.

Note, however, the root of the Zettelkasten can be referenced by two names:
`"main"`, or `""` (empty string)

### The role of projects

Projects provide only a formal separation between notes, since any note can
reference any other note, regardless of where they are.

It's up to you how you use projects. General Zettelkasten guidelines indicate
using as few hierarchical structures as possible. They are most practical when
you want to separate notes that shouldn't mix together: you may create a
`writings` project to hold your publishings, or a `literature` project to hold
notes on what you read or plan to read.
