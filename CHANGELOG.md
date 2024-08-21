# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
Boilerplate to copy paste and adapt at each release

Take this line and fix version with current ($CV)
## [Unreleased](https://github.com/plx-pdg/plx/compare/$CV...HEAD)

### Added
### Changed

## [$CV without v!](https://github.com/plx-pdg/plx/compare/$CV...HEAD) - $DATE
-->


## [Unreleased](https://github.com/plx-pdg/plx/compare/v0.1.0...HEAD)

### Added
### Changed

## [0.1.1](https://github.com/plx-pdg/plx/compare/v0.1.1..HEAD) - 2024-08-21

### Added
- Add CI/CD jobs for build+test+formatting+tag+release (this release is used to test everything is working).
- Do a small change in `main.rs` output to see changes

## [0.1.0](https://github.com/plx-pdg/plx/compare/v0.1.0..HEAD) - 2024-08-19

### Added
- Create an empty Rust crate to reserve the name on [`crates.io`](https://crates.io/crates/plx) with Markdown excluded (it will be reincluded later) when the main README will be rewritten in English and be shorter...
- Define `license-file = "LICENSE"`  in `Cargo.toml` and create a `LICENSE` file with `All rights reserved` mention, just to be able to run `cargo publish`. There is no SPDX license identifier for "proprietary".
- Write first version of README in french with WHY and context details, in addition to the learning experience and the planned features.
