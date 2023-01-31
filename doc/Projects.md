# Projects

### The definition of a project

`settle` understands your Zettelkasten in terms of *projects*. A project is any
directory that contains at least one note, i.e. at least one Markdown file.

The catch is that only the root of your Zettelkasten directory and only the
subdirectories it contains can count as projects. That is to say, if the root of
your Zettelkasten is at `~/docs/zettelkasten`, then
`~/docs/zettelkasten/myproject` can be a project, but
`~/docs/zettelkasten/myproject/mysubproject` cannot. Likewise, since `~/docs`
isn't a subdirectory of your Zettelkasten's root, it can't can't count as a
project.

The reason for this design choice is that, with subprojects like this, it would
become a hierarchical nightmare extremely fast.

### The role of projects

Projects provide only a formal separation between notes, since any note can
reference any other note, regardless of where they are.

Your most basic projects are the root of the Zettelkasten and your inbox - the
former should contain permanent notes, the other should contain temporary notes.

It's up to you how you use your ability to create and manage projects. My advice
would be to use as few as possible. Some projects, such as having, say, a
`literature` project for literature notes, would be useful. Aside from that,
use-cases like writing about game's lore, or writing a book's chapter, or using
projects for anything that isn't and can never be related to the rest of your
Zettelkasten, come to mind.
