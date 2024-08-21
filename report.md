# Report

## Installation for development
### Environment

1. Install the Rust toolchain via `rustup` on [rustup.rs](https://rustup.rs/)
1. Install xmake on [xmake.io](https://xmake.io/#/guide/installation)
1. Clone the repository `git clone git@github.com:plx-pdg/plx.git`
1. We need to define how exos files are edited, for this we choose the standard `$EDITOR` environment variable
  1. You can choose any command line editor like `nano, vim, nvim` or others, but GUI IDE also works `code, codium, idea`, as a replacement of `<ide>` below.
  1. On Mac and Linux you should change your shell configuration (`~/.bashrc` for ex.) with a line like `export EDITOR=<ide>`
  1. On Windows `setx /m EDITOR <ide>` (check it worked you can run `echo %EDITOR%` in a new terminal)
  1. In case it doesn't work, make sure to reload your shell
  1. When you enter an exo your $EDITOR will automatically open the correct file

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

1. We protect the main branch on the main repository to avoid pushing commits directly without any review. The 2 others repository (website + organisation profile) are not protected for ease of change.
1. For each feature or change:
  1. we create a new issue and assign it to the correct person
  1. create a new branch,
  1. try to follow the conventionnal commits standard for writing commit messages,
  1. when done we send a PR.
  1. The PR is automatically merged only after one review, and trivial changes that do not review can be merged by the PR creator.
  1. Github is configured to block merging if CI jobs are failing.
  1. We try to delete the branch when PR is merged.
1. We do 2 small coordination meetings starting between 9:30 and 10:00, and another one around 15:00.


