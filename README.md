# fesh - the Fe (ferrum) shell
`fesh` is a simple shell implemented in Rust. This project was created as part of a university course and aims to help deepen my understanding of both shells and the Rust programming language 🤓

## Run the unit tests
```shell
$ cargo test

# or use nextest for a better ui
$ cargo install cargo-nextest --locked
$ cargo nextest run
```

## How to run the test scripts (wip)
To run the test scripts, you need to add `fesh` to your PATH. You can do this temporarily using the following command:
```shell
$ (PATH=$PWD/target/debug:$PATH; ./test-files/echo.fesh)
```
