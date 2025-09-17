# Java class files parser for Rust

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![MIT Licensed][license-mit-image]][license-mit-link]

## Introduction

This crate reads and parses Java class files, following the [Java Virtual Machine Specification (JVMS), Chapter §4][jvms-4].
While there are other crates with similar functionality, the `jclassfile` crate stands out for its extensive testing within the [rusty-jvm][rusty-jvm] project.
It has been used to parse thousands of OpenJDK standard library classes and is also exercised through numerous integration tests within the project.

## Implementation Status
The following sections of the Java Virtual Machine Specification (JVMS, Java SE 25 Edition) are implemented:

- [x] The ClassFile Structure ([§4.1][jvms-4.1])
- [x] The Constant Pool ([§4.4][jvms-4.4])
  - [x] The CONSTANT_Class_info Structure ([§4.4.1][jvms-4.4.1])
  - [x] The CONSTANT_Fieldref_info, CONSTANT_Methodref_info, and CONSTANT_InterfaceMethodref_info Structures ([§4.4.2][jvms-4.4.2])
  - [x] The CONSTANT_String_info Structure ([§4.4.3][jvms-4.4.3])
  - [x] The CONSTANT_Integer_info and CONSTANT_Float_info Structures ([§4.4.4][jvms-4.4.4])
  - [x] The CONSTANT_Long_info and CONSTANT_Double_info Structures ([§4.4.5][jvms-4.4.5])
  - [x] The CONSTANT_NameAndType_info Structure ([§4.4.6][jvms-4.4.6])
  - [x] The CONSTANT_Utf8_info Structure ([§4.4.7][jvms-4.4.7])
  - [x] The CONSTANT_MethodHandle_info Structure ([§4.4.8][jvms-4.4.8])
  - [x] The CONSTANT_MethodType_info Structure ([§4.4.9][jvms-4.4.9])
  - [x] The CONSTANT_Dynamic_info and CONSTANT_InvokeDynamic_info Structures ([§4.4.10][jvms-4.4.10])
  - [x] The CONSTANT_Module_info Structure ([§4.4.11][jvms-4.4.11])
  - [x] The CONSTANT_Package_info Structure ([§4.4.12][jvms-4.4.12])
- [x] Fields ([§4.5][jvms-4.5])
- [x] Methods ([§4.6][jvms-4.6])
- [x] Attributes ([§4.7][jvms-4.7])
  - [x] The ConstantValue Attribute ([§4.7.2][jvms-4.7.2])
  - [x] The Code Attribute ([§4.7.3][jvms-4.7.3])
  - [x] The StackMapTable Attribute ([§4.7.4][jvms-4.7.4])
  - [x] The Exceptions Attribute ([§4.7.5][jvms-4.7.5])
  - [x] The InnerClasses Attribute ([§4.7.6][jvms-4.7.6])
  - [x] The EnclosingMethod Attribute ([§4.7.7][jvms-4.7.7])
  - [x] The Synthetic Attribute ([§4.7.8][jvms-4.7.8])
  - [x] The Signature Attribute ([§4.7.9][jvms-4.7.9])
  - [x] The SourceFile Attribute ([§4.7.10][jvms-4.7.10])
  - [ ] The SourceDebugExtension Attribute ([§4.7.11][jvms-4.7.11])
  - [x] The LineNumberTable Attribute ([§4.7.12][jvms-4.7.12])
  - [x] The LocalVariableTable Attribute ([§4.7.13][jvms-4.7.13])
  - [x] The LocalVariableTypeTable Attribute ([§4.7.14][jvms-4.7.14])
  - [x] The Deprecated Attribute ([§4.7.15][jvms-4.7.15])
  - [x] The RuntimeVisibleAnnotations Attribute ([§4.7.16][jvms-4.7.16])
    - [x] The element_value structure ([§4.7.16.1][jvms-4.7.16.1])
  - [x] The RuntimeInvisibleAnnotations Attribute ([§4.7.17][jvms-4.7.17])
  - [x] The RuntimeVisibleParameterAnnotations Attribute ([§4.7.18][jvms-4.7.18])
  - [ ] The RuntimeInvisibleParameterAnnotations Attribute ([§4.7.19][jvms-4.7.19])
  - [x] The RuntimeVisibleTypeAnnotations Attribute ([§4.7.20][jvms-4.7.20])
    - [x] The target_info union ([§4.7.20.1][jvms-4.7.20.1])
    - [x] The type_path structure ([§4.7.20.2][jvms-4.7.20.2])
  - [ ] The RuntimeInvisibleTypeAnnotations Attribute ([§4.7.21][jvms-4.7.21])
  - [x] The AnnotationDefault Attribute ([§4.7.22][jvms-4.7.22])
  - [x] The BootstrapMethods Attribute ([§4.7.23][jvms-4.7.23])
  - [x] The MethodParameters Attribute ([§4.7.24][jvms-4.7.24])
  - [ ] The Module Attribute ([§4.7.25][jvms-4.7.25])
  - [ ] The ModulePackages Attribute ([§4.7.26][jvms-4.7.26])
  - [ ] The ModuleMainClass Attribute ([§4.7.27][jvms-4.7.27])
  - [x] The NestHost Attribute ([§4.7.28][jvms-4.7.28])
  - [x] The NestMembers Attribute ([§4.7.29][jvms-4.7.29])
  - [x] The Record Attribute ([§4.7.30][jvms-4.7.30])
  - [x] The PermittedSubclasses Attribute ([§4.7.31][jvms-4.7.31])
