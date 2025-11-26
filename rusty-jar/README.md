rusty-jar
===
A simple Rust library for reading JAR files.

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![License][license-mit-image]][license-mit-link]

## Introduction
`rusty-jar` is a Rust library and command-line utility for reading and extracting resources from JAR (Java ARchive) files.

### CLI
`rusty-jar` command-line utility can be used to:
- extract resources from a JAR file `rusty-jar extract -r META-INF/MANIFEST.MF sample.jar`
- list all resources in a JAR file `rusty-jar list sample.jar`

## License
This project is licensed under the [MIT license][license-mit-link].

[//]: # (links)
[crate-image]: https://img.shields.io/crates/v/rusty-jar.svg
[crate-link]: https://crates.io/crates/rusty-jar
[docs-image]: https://docs.rs/rusty-jar/badge.svg
[docs-link]: https://docs.rs/rusty-jar
[license-mit-image]: https://img.shields.io/badge/license-MIT-blue.svg
[license-mit-link]: LICENSE
