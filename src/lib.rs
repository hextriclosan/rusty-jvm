//! # Rusty JVM
//!
//! A Java Virtual Machine (JVM) interpreter written in Rust from scratch — with no dependency on existing JVMs.
//!
//! `rusty-jvm` executes Java bytecode in interpreted mode and aims to support as much of the Java language as possible.
//! It currently supports a broad set of language features up to Java 23, with garbage collection and multithreading planned for future releases.
//!
//! ## Features
//! See the [README.md](https://github.com/hextriclosan/rusty-jvm/blob/rusty_jvm-v0.4.0/README.md#implemented-key-features) for the full list of implemented features.
//!
//! ## Usage
//!
//! ### Step 1: Create a simple Java program
//! Create a file named `Hello.java` with the following content:
//! ```java
//! public class Hello {
//!     public static void main(String[] args) {
//!         System.out.println("Hello, world!");
//!     }
//! }
//! ```
//!
//! #### Shortcut (one-liner)
//! On Windows (cmd):
//! ```shell
//! echo public class Hello { public static void main(String[] args) { System.out.println("Hello, world!"); } } > Hello.java
//! ```
//! On Unix-like systems:
//! ```bash
//! echo 'public class Hello { public static void main(String[] args) { System.out.println("Hello, world!"); } }' > Hello.java
//! ```
//!
//! ### Step 2: Compile the Java program (currently tested with Java 23)
//! ```bash
//! javac Hello.java
//! ```
//!
//! ### Step 3: Run the compiled Java program with Rusty JVM
//! ```bash
//! rusty-jvm Hello
//! ```
mod vm;
use derive_new::new;
use getset::Getters;
use indexmap::IndexMap;
pub use vm::run;

#[derive(PartialEq, Debug, new, Getters, Default)]
#[get = "pub"]
/// Represents the command-line arguments for the Java program.
pub struct Arguments {
    /// The entry point for the Java program.  
    /// This may be empty when running in install or purge mode.
    entry_point: String,
    /// The arguments passed to the Java program.
    program_args: Vec<String>,
    /// The standard Java options (e.g., `-version`, `-help`).
    java_standard_options: Vec<String>,
    /// The options passed to the Java launcher (e.g., `--dry-run`).
    java_launcher_options: Vec<String>,
    /// System properties set for the Java program (e.g., `-Dproperty=value`).
    system_properties: IndexMap<String, String>,
    /// JVM options (e.g., `-Xmx1024m`).
    jvm_options: Vec<String>,
    /// Advanced JVM options (e.g., `-XX:+UseG1GC`).
    advanced_jvm_options: Vec<String>,
}

impl Arguments {
    /// Creates a new `Arguments` instance with the specified entry point.
    /// # Arguments
    /// * `entry_point` - The entry point for the Java program.
    /// # Returns
    /// A new `Arguments` instance with the specified entry point and default values for other fields.
    pub fn new_with_entry_point(entry_point: String) -> Self {
        Self {
            entry_point,
            ..Default::default()
        }
    }
}
