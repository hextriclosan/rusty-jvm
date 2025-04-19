mod argument_parser;

use crate::argument_parser::group_args;
use clap::{Arg, ArgAction, Command};
use std::process;
use vm::vm::VM;

fn main() {
    let matches = Command::new("rusty-jvm")
        .arg(
            Arg::new("args")
                .action(ArgAction::Append)
                .num_args(1..) // capture everything after program name
                .trailing_var_arg(true)
                .allow_hyphen_values(true),
        )
        .get_matches();

    let raw_args = matches
        .get_many::<String>("args")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let parsed = group_args(raw_args).expect("Could not parse arguments");

    if let Err(err) = VM::run(
        parsed.entry_point(),
        &parsed.system_properties(),
        &parsed.program_args(),
    ) {
        eprintln!("VM execution failed: {}", err);
        process::exit(1);
    }
}
