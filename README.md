# Speak to Vim

An experiment in hands-free editing, aiming to make the craft of software engineering more accessible.

## Requirements

* `wget`
* [Rust](https://rust-lang.org) version `1.41.1` (or greater)
* GNU Make

## Building

Unfortunately, project requirements make running `cargo` standalone a bit more complex than usual. Instead, you'll likely use the provided [`Makefile`](Makefile) which presents the following commands:

* `make`: clean dependency and build directories before downloading the DeepSpeech native client for a given operating system and architecture
* `make clean`: remove temporary dependencies and build artifacts
* `make build`: sets the environment variable `LIBRARY_PATH` to point at required dependencies before calling `cargo build`

## Why Vim?

At the core, Vim's modes and key commands are essentially a grammar for communicating with your editor.

## License

Copyright 2020 Nicholas Young. Licensed under the [Mozilla Public License, version 2.0](LICENSE).
