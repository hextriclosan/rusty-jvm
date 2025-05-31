mod cli;
use crate::cli::execution_mode::ExecutionMode;
use crate::cli::help::help_msg;
use crate::cli::installer::{do_install, do_purge};
use crate::cli::utils::resolve_std_dir;
use clap::{Arg, ArgAction, Command};
use rusty_jvm::{run, ParsedArguments};
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

    let exit_code = handle_execution(raw_args.into()).unwrap_or_else(|msg| {
        eprint!("{msg}");
        EXIT_FAILURE
    });

    process::exit(exit_code);
}

fn handle_execution(mode: ExecutionMode) -> Result<i32, String> {
    match mode {
        ExecutionMode::Normal(parsed) => handle_normal(parsed),
        ExecutionMode::Install(yes) => handle_install(yes),
        ExecutionMode::Purge(yes) => handle_purge(yes),
    }
}

fn handle_normal(parsed: ParsedArguments) -> Result<i32, String> {
    if parsed.entry_point().is_empty() {
        return Err(format!(
            "No entry point provided. Please specify the main class to run.\n{}",
            help_msg()
        ));
    }

    let std_dir = resolve_std_dir().map_err(|err| {
        format!("Error resolving standard library directory: {}", err)
    })?;

    let std_dir = std_dir.ok_or_else(|| {
        r#"Standard library directory was not found. You can either:
- Run the installation command: rusty-jvm --install
- Set the RUSTY_LIB_DIR environment variable to the path of the standard libraries
"#
            .to_string()
    })?;

    run(&parsed, &std_dir).map(|()| EXIT_SUCCESS).map_err(|err| {
        if err.is_exception_thrown() {
            String::new() // no error message needed for exceptions since it already handled by the VM
        } else {
            format!("VM execution failed: {}", err)
        }
    })
}

fn handle_install(yes: bool) -> Result<i32, String> {
    do_install(yes).map(|()| EXIT_SUCCESS).map_err(|err| format!("Installation failed: {}", err))
}

fn handle_purge(yes: bool) -> Result<i32, String> {
    do_purge(yes).map(|()| EXIT_SUCCESS).map_err(|err| format!("Purge failed: {}", err))
}
