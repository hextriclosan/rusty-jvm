use clap::{arg, Command};
use std::process;
use vm::vm::VM;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("rusty-jvm")
        .arg(arg!(<main_class>).help("Class to run").required(true))
        .get_matches();

    let entry_point = matches
        .get_one::<String>("main_class")
        .expect("Missing entry point");

    let result = match VM::run(entry_point) {
        Ok(output) => output,
        Err(err) => {
            eprintln!("VM execution failed: {}", err);
            process::exit(1);
        }
    };

    println!("{:?}", result.map_or_else(|| vec![], |v| v));
    Ok(())
}
