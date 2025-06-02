//! # Rusty JVM
//!
//! A JVM interpreter written in Rust from scratch — no dependency on existing JVMs.
//!
//! `rusty-jvm` executes Java bytecode in interpreted mode and aims to support as much of the Java language as possible.
//! Currently supports a range of Java language features up to Java 23 (excluding GC and lambdas for now).
//!
//! ## Features
//! Refer to the [README.md](https://github.com/hextriclosan/rusty-jvm/blob/main/README.md#implemented-key-features)
//!
//! ## Usage Modes
//!
//! The binary supports three main modes, based on CLI arguments:
//!
//! - (default) Runs a Java class file with an entry point.
//! - `--install` – Installs standard libraries (used by the VM).
//! - `--purge` – Removes installed standard libraries.
//!
//! ## Example
//! ### Installing core libraries
//! ```bash
//! rusty-jvm --install
//! ```
//! ### Creating a simple Java program
//! ###### Windows
//! ```bash
//! echo public class Hello { public static void main(String[] args) { System.out.println("Hello, world!"); } } > Hello.java
//! ```
//! ###### Unix-like systems
//! ```bash
//! echo 'public class Hello { public static void main(String[] args) { System.out.println("Hello, world!"); } }' > Hello.java
//! ```
//! ### Compiling the Java program (current supported version is Java 23)
//! ```bash
//! javac Hello.java
//! ```
//! ### Running the compiled Java program with Rusty JVM
//! ```bash
//! rusty-jvm Hello
//! ```
mod vm;
use derive_new::new;
use getset::Getters;
use indexmap::IndexMap;
pub use vm::run;

#[derive(PartialEq, Debug, new, Getters)]
#[get = "pub"]
/// Represents the parsed command-line arguments for the Java program.
pub struct ParsedArguments {
    /// The entry point for the Java program.  
    /// This may be empty when running in install or purge mode.
    entry_point: String,
    /// The arguments passed to the Java program.
    program_args: Vec<String>,
    /// The standard Java options (e.g., `-version`, `-help`).
    java_standard_options: Vec<String>,
    /// The options passed to the Java launcher (e.g., `--install`, `--purge`).
    java_launcher_options: Vec<String>,
    /// System properties set for the Java program (e.g., `-Dproperty=value`).
    system_properties: IndexMap<String, String>,
    /// JVM options (e.g., `-Xmx1024m`).
    jvm_options: Vec<String>,
    /// Advanced JVM options (e.g., `-XX:+UseG1GC`).
    advanced_jvm_options: Vec<String>,
}
