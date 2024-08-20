# Report

## Installation for development
### Environment

1. Install the Rust toolchain via `rustup` on [rustup.rs](https://rustup.rs/)
1. Install xmake on [xmake.io](https://xmake.io/#/guide/installation)
1. Clone the repository `git clone git@github.com:plx-pdg/plx.git`

### Build
```sh 
cargo build
# or in release mode
cargo build --release
```

You can find the result binary in `target/debug/plx` and `target/release/plx`.

### Run
To run the binary without knowning its path just run `cargo run`.

## Production usage
We do not provide binaries for the project so you have to compile it yourself but it is easy via cargo.

1. Install the Rust toolchain via `rustup` on [rustup.rs](https://rustup.rs/)
1. Install xmake on [xmake.io](https://xmake.io/#/guide/installation)

To install PLX without cloning the repository, you can install and compile it from `crates.io` with

```sh
cargo install plx
```

And then just run the `plx` command in your terminal. (If it is not found make sure you restart your terminal or check if `~/.cargo/bin/` is in your $PATH).

## Project description
TODO: functionnal and non fonctionnal goals

> **P**ractice programming exos in a delightful **L**earning e**X**perience

## Architecture
TODO: small schema
TODO: small description


## CI/CD strategy
1. On each PR (and when new commits arrive) and on push on main, `cargo build` and `cargo test` are run to make sure everything is working
1. On each git tag, we will run a CI job to test, build and run `cargo publish` to release PLX on [crates.io](https://crates.io/crates/plx)

todo: document release process
todo: document other OS

## Mockups
todo: to import

## Landing page
todo

## Technical choices
todo: why rust
todo: why ratatui ?
todo: why a TUI
todo: why xmake
todo: exos structure and files

## Project management
todo

