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
