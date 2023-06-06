# Manage notes, not editors

`settle` wasn't designed to work with any particular editor or suite of tools.
Even if most of your interactions with the Zettelkasten are just writing!

This has one main advantage: it's the same Zettelkasten everywhere you go, with
any possible editor you may use. This is to say, `settle` is a potentially
universal backend-like interface to your notes.

Any editor integration is done with editor plugins/wrappers. Since I'm a Neovim
user myself, I wrote [settle.vim](https://github.com/xylous/settle.vim), which
overall improves the writing experience, by doing things like automatically
updating the notes you're working on, by providing autocompletion, and by
creating a few useful commands.
