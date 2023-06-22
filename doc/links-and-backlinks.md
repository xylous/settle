# Links and backlinks

### Forward links

`settle` can only understand wiki-style links, such as `[[Neurons]]`, or
`[[Language learning]]`, which would redirect to a note called `Neurons` or
`Language leraning`, respectively. Case-in-point, a wiki-style link is any text
between two matching `[[` and `]]`.

Such links may be embedded anywhere inside a note.

### Backlinks

If we're considering some note X, then its forward links are the set of every
note that X links to. Meanwhile, its backlinks are the set of every note that
links to X.

More concretely, let's say we have two notes, note X that links to note Y.
Therefore, X has Y as a forward link, Y has X as a backlink.

Since only the bare minimum of metadata is stored, backlinks aren't actually
kept in the database: they can be deduced, or, rather, computed, from forward
links. Also note that, of course, backlinks aren't stored anywhere within the
notes' contents.
