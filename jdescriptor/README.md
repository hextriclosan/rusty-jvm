# Java Bytecode Descriptor Parser for Rust

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![MIT Licensed][license-mit-image]][license-mit-link]

## Introduction

Effortlessly parse and manipulate Java bytecode descriptors in Rust. 
This crate provides a **lightweight, efficient, and easy-to-use** solution for working with Java class, field, and method descriptors.

## Why Use This Crate?
✅ **Parse Java Method Signatures** – Extract return types, parameters, and more from JVM bytecode descriptors.<br>
✅ **Ultra-Lightweight** – Zero dependencies and minimal runtime overhead.<br>
✅ **Rust & Java Interoperability** – Perfect for Rust tools interacting with Java class files.<br>
✅ **Beyond Classfiles** – Useful for RPC, static analysis, IDE plugins, serialization, and more!<br>
✅ **Optimized for Performance** – Designed to handle large-scale descriptor processing efficiently.

For more details about Java bytecode descriptors, check out the relevant sections in the [Java Virtual Machine Specification (JVMS)][jvms-index]:
- **Field Descriptors**: [Section 4.3.2][jvms-4.3.2]
- **Method Descriptors**: [Section 4.3.3][jvms-4.3.3]


## Usage

### Dealing with Type Descriptors
```rust
let parsed: TypeDescriptor = "[[Ljava/lang/String;".parse().unwrap();
println!("Rust representation: {:?}", parsed); // Array(Object("java/lang/String"), 2)
println!("Java representation: {}", parsed); // [[Ljava/lang/String;
```

### Dealing with Method Descriptors
```rust
let parsed: MethodDescriptor = "(FIB)S".parse().unwrap();
println!("Rust structure: {:?}", parsed); // MethodDescriptor { parameter_types: [Float, Integer, Byte], return_type: Short }
println!("Java representation: {}", parsed); // (FIB)S
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
[jvms-index]: https://docs.oracle.com/javase/specs/jvms/se23/html/
[jvms-4.3.2]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.3.2
[jvms-4.3.3]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.3.3
