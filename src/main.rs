use clap::{arg, Command};
use std::process;
use vm::vm::VM;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("rusty-jvm")
        .arg(
            arg!(--"std-dir" <dir>)
                .help("Path to Java Standard libraries dir")
                .required(true),
        )
        .arg(
            arg!(--"entry-point" <main_class>)
                .help("Class to run")
                .required(true),
        )
        .get_matches();

    let std_dir = matches
        .get_one::<String>("std-dir")
        .expect("Missing standard library directory");
    let entry_point = matches
        .get_one::<String>("entry-point")
        .expect("Missing entry point");
    let vm = VM::new(std_dir)?;

    let result = match vm.run(entry_point) {
        Ok(output) => output,
        Err(err) => {
            eprintln!("VM execution failed: {}", err);
            process::exit(1);
        }
    };

    println!("\nresult={:?}", result.map_or_else(|| vec![], |v| v));
    Ok(())
}
