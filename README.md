# fesh - the Fe (ferrum) shell
`fesh` is a simple shell implemented in Rust. This project was created as part of a university course and aims to help deepen my understanding of both shells and the Rust programming language 🤓

## Features
- Pipes `|`: `find . | wc -l`
- Redirects `> >>`: `echo hello-world > file.txt`
- History
- Builtins: `exit`, `+debug`, `cd`
- Configuration via toml file, see [config.toml](./config.toml)

## Run the unit tests
```shell
$ cargo test

# or use nextest for a better ui
$ cargo install cargo-nextest --locked
$ cargo nextest run
```
