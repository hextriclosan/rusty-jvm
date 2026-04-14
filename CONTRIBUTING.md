# Contributing to rusty-jvm

Thank you for your interest in contributing!
This guide explains how to build the project, run the tests, and extend the VM with new features.

---

## Table of Contents
- [Building the project](#building-the-project)
  - [Prerequisites](#prerequisites)
- [Running the tests](#running-the-tests)
- [Adding an integration test](#adding-an-integration-test)
  - [Step 1 - Write the Java program](#step-1---write-the-java-program)
  - [Step 2 - Add the test function](#step-2---add-the-test-function)
- [Project structure](#project-structure)

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
No need to compile it manually, the test runner will do that for you.

```java
// tests/test_data/samples/javacore/trivial/HelloContributor.java
package samples.javacore.trivial;

public class HelloContributor {
    public static void main(String[] args) {
        System.out.println("Hello, contributor!");
    }
}
```

### Step 2 - Add the test function

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
│   └── utils.rs              # Test helpers (assert_success, etc.)
├── jclassfile/               # Sub-crate: .class file parser
├── jdescriptor/              # Sub-crate: JVM type descriptor parser
├── jimage-rs/                # Sub-crate: JDK jimage archive reader
├── jniname/                  # Sub-crate: JNI name mangling utilities
└── docs/                     # Design docs and architecture diagrams
```
