use clap::{arg, Command};
use std::process;
use vm::vm::VM;

fn main() {
    let matches = Command::new("rusty-jvm")
        .arg(arg!(<main_class>).help("Class to run").required(true))
        .get_matches();

    let entry_point = matches
        .get_one::<String>("main_class")
        .expect("Missing entry point");

    if let Err(err) = VM::run(entry_point) {
        eprintln!("VM execution failed: {}", err);
        process::exit(1);
    }
}
