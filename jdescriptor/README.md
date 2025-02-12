# Java Bytecode Descriptor Parser for Rust

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![MIT Licensed][license-mit-image]][license-mit-link]

## Introduction

Effortlessly parse and manipulate Java bytecode descriptors in Rust. 
This crate provides a **lightweight, efficient, and easy-to-use** solution for working with Java class, field, and method descriptors.
To use it, add the following lines to your `Cargo.toml` file:

```toml
[dependencies]
jdescriptor = "0.1.0"
```

## Why Use This Crate?
✅ **Parse Java Method Signatures** – Extract return types, parameters, and more from JVM bytecode descriptors.<br>
✅ **Ultra-Lightweight** – Zero dependencies and minimal runtime overhead.<br>
✅ **Rust & Java Interoperability** – Perfect for Rust tools interacting with Java class files.<br>
✅ **Beyond Classfiles** – Useful for RPC, static analysis, IDE plugins, serialization, and more!<br>
✅ **Optimized for Performance** – Designed to handle large-scale descriptor processing efficiently.

## Usage

### Dealing with Type Descriptors
```rust
let parsed: TypeDescriptor = "[[Ljava/lang/String;".parse().unwrap();
println!("Rust representation: {:?}", parsed);
println!("Java representation: {}", parsed);
```

### Dealing with Method Descriptors
```rust
let parsed: MethodDescriptor = "(FIB)S".parse().unwrap();
println!("Rust representation: {:?}", parsed);
println!("Java representation: {}", parsed);
```

## Use Cases
- **Java Classfile Parsing** – Helper for parsing `.class` files and analyze type and method signatures.
- **Interoperability** – Bridge Rust and Java, ensuring type correctness in JNI/FFI calls.
- **Code Generation** – Generate Java bindings, stubs, and DSLs with validated signatures.
- **Static Analysis** – Build security tools that detect unsafe Java method patterns.
- **IDE Plugins & Developer Tools** – Provide type hints and method signature insights.

## Contributing
Contributions are welcome! Feel free to open an issue or submit a pull request.

## License
This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.

[//]: # (links)
[crate-image]: https://img.shields.io/crates/v/jdescriptor.svg
[crate-link]: https://crates.io/crates/jdescriptor
[docs-image]: https://docs.rs/jdescriptor/badge.svg
[docs-link]: https://docs.rs/jdescriptor
[license-mit-image]: https://img.shields.io/badge/license-MIT-blue.svg
[license-mit-link]: LICENSE
