mod cli;
use crate::cli::argument_parser::group_args;
use crate::cli::installer::{do_install, do_purge};
use crate::cli::utils::resolve_std_dir;
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

    let parsed = group_args(raw_args);
    if parsed.is_install_mode() {
        let exit_code = match do_install(parsed.is_yes()) {
            Ok(()) => EXIT_SUCCESS,
            Err(err) => {
                eprintln!("Installation failed: {}", err);
                EXIT_FAILURE
            }
        };
        process::exit(exit_code);
    }

    if parsed.is_purge_mode() {
        let exit_code = match do_purge(parsed.is_yes()) {
            Ok(()) => EXIT_SUCCESS,
            Err(err) => {
                eprintln!("Purge failed: {}", err);
                EXIT_FAILURE
            }
        };
        process::exit(exit_code);
    }

    let std_dir = match resolve_std_dir() {
        Ok(Some(dir)) => dir,
        Ok(None) => {
            eprintln!(
                r#"Standard library directory was not found. You can either:
    - Run the installation command: rusty-jvm --install
    - Set the RUSTY_LIB_DIR environment variable to the path of the standard libraries
"#
            );
            process::exit(EXIT_FAILURE);
        }
        Err(err) => {
            eprintln!("Error resolving standard library directory: {}", err);
            process::exit(EXIT_FAILURE);
        }
    };

    let entry_point = match parsed.entry_point() {
        Some(ep) => ep,
        None => {
            eprintln!("No entry point provided. Please specify the main class to run.");
            eprintln!("{}", help());
            process::exit(EXIT_FAILURE);
        }
    };

    let exit_code = match VM::run(
        entry_point,
        parsed.system_properties().clone(),
        &parsed.program_args(),
        &std_dir,
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
    -h, --help        Show this help message
    
Installation options:
    --install         Download and install standard libraries
    --purge           Remove all installed standard libraries
    --yes             Automatically say "yes" to prompts
"#
}