- [ ] Format Checking ([§4.8][jvms-4.8])
- [ ] Constraints on Java Virtual Machine Code ([§4.9][jvms-4.9])
- [ ] Verification of class Files ([§4.10][jvms-4.10])


## Usage

```rust
pub fn main() {
  let file_content = include_bytes!("HelloWorld.class");
  let class_file = jclassfile::class_file::parse(file_content).unwrap();
  println!("{:?}", class_file);
}
```

## Contributing
Contributions are welcome! Feel free to open an issue or submit a pull request.

## License
This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.

[//]: # (links)
[crate-image]: https://img.shields.io/crates/v/jclassfile.svg
[crate-link]: https://crates.io/crates/jclassfile
[docs-image]: https://docs.rs/jclassfile/badge.svg
[docs-link]: https://docs.rs/jclassfile
[license-mit-image]: https://img.shields.io/badge/license-MIT-blue.svg
[license-mit-link]: LICENSE

[jvms-4]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html
[jvms-4.1]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.1
[jvms-4.4]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4
[jvms-4.4.1]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.1
[jvms-4.4.2]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.2
[jvms-4.4.3]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.3
[jvms-4.4.4]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.4
[jvms-4.4.5]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.5
[jvms-4.4.6]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.6
[jvms-4.4.7]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.7
[jvms-4.4.8]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.8
[jvms-4.4.9]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.9
[jvms-4.4.10]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.10
[jvms-4.4.11]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.11
[jvms-4.4.12]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.4.12
[jvms-4.5]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.5
[jvms-4.6]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.6
[jvms-4.7]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7
[jvms-4.7.2]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.2
[jvms-4.7.3]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.3
[jvms-4.7.4]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.4
[jvms-4.7.5]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.5
[jvms-4.7.6]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.6
[jvms-4.7.7]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.7
[jvms-4.7.8]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.8
[jvms-4.7.9]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.9
[jvms-4.7.10]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.10
[jvms-4.7.11]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.11
[jvms-4.7.12]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.12
[jvms-4.7.13]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.13
[jvms-4.7.14]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.14
[jvms-4.7.15]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.15
[jvms-4.7.16]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.16
[jvms-4.7.16.1]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.16.1
[jvms-4.7.17]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.17
[jvms-4.7.18]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.18
[jvms-4.7.19]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.19
[jvms-4.7.20]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.20
[jvms-4.7.20.1]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.20.1
[jvms-4.7.20.2]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.20.2
[jvms-4.7.21]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.21
[jvms-4.7.22]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.22
[jvms-4.7.23]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.23
[jvms-4.7.24]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.24
[jvms-4.7.25]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.25
[jvms-4.7.26]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.26
[jvms-4.7.27]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.27
[jvms-4.7.28]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.28
[jvms-4.7.29]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.29
[jvms-4.7.30]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.30
[jvms-4.7.31]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.31
[jvms-4.8]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.8
[jvms-4.9]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.9
[jvms-4.10]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.10

[rusty-jvm]: https://github.com/hextriclosan/rusty-jvm
