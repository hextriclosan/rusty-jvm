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
 - supports the internal versions 1.0 (JDK 9-26) and 1.1 (JDK 27+) of the jimage file format (see [jimage Versions](#jimage-versions))

## jimage Versions
A jimage file records its format version in the second 32-bit word of its header: the major version in the upper 16 bits
and the minor version in the lower 16 bits. The JDK reader only accepts the exact version it was built for, so the
version of `lib/modules` tells which JDK releases can read it.

| Version | JDK releases | Changes                                                                                          |
|---------|--------------|--------------------------------------------------------------------------------------------------|
| 1.0     | 9 - 26       | Original format.                                                                                 |
| 1.1     | 27+          | Preview mode support: a new location attribute and preview versions of resources (see below).    |

Version 1.1 keeps the layout of version 1.0 and adds:
 - the `PREVIEW_FLAGS` location attribute (kind `8`), a bit set describing how a location relates to preview mode:

   | Flag                  | Value | Meaning                                                                         |
   |-----------------------|-------|---------------------------------------------------------------------------------|
   | `HAS_PREVIEW_VERSION` | `0x1` | A regular location that also has a preview version.                             |
   | `IS_PREVIEW_VERSION`  | `0x2` | A location in the `/<module>/META-INF/preview/...` namespace.                   |
   | `IS_PREVIEW_ONLY`     | `0x4` | A location that exists only because of preview resources.                       |

 - the `/<module>/META-INF/preview/` namespace, which holds the preview versions of resources.

This crate reads both versions. It parses the `PREVIEW_FLAGS` attribute but does not implement preview mode:
 - lookups resolve the regular, non-preview resources under their usual names, and preview versions are never
   substituted for them
 - preview versions are ordinary entries: they are listed, and can be looked up, under their full
   `/<module>/META-INF/preview/...` names (unlike the JDK reader, which hides that namespace)
 - preview-only resources exist only under `/<module>/META-INF/preview/...`, not under their regular names

Files of any other version are rejected with `JImageError::Version`.

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
