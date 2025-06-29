mod cli;
use crate::cli::argument_parser::into_args;
use crate::cli::help::help_msg;
use clap::{Arg, ArgAction, Command};
use rusty_jvm::{run, Arguments};
use std::path::PathBuf;
use std::{env, process};

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
        .override_help(help_msg())
        .get_matches();

    let raw_args = matches
        .get_many::<String>("args")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let exit_code = handle_execution(into_args(raw_args)).unwrap_or_else(|msg| {
        eprintln!("{msg}");
        EXIT_FAILURE
    });

    process::exit(exit_code);
}

fn handle_execution(arguments: Arguments) -> Result<i32, String> {
    if arguments.entry_point().is_empty() {
        return Err(format!(
            "No entry point provided. Please specify the main class to run.\n{}",
            help_msg()
        ));
    }

    let java_home = env::var("JAVA_HOME")
        .map_err(|_| "The JAVA_HOME environment variable is not set.\nSet JAVA_HOME to an existing JDK 23 directory.".to_string())?;

    run(&arguments, &PathBuf::from(java_home))
        .map(|()| EXIT_SUCCESS)
        .map_err(|err| {
            if err.is_exception_thrown() {
                String::new() // no error message needed for exceptions since it already handled by the VM
            } else {
                format!("VM execution failed: {}", err)
            }
        })
}
