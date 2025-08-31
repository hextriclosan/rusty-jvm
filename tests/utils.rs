#![allow(dead_code)]
use assert_cmd::Command;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::{env, fs, iter};
use tempfile::TempDir;

#[derive(PartialEq)]
pub enum ExecutionResult {
    Success,
    Failure,
}

pub(crate) static REPO_PATH: Lazy<PathBuf> =
    Lazy::new(|| env::current_dir().expect("Failed to get current directory"));
pub(crate) static TEST_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let out_dir = PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".into()))
        .join("java_classes_for_tests");

    out_dir
});

pub fn is_bigendian() -> bool {
    cfg!(target_endian = "big")
}

pub fn line_ending() -> &'static str {
    if cfg!(windows) {
        "\r\n"
    } else {
        "\n"
    }
}

pub fn get_file_separator() -> &'static str {
    if cfg!(windows) {
        "\\"
    } else {
        "/"
    }
}

pub fn get_path_separator() -> &'static str {
    if cfg!(windows) {
        ";"
    } else {
        ":"
    }
}

pub fn map_library_name(name: &str) -> String {
    if cfg!(target_os = "windows") {
        format!("{name}.dll")
    } else if cfg!(target_os = "macos") {
        format!("lib{name}.dylib")
    } else if cfg!(target_os = "linux") {
        format!("lib{name}.so")
    } else {
        unreachable!("Unsupported OS")
    }
}

pub fn get_os_name() -> &'static str {
    if cfg!(windows) {
        "windows"
    } else if cfg!(target_os = "macos") {
        "mac"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        unreachable!("Unsupported OS")
    }
}

pub fn assert_success(entry: &str, expected_stdout: &str) {
    assert_success_with_args(entry, &vec![], expected_stdout)
}

pub fn assert_success_with_stderr(entry: &str, expected_stdout: &str, expected_stderr: &str) {
    assert_success_with_args_with_stderr(entry, &vec![], expected_stdout, expected_stderr)
}

pub fn assert_failure_with_stderr(entry: &str, expected_stdout: &str, expected_stderr: &str) {
    assert_failure_with_args_with_stderr(entry, &vec![], expected_stdout, expected_stderr)
}

fn assert_success_with_args_with_stderr(
    entry: &str,
    arguments: &[&str],
    expected_stdout: &str,
    expected_stderr: &str,
) {
    assert_with_all_args(
        &[],
        entry,
        arguments,
        expected_stdout,
        expected_stderr,
        ExecutionResult::Success,
    )
}

fn assert_failure_with_args_with_stderr(
    entry: &str,
    arguments: &[&str],
    expected_stdout: &str,
    expected_stderr: &str,
) {
    assert_with_all_args(
        &[],
        entry,
        arguments,
        expected_stdout,
        expected_stderr,
        ExecutionResult::Failure,
    )
}

pub fn assert_with_all_args(
    program_args: &[&str],
    entry: &str,
    arguments: &[&str],
    expected_stdout: &str,
    expected_stderr: &str,
    expected_result: ExecutionResult,
) {
    #[cfg(target_os = "windows")]
    let expected_stdout = to_windows(expected_stdout);
    #[cfg(target_os = "windows")]
    let expected_stderr = to_windows(expected_stderr);

    let args = program_args
        .iter()
        .copied()
        .chain(iter::once(entry))
        .chain(arguments.iter().copied())
        .collect::<Vec<_>>();
    let assert = get_command(&args).assert();
    let assert = if expected_result == ExecutionResult::Success {
        assert.success()
    } else {
        assert.failure()
    };

    let output = assert.get_output();
    let actual_stdout =
        String::from_utf8(output.stdout.clone()).expect("Failed to convert stdout to string");
    let actual_stderr =
        String::from_utf8(output.stderr.clone()).expect("Failed to convert stderr to string");

    let actual_stdout = normalize_lambda(&actual_stdout);

    assert_eq!(actual_stdout, expected_stdout.to_string());
    assert_eq!(actual_stderr, expected_stderr.to_string());
}

pub fn assert_success_with_args(entry: &str, arguments: &[&str], expected_stdout: &str) {
    assert_success_with_args_with_stderr(entry, arguments, expected_stdout, "")
}

pub fn assert_failure(entry: &str, expected: &str) {
    assert_failure_with_args(entry, &vec![], expected)
}

fn assert_failure_with_args(entry: &str, arguments: &[&str], expected: &str) {
    #[cfg(target_os = "windows")]
    let expected = to_windows(expected);

    let args = iter::once(entry)
        .chain(arguments.iter().copied())
        .collect::<Vec<_>>();
    get_command(&args)
        .assert()
        .failure()
        .stderr(format!("{}\n", expected)); //todo: remove \n (it's added as a workaround for windows)
}

pub fn get_output(entry: &str) -> String {
    get_output_with_args(entry, &vec![])
}

pub fn get_output_with_args(entry: &str, arguments: &[&str]) -> String {
    let args = iter::once(entry)
        .chain(arguments.iter().copied())
        .collect::<Vec<_>>();

    get_output_with_raw_args(&args)
}

pub fn get_output_with_raw_args(args: &[&str]) -> String {
    let output = get_command(&args)
        .output()
        .expect("Failed to execute process");

    String::from_utf8(output.stdout).expect("Failed to convert output to string")
}

pub fn assert_file_with_args(
    entry: &str,
    arguments: &[&str],
    file_path: &str,
    expected_file_content: &str,
) {
    assert_success_with_args(entry, arguments, "");

    #[cfg(target_os = "windows")]
    let expected_file_content = to_windows(expected_file_content);

    assert!(fs::metadata(file_path).is_ok(), "File does not exist");
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    assert_eq!(
        content, expected_file_content,
        "File content does not match"
    );
    fs::remove_file(file_path).expect("Failed to delete file");
}

fn get_command(arguments: &[&str]) -> Command {
    let mut cmd = Command::cargo_bin("rusty-jvm").expect("Failed to locate rusty-jvm binary");
    cmd.current_dir(TEST_PATH.as_path()).args(arguments);
    cmd
}

#[cfg(target_os = "windows")]
pub fn to_windows(input: &str) -> String {
    input.replace("\r\n", "\n").replace("\n", "\r\n")
}

/// `com.example.Example$$Lambda/0x000001f9ca001850` -> `com.example.Example$$Lambda/0x0000`
fn normalize_lambda(to_strip: &str) -> String {
    let re = regex::Regex::new(r"(\$\$Lambda/0x0000)[0-9a-fA-F]+").unwrap();
    re.replace_all(to_strip, "$1").to_string()
}

pub fn tmp_file(file_name: &str) -> (String, TempDir) {
    let tmp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = tmp_dir.path().join(file_name);
    let file_path = file_path
        .to_str()
        .expect("Conversion of file failed")
        .to_owned();
    (file_path, tmp_dir)
}
