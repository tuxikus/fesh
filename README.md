# fesh - the Fe (ferrum) shell
`fesh` is a simple shell implemented in Rust. This project was created as part of a university course and aims to help deepen my understanding of both shells and the Rust programming language 🤓

## Features
- Pipes `|`: `find . | wc -l`
- Redirects `> >>`: `echo hello-world > file.txt`
- History
  - completion
  - search via `i-search`
- Builtins: 
  - `exit`
  - `+debug`: toggle debug mode
  - `cd`
  - `aliases`: list all defined aliases
- Configuration via toml file, see [config.toml](./config.toml)
  - prompt customization
    - color
    - cwd
    - current user
    - current git branch
  - aliases
  - `vi` and `emacs` edit modes
  - custom history file path, defaults to `$XDG_DATA_DIR/fesh/history` (or `$HOME/.local/share/fesh/history`)

# Development

## Run the unit tests
```shell
$ cargo test

# or use nextest for a better ui
$ cargo install cargo-nextest --locked
$ cargo nextest run
```
