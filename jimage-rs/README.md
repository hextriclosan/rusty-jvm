jimage-rs
===
A fast and efficient Rust library for working with `jimage` files used by the Java Platform Module System.

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![License][license-mit-image]][license-mit-link]

## Introduction
`jimage-rs` is a Rust library and command-line utility for reading and extracting resources from Java image files (`jimage`).

`jimage` is a file format used by the Java Virtual Machine (JVM) to store class files and other resources in a compressed format. It is typically found in the `lib/modules` directory of a Java installation.
The format was developed as part of [Project Jigsaw][project-jigsaw] ([JEP-220][jep-220]) and is used in Java Platform Module System (JPMS).

This crate: 
 - is implemented from scratch
 - based on publicly available information (since there is no official jimage file format specification)
 - supports the latest internal version 1.0 of the jimage file format

## Library
Sample code of extracting resources from a jimage file:

```rust
use std::env;
use std::path::PathBuf;
use jimage_rs::JImage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(env::var("JAVA_HOME")?)
        .join("lib")
        .join("modules");
    let jimage = JImage::open(path)?;

    let resource_count = jimage.resource_names_iter().count();
    println!("Total resources in jimage: {}", resource_count);

    match jimage.find_resource("/java.base/java/lang/String.class")? {
        Some(resource) => println!("Resource found, its size is {} bytes", resource.len()),
        None => println!("Resource not found"),
    }

    Ok(())
}
```

### CLI
`jimage-rs` command-line utility can be used to:
- extract resources from a jimage file `jimage-rs extract -r /java.base/java/lang/String.class $JAVA_HOME/lib/modules`
- list all resources in a jimage file `jimage-rs list $JAVA_HOME/lib/modules`

## License

This project is licensed under the [MIT license][license-mit-link].

[//]: # (links)
[crate-image]: https://img.shields.io/crates/v/jimage-rs.svg
[crate-link]: https://crates.io/crates/jimage-rs
[docs-image]: https://docs.rs/jimage-rs/badge.svg
[docs-link]: https://docs.rs/jimage-rs
[license-mit-image]: https://img.shields.io/badge/license-MIT-blue.svg
[license-mit-link]: LICENSE
[project-jigsaw]: https://openjdk.org/projects/jigsaw/
[jep-220]: https://openjdk.org/jeps/220
