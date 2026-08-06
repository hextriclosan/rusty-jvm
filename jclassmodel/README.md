# Runtime-friendly view over parsed Java class files

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![MIT Licensed][license-mit-image]][license-mit-link]

## Introduction

The [jclassfile][jclassfile] crate parses a Java class file into a structure that mirrors the
[Java Virtual Machine Specification (JVMS), Chapter §4][jvms-4] exactly: constant pool entries hold
raw `u16` indexes into each other, attributes are an untyped list, and names are only reachable by
following an index to a `Utf8` entry.

That faithfulness is the right shape for a parser, but not for a consumer. Anything that actually
*uses* a class file has to do the same postprocessing first: resolve name and descriptor indexes
into strings, key methods by name and descriptor, group attributes by kind, and turn the `Code`
attribute into something with resolved exception handlers and line numbers.

`jclassmodel` does that work once, on top of `jclassfile`, and hands back a view meant for use
rather than for inspection.

## Usage

Parse the bytes, turn them into a model, and read it. No constant-pool index appears after the
first line.

```rust
use jclassmodel::{parse, MethodModifier};

let bytecode = std::fs::read("Example.class")?;
let model = parse(&bytecode)?.into_model_with_recorded_name()?;

println!("{} (class file version {})", model.external_name, model.version);
if let Some(super_class_name) = &model.super_class_name {
    println!("  extends {super_class_name}");
}
for interface in &model.interfaces {
    println!("  implements {interface}");
}

for field in &model.fields {
    println!("  field {}: {}", field.name, field.descriptor);
}

for method in &model.methods {
    println!("  method {} returning {}", method.name, method.descriptor.return_type());

    if method.modifiers.contains(MethodModifier::Static) {
        println!("    static");
    }
    for exception in &method.exceptions {
        println!("    throws {exception}");
    }
    if let Some(code) = &method.code {
        println!("    {} bytes of bytecode", code.bytecode.len());

        // Line numbers are keyed by bytecode index, so a stack trace is a lookup away.
        if let Some((_, line)) = code.line_numbers.iter().next() {
            println!("    starts at source line {line}");
        }
    }
}
```

### Which name is the class loaded under?

`into_model_with_recorded_name` names the class the way its own constant pool does, and derives the
external name by replacing `/` with `.`. That is correct for any class read from a `.class` file,
so it is the entry point to reach for.

The lower-level `ParsedClass::into_model(internal_name, external_name)` exists because a JVM does
not always load a class under the name recorded in it. A hidden class
(`Lookup.defineHiddenClass`, and therefore every lambda) is defined from a class file that records
`my/package/MyClass` while the VM loads it as `my/package/MyClass/0xABCDEF`; the suffix is minted
at definition time and appears nowhere in the file. Such a name is also not a plain `/`-to-`.`
replacement, since the final separator stays a slash: `my.package.MyClass/0xABCDEF`. The same
applies to classes a VM invents outright, such as primitives (`I` is externally `int`) and arrays.

If that is you, read the recorded name with `ParsedClass::this_class_name()`, apply your own naming
policy, and pass both names in. Otherwise ignore this entirely.

## Relationship to `jclassfile`

| | `jclassfile` | `jclassmodel` |
| --- | --- | --- |
| Shape | 1:1 with JVMS §4 | organized for lookup |
| Constant pool | `Vec` of raw entries, indexes unresolved | typed accessors returning resolved values |
| Attributes | flat `Vec<Attribute>` | grouped by kind |
| Cross-references | `u16` indexes | resolved names and descriptors |

The dividing line is whether a step requires an interpretation choice. Decoding the byte layout
belongs to the parser; deciding how a consumer wants the result indexed belongs here.

`jclassfile` is a private implementation detail: it does not appear anywhere in this crate's public
API, so depending on `jclassmodel` does not tie you to a particular `jclassfile` version.

## Relationship to `jdescriptor`

[jdescriptor][jdescriptor] is treated the other way round, on purpose. `MethodModel::descriptor` and
`FieldModel::descriptor` hand back its `MethodDescriptor` and `TypeDescriptor`, because a parsed
descriptor is vocabulary a consumer needs to pass around, not a detail worth hiding. That makes
`jdescriptor` a *public* dependency: its version is part of this crate's semver contract.

Rust type identity is per compiled copy of a crate, so a build that resolves a second,
semver-incompatible `jdescriptor` gets a `MethodDescriptor` that is a distinct type from this one,
and the two cannot be exchanged. To keep that from happening, `jdescriptor` is re-exported. Reach
for `jclassmodel::MethodDescriptor`, or the whole crate at `jclassmodel::jdescriptor`, instead of
declaring your own dependency on it.

## Type mapping

### Top-level structures

| `jclassfile` | `jclassmodel` | What the conversion does |
| --- | --- | --- |
| `ClassFile` | `ClassModel` | `this_class`/`super_class` indexes become `name`/`super_class_name: Option<String>`; `interfaces: Vec<u16>` becomes `Vec<String>` |
| `major_version`/`minor_version` | `ClassFileVersion` | Two loose `u16`s become one ordered value with `is_preview()`; also readable from `ParsedClass::version()` before resolving the class |
| `Vec<ConstantPool>` | `ConstantPool` | A flat vector of entries becomes a lookup: `get_utf8`, `get_class_name`, `get_full_method_info`, `get_method_handle`, … each resolving the indexes an entry points at |
| `ConstantPool` (one entry) | `ConstantTag` | The entry's *kind*, without its payload, for callers that only need the tag byte |
| `Vec<Attribute>` | `Attributes` | An untyped list becomes a map keyed by attribute kind |
| `MethodInfo` | `MethodModel` | `name_index`/`descriptor_index` become `name: String` and `descriptor: MethodDescriptor` (parsed via [jdescriptor][jdescriptor]), plus `name_signature` (`"name:descriptor"`) |
| `FieldInfo` | `FieldModel` | `name_index`/`descriptor_index` become `name: String` and `descriptor: TypeDescriptor` |
| `jclassfile::error::Error` | `jclassmodel::Error` | A coarse `ErrorKind` (`ClassFile`, `ConstantPool`, `Descriptor`, `Structure`); the parser error stays reachable via `Error::source` |

