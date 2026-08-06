# 0.1.0

Initial release. Extracted from the [rusty-jvm](https://github.com/hextriclosan/rusty-jvm) project,
where this postprocessing layer has been exercised against the whole OpenJDK standard library.

## What's Changed
* `parse` and `ParsedClass` turn class file bytes into a `ClassModel` with every constant-pool
  index resolved, via `into_model_with_recorded_name` for classes named by their own constant pool
  or `into_model` for callers that mint their own names, such as a JVM defining a hidden class.
* `ClassModel`, `MethodModel`, `FieldModel` and `CodeModel` carry resolved names, `jdescriptor`
  descriptors, thrown exceptions, line numbers keyed by bytecode index, and exception handlers.
* `ConstantPool` replaces a flat entry vector with typed lookups (`get_utf8`, `get_class_name`,
  `get_full_method_info`, `get_method_handle`, …), and `Attributes` groups attributes by kind.
* Typed modifiers for all five JVMS flag words: `ClassModifier`, `FieldModifier`,
  `MethodModifier`, `NestedClassModifier` and `ParameterModifier`.
* `ClassFileVersion` pairs `major` and `minor` into one ordered value with `is_preview()`, readable
  from `ParsedClass::version()` before a class is resolved.
* `CatchType` represents a catch-all handler explicitly rather than by a sentinel class name.
* `EnclosingMethodInfo` and `ParameterInfo` model the JVMS zero-index cases as `Option`, so a class
  enclosed by an initializer keeps its enclosing class and an unnamed parameter keeps its
  modifiers.
* Class files are treated as untrusted input: malformed input is reported through `Error` and
  `ErrorKind` rather than panicking.
* `jdescriptor` is re-exported, since it is part of this crate's public API. `jclassfile` is a
  private dependency and appears nowhere in it.
