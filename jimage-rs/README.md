# jimage-rs
===
A fast and efficient Rust library for dealing with Java image files.

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![License][license-mit-image][license-mit-link]

## Introduction
`jimage-rs` is a Rust library and command-line utility for reading and extracting resources from Java image files (`jimage`).

`jimage` is a file format uses by the Java Virtual Machine (JVM) to store class files and other resources in a compressed format. It is typically found in the `lib/modules` directory of a Java installation.
The file format developed in the scope of [Project Jigsaw][project-jigsaw] ([JEP-220][jep-220]) is used to store the Java Platform Module System (JPMS) modules.

This crate: 
 - is implemented from scratch
 - based on open information (since there is no official `jimage`-format specification) 
 - supports the latest internal version 1.0

## Library
Sample code of extracting resources from a jimage file:

```rust
let path = format!("{}/lib/modules", env::var("JAVA_HOME").unwrap());
let jimage = JImage::open(path).unwrap();
let resource = jimage.find_resource("/java.base/java/lang/String.class");
```

### CLI
`jimage-rs` command-line utility can be used to extract resources from a jimage file:
```shell
jimage-rs extract -r /java.base/java/lang/String.class $JAVA_HOME/lib/modules
```

[//]: # (links)
[crate-image]: https://img.shields.io/crates/v/jimage-rs.svg
[crate-link]: https://crates.io/crates/jimage-rs
[docs-image]: https://docs.rs/jimage-rs/badge.svg
[docs-link]: https://docs.rs/jimage-rs
[license-mit-image]: https://img.shields.io/badge/license-MIT-blue.svg
[license-mit-link]: LICENSE
[project-jigsaw]: https://openjdk.org/projects/jigsaw/
[jep-220]: https://openjdk.org/jeps/220
