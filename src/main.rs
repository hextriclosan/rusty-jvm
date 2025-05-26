mod argument_parser;

use crate::argument_parser::group_args;
use clap::{Arg, ArgAction, Command};

use rusty_jvm::VM;
use std::process;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;

fn main() {
    let matches = Command::new("rusty-jvm")
        .arg(
            Arg::new("args")
                .action(ArgAction::Append)
                .num_args(1..) // capture everything after program name
                .trailing_var_arg(true)
                .allow_hyphen_values(true),
        )
        .override_help(help())
        .get_matches();

    let raw_args = matches
        .get_many::<String>("args")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let parsed = match group_args(raw_args) {
        Ok(parsed) => parsed,
        Err(err) if matches!(err, argument_parser::ParserError::NoEntryPointProvided) => {
            eprintln!("{}", help());
            process::exit(EXIT_FAILURE);
        }
        Err(err) => {
            eprintln!("Parsing error: {:?}", err);
            process::exit(EXIT_FAILURE);
        }
    };

    let exit_code = match VM::run(
        parsed.entry_point(),
        parsed.system_properties().clone(),
        &parsed.program_args(),
    ) {
        Ok(()) => EXIT_SUCCESS,
        Err(err) if err.is_exception_thrown() => EXIT_FAILURE, // Unhandled exception, print nothing here, since stack trace is already printed by the VM
        Err(err) => {
            eprintln!("VM execution failed: {}", err);
            EXIT_FAILURE
        }
    };

    process::exit(exit_code)
}

fn help() -> &'static str {
    r#"Usage: rusty-jvm [options] <mainclass> [args...]

Options:
    -D<name>=<value>  Set a system property
    -X<option>        JVM options
    -XX:<option>      Advanced JVM options
    --<option>        Java launcher options
    -<option>         Java standard options
    -h, --help        Show this help message"#
}
