#![allow(dead_code)]
use assert_cmd::Command;
use once_cell::sync::Lazy;
use std::{env, fs, iter};

const PATH: &str = "tests/test_data";

pub(crate) static REPO_PATH: Lazy<std::path::PathBuf> =
    Lazy::new(|| env::current_dir().expect("Failed to get current directory"));

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

pub fn assert_success(entry: &str, expected: &str) {
    assert_success_with_args(entry, &vec![], expected)
}

pub fn assert_success_with_args(entry: &str, arguments: &[&str], expected: &str) {
    #[cfg(target_os = "windows")]
    let expected = to_windows(expected);

    let args = iter::once(entry)
        .chain(arguments.iter().copied())
        .collect::<Vec<_>>();
    get_command(&args)
        .assert()
        .success()
        .stdout(expected.to_string());
}

pub fn get_output(entry: &str) -> String {
    get_output_with_args(entry, &vec![])
}

pub fn get_output_with_args(entry: &str, arguments: &[&str]) -> String {
    let args = iter::once(entry)
        .chain(arguments.iter().copied())
        .collect::<Vec<_>>();
    let output = get_command(&args)
        .output()
        .expect("Failed to execute process");

    String::from_utf8(output.stdout).expect("Failed to convert output to string")
}

pub fn assert_file(entry: &str, file_path: &str, expected_file_content: &str) {
    assert_success(entry, "");

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
    let repo_path = REPO_PATH.as_path();

    let mut cmd = Command::cargo_bin("rusty-jvm").expect("Failed to locate rusty-jvm binary");
    cmd.env("RUSTY_JAVA_HOME", repo_path)
        .current_dir(PATH)
        .args(arguments);
    cmd
}

#[cfg(target_os = "windows")]
pub fn to_windows(input: &str) -> String {
    input.replace("\r\n", "\n").replace("\n", "\r\n")
}
