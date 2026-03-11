# JNI Function Name Generator for Rust

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![MIT Licensed][license-mit-image]][license-mit-link]

## Introduction 

A library for generating C-style JNI function name from Java method descriptor according to [specs][jni].

## Usage

```rust
use jniname::{c_name, Error};
let result = c_name("Test", "foo", "(I)I");
assert_eq!(result, Ok(("Java_Test_foo".to_string(), "Java_Test_foo__I".to_string())));
```

## License
This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.

[//]: # (links)
[crate-image]: https://img.shields.io/crates/v/jniname.svg
[crate-link]: https://crates.io/crates/jniname
[docs-image]: https://docs.rs/jniname/badge.svg
[docs-link]: https://docs.rs/jniname
[jni]: https://docs.oracle.com/en/java/javase/25/docs/specs/jni/design.html#resolving-native-method-names
[license-mit-image]: https://img.shields.io/badge/license-MIT-blue.svg
[license-mit-link]: LICENSE
