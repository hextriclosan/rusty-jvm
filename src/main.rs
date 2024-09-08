use clap::{arg, Arg, ArgAction, Command};
use std::process;
use vm::vm::VM;

fn main() {
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
        .arg(
            Arg::new("classes")
                .action(ArgAction::Append)
                .help("Java classes to load")
                .required(true),
        )
        .get_matches();

    let std_dir = matches.get_one::<String>("std-dir").unwrap();
    let entry_point = matches.get_one::<String>("entry-point").unwrap();
    let classes = matches
        .get_many::<String>("classes")
        .unwrap()
        .into_iter()
        .map(|s| s.as_str())
        .collect();
    let vm = match VM::new(classes, std_dir) {
        Ok(vm) => vm,
        Err(err) => {
            eprintln!("Failed to create VM: {}", err);
            process::exit(1);
        }
    };

    let result = match vm.run(entry_point) {
        Ok(output) => output,
        Err(err) => {
            eprintln!("VM execution failed: {}", err);
            process::exit(1);
        }
    };

    println!(
        "\nresult={}",
        result.map_or_else(|| "<empty>".to_string(), |v| v.to_string())
    );
}