### Attributes and the records inside them

| `jclassfile` | `jclassmodel` | What the conversion does |
| --- | --- | --- |
| `Attribute::Code` | `CodeModel` | Keeps `max_stack`/`max_locals`/bytecode, and resolves the tables nested inside it |
| `ExceptionRecord` | `ExceptionTableRecord` | `catch_type: u16` becomes a `CatchType`: `Class(name)`, or `Any` for the `0` catch-all a `finally` compiles to |
| `LineNumberRecord` | `line_numbers: BTreeMap<u16, u16>` | A sequence of records becomes a lookup from bytecode index to source line |
| `LocalVariableTableRecord` | `LocalVariableInfo` | `name_index` becomes `name: String`; `index` is renamed `slot` |
| `BootstrapMethodRecord` | `BootstrapMethodInfo` | `bootstrap_method_ref` is resolved into `ref_kind` + class, method name and descriptor, joined with the `invokedynamic` name and descriptor |
| `Attribute::InnerClasses` | `ClassModel::declaring_class` + `nested_class_modifiers` | Finds the record describing *this* class, resolves its outer class name and keeps its `inner_class_access_flags` |
| `Attribute::MethodParameters` | `MethodModel::parameters` | `name_index` becomes `name: Option<String>` (zero means unnamed), flags become `ParameterModifier` |
| `Attribute::EnclosingMethod` | `EnclosingMethodInfo` | Two indexes become an enclosing class name plus an `Option` method name and descriptor, since JVMS §4.7.7 allows `method_index` to be zero |
| `Attribute::Exceptions` | `MethodModel::exceptions` | `exception_index_table: Vec<u16>` becomes `Vec<String>` of thrown class names, in declaration order |
| `Attribute::SourceFile` | `ClassModel::source_file` | Index becomes `Option<String>` |
| `Attribute::RuntimeVisibleAnnotations` | `runtime_visible_annotations: HashSet<String>` | Annotation type indexes become type names; the raw bytes stay available as `annotations_raw` for reflection |
| `ClassFlags` | `ClassModifier` | Same bits, owned by this crate so `jclassfile` stays private |
| `MethodFlags` | `MethodModifier` | Same bits, plus `MethodModel::is_polymorphic_signature()` for the `MethodHandle` case |
| `FieldFlags` | `FieldModifier` | Same bits, plus `FieldModel::is_static()` for the common case |
| `NestedClassFlags` | `NestedClassModifier` | Same bits; this is where `private`/`protected`/`static` live for a nested class, since `ClassModifier` cannot express them |
| `MethodParameterFlags` | `ParameterModifier` | Same bits (`Final`, `Synthetic`, `Mandated`) |

### Not surfaced

Because `jclassfile` is a private dependency, this crate cannot hand you a raw `Attribute` as an
escape hatch: an attribute is reachable only once it has been modelled here. Anything below is
parsed by `jclassfile` and held in `Attributes`, but has no accessor yet. Each is a small, additive
change when a consumer needs it, so treat this as a roadmap rather than a limitation.

Needed by a JVM eventually, roughly in that order:

| Attribute | What it unblocks |
| --- | --- |
| `ConstantValue` | Static final initializers. JVMS §5.4.2 has the JVM set these during preparation and javac emits no `<clinit>` store for them, so today they are visible only because javac inlines compile-time constants at their use sites; a reflective field read would see the default value |
| `NestHost`, `NestMembers` | Nestmate access control for private members (Java 11+) |
| `Record` | Reflection over record components |
| `PermittedSubclasses` | Sealed class checks |
| `Signature` | Generic type information for reflection |

Deliberately out of scope for now: `StackMapTable`. It exists for the bytecode verifier, so it is
worth modelling the day there is one, and `jclassfile` already parses it into `Vec<StackMapFrame>`
for that day. The same goes for `SourceDebugExtension`, `LocalVariableTypeTable`, the
`RuntimeInvisible*` and type-annotation variants, and the `Module*` family.

`magic` is never surfaced: it is `0xCAFEBABE` or parsing already failed, so it carries no
information.

## Status

Early development. This crate is extracted from the [rusty-jvm][rusty-jvm] project, where this
postprocessing layer has been exercised against the whole OpenJDK standard library.

## Contributing
Contributions are welcome! Feel free to open an issue or submit a pull request.

## License
This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.

[//]: # (links)
[crate-image]: https://img.shields.io/crates/v/jclassmodel.svg
[crate-link]: https://crates.io/crates/jclassmodel
[docs-image]: https://docs.rs/jclassmodel/badge.svg
[docs-link]: https://docs.rs/jclassmodel
[license-mit-image]: https://img.shields.io/badge/license-MIT-blue.svg
[license-mit-link]: LICENSE
[jclassfile]: https://crates.io/crates/jclassfile
[jdescriptor]: https://crates.io/crates/jdescriptor
[rusty-jvm]: https://github.com/hextriclosan/rusty-jvm
[jvms-4]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html
