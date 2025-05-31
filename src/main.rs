mod cli;
use crate::cli::execution_mode::ExecutionMode;
use crate::cli::help::help_msg;
use crate::cli::installer::{do_install, do_purge};
use crate::cli::utils::resolve_std_dir;
use clap::{Arg, ArgAction, Command};
use rusty_jvm::run;
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
        .override_help(help_msg())
        .get_matches();

    let raw_args = matches
        .get_many::<String>("args")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let exit_code = match raw_args.into() {
        ExecutionMode::Install(yes) => match do_install(yes) {
            Ok(()) => EXIT_SUCCESS,
            Err(err) => {
                eprintln!("Installation failed: {}", err);
                EXIT_FAILURE
            }
        },
        ExecutionMode::Purge(yes) => match do_purge(yes) {
            Ok(()) => EXIT_SUCCESS,
            Err(err) => {
                eprintln!("Purge failed: {}", err);
                EXIT_FAILURE
            }
        },
        ExecutionMode::Normal(parsed) => {
            if parsed.entry_point().is_empty() {
                eprintln!("No entry point provided. Please specify the main class to run.");
                eprintln!("{}", help_msg());
                process::exit(EXIT_FAILURE);
            }
            match resolve_std_dir() {
                Ok(Some(std_dir)) => match run(&parsed, &std_dir) {
                    Ok(()) => EXIT_SUCCESS,
                    Err(err) if err.is_exception_thrown() => EXIT_FAILURE,
                    Err(err) => {
                        eprintln!("VM execution failed: {}", err);
                        EXIT_FAILURE
                    }
                },
                Ok(None) => {
                    eprintln!(
                        r#"Standard library directory was not found. You can either:
- Run the installation command: rusty-jvm --install
- Set the RUSTY_LIB_DIR environment variable to the path of the standard libraries
"#
                    );
                    EXIT_FAILURE
                }
                Err(err) => {
                    eprintln!("Error resolving standard library directory: {}", err);
                    EXIT_FAILURE
                }
            }
        }
    };

    process::exit(exit_code)
}
