# Java Bytecode Descriptor Parser for Rust

[![Crates.io](https://img.shields.io/crates/v/jdescriptor.svg)](https://crates.io/crates/jdescriptor)
[![Docs.rs](https://docs.rs/jdescriptor/badge.svg)](https://docs.rs/jdescriptor)
[![License](https://img.shields.io/crates/l/jdescriptor.svg)](https://github.com/hextriclosan/rusty-jvm/blob/main/jdescriptor/LICENSE)

Effortlessly parse and manipulate Java bytecode descriptors in Rust! This crate provides a **lightweight, efficient, and easy-to-use** solution for working with Java class, field, and method descriptors.

## Why Use This Crate?
✅ **Parse Java Method Signatures** – Extract return types, parameters, and more from JVM bytecode descriptors.<br>
✅ **Ultra-Lightweight** – Zero dependencies and minimal runtime overhead.<br>
✅ **Rust & Java Interoperability** – Perfect for Rust tools interacting with Java class files.<br>
✅ **Beyond Classfiles** – Useful for RPC, static analysis, IDE plugins, serialization, and more!<br>
✅ **Optimized for Performance** – Designed to handle large-scale descriptor processing efficiently.

## Installation
To bring this crate into your repository, either add csv to your `Cargo.toml`, or run `cargo add jdescriptor`.

## Usage

### Parsing Java Descriptors
```rust
let descriptor = "(Ljava/lang/String;I)V";
let parsed = descriptor.parse().unwrap();
println!("Parsed: {:?}", parsed);
```

### Extracting Return Type & Parameters
```rust
let method = Descriptor::from("(Ljava/lang/String;I)Ljava/util/List;");
println!("Return type: {:?}", method.return_type);
println!("Parameters: {:?}", method.parameters);
```

## Use Cases
- **Java Classfile Parsing** – Process `.class` files and analyze method signatures.
- **Interoperability** – Bridge Rust and Java, ensuring type correctness in JNI/FFI calls.
- **Code Generation** – Generate Java bindings, stubs, and DSLs with validated signatures.
- **Static Analysis** – Build security tools that detect unsafe Java method patterns.
- **IDE Plugins & Developer Tools** – Provide type hints and method signature insights.

## Contributing
Contributions are welcome! Feel free to open an issue or submit a pull request.

## License
This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.
