# Contributing to rusty-jvm

Thank you for your interest in contributing!
This guide explains how to build the project, run the tests, and extend the VM with new features.

---

## Table of Contents

1. [Building the project](#building-the-project)
2. [Running the tests](#running-the-tests)
3. [Adding an integration test](#adding-an-integration-test)
4. [Adding a new native method](#adding-a-new-native-method)
5. [Project structure](#project-structure)
6. [Relevant JVM Specification sections](#relevant-jvm-specification-sections)

---

## Building the project

### Prerequisites

- **Rust** (stable) - install via [rustup](https://rustup.rs/)
- **JDK 25 (LTS)** - must be installed and `JAVA_HOME` must point to it
- **cargo-nextest** (recommended for running tests) - `cargo install cargo-nextest`

```sh
# Clone and build
git clone https://github.com/hextriclosan/rusty-jvm
cd rusty-jvm
cargo build --workspace
```

---

## Running the tests

The test suite is an integration test that compiles a set of small Java programs
and runs them through the VM, comparing stdout with expected output.

```sh
# Run all tests (fastest with nextest)
cargo nextest run --workspace

# Run a specific test by name
cargo nextest run should_print_hello_world

# Run with the standard test runner
cargo test --workspace
```

> **Note:** The integration tests require the JDK to be on `PATH` (for `javac`) and
> `JAVA_HOME` to be set.

---

## Adding an integration test

Each integration test consists of:
1. A Java source file under `tests/test_data/`.
2. A test function in `tests/integration_tests.rs` (or one of the other `integration_tests_*.rs`
   files) that calls `assert_success(class_name, expected_output)`.

### Step 1 - Write the Java program

Create `tests/test_data/<YourExample>.java`.
The file should define a `public class <YourExample>` with a `main` method.
Place it in the appropriate package under `samples/` if it is more than a trivial snippet.

```java
// tests/test_data/samples/javacore/trivial/HelloContributor.java
package samples.javacore.trivial;

public class HelloContributor {
    public static void main(String[] args) {
        System.out.println("Hello, contributor!");
    }
}
```

### Step 2 - Compile it

```sh
cd tests/test_data
javac -d . samples/javacore/trivial/HelloContributor.java
```

The compiled `.class` file must live alongside the source in the repository so that CI can
run the tests without a separate compilation step.

### Step 3 - Add the test function

Open `tests/integration_tests.rs` and add:

```rust
#[test]
fn hello_contributor() {
    assert_success(
        "samples.javacore.trivial.HelloContributor",
        "Hello, contributor!\n",
    );
}
```

`assert_success` is defined in `tests/utils.rs`; it invokes `cargo run` with the given class
name and asserts that stdout equals the expected string.

---

## Adding a new native method

JDK classes often declare methods as `native`, meaning the implementation lives in C/C++ inside
the JDK itself.
`rusty-jvm` replaces those implementations with Rust code registered in a static lookup table.

### Step 1 - Find the JVM method signature

Look up the method's JVM internal descriptor.
For example, `System.nanoTime()` has the descriptor `()J` (no args, returns `long`).

### Step 2 - Write the Rust implementation

Add an implementation function in `src/vm/system_native/`.
Put related methods in an appropriate existing file, or create a new one for a new class.

```rust
// src/vm/system_native/system.rs  (example)
pub(crate) fn nano_time(_args: &[i32]) -> crate::vm::error::Result<Option<Vec<i32>>> {
    let nanos = /* … platform call … */;
    Ok(Some(vec![(nanos >> 32) as i32, nanos as i32]))
}
```

Return `Ok(None)` for `void` methods, or `Ok(Some(vec![…]))` where the `Vec<i32>` encodes the
return value in JVM stack-slot format (one `i32` per 32-bit slot; `long`/`double` use two slots,
high word first).

### Step 3 - Register the method

Open `src/vm/execution_engine/system_native_table.rs` and add an entry to the `NATIVE_TABLE`
static map:

```rust
("java/lang/System", "nanoTime", "()J") => system::nano_time,
```

The key is a `(class_name, method_name, descriptor)` triple using JVM internal names
(slashes, not dots).

### Step 4 - Add an integration test

Follow the [integration test guide](#adding-an-integration-test) to verify the new method works
end-to-end with a real Java program.

---

## Project structure

```
rusty-jvm/
├── src/
│   ├── lib.rs               # Public API: Arguments, run()
│   └── vm/
│       ├── execution_engine/ # Bytecode interpreter & opcode processors
│       ├── heap/             # Object and array storage
│       ├── method_area/      # Class loading, vtable, method resolution
│       ├── stack/            # Operand stack and call frames
│       ├── jni/              # JNI ABI bridge
│       ├── system_native/    # Native method implementations & dispatch table
│       ├── exception/        # Exception creation and propagation
│       ├── properties/       # System properties and classpath resolution
│       └── validation/       # Bytecode pre-checks
├── tests/
│   ├── test_data/            # Java source and .class files for integration tests
│   ├── integration_tests.rs  # Main integration test suite
│   └── utils.rs              # Test helpers (assert_success, …)
├── jclassfile/               # Sub-crate: .class file parser
├── jdescriptor/              # Sub-crate: JVM type descriptor parser
├── jimage-rs/                # Sub-crate: JDK jimage archive reader
├── jniname/                  # Sub-crate: JNI name mangling utilities
└── docs/
    ├── architecture.md       # Architecture diagrams (Mermaid)
    └── design-decisions.md   # Key design decision records
```

---

## Relevant JVM Specification sections

| Topic | JVMS section |
|---|---|
| Class file format | [§4](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html) |
| Class loading & resolution | [§5](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html) |
| Bytecode instruction set | [§6](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html) |
| Operand stack & frames | [§2.6](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6) |
| Static initialisation | [§5.5](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.5) |
| Virtual method dispatch | [§5.4.5](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.4.5) |
| `invokedynamic` | [§6.5 invokedynamic](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokedynamic) |
| Exceptions | [§2.10](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.10) |
