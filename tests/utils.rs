use assert_cmd::Command;
use std::{env, fs};

const PATH: &str = "tests/test_data";

pub fn is_bigendian() -> bool {
    cfg!(target_endian = "big")
}

#[allow(dead_code)]
pub fn assert_success(entry: &str, expected: &str) {
    #[cfg(target_os = "windows")]
    let expected = to_windows(expected);

    get_command(entry)
        .assert()
        .success()
        .stdout(expected.to_string());
}

#[allow(dead_code)]
pub fn get_output(entry: &str) -> String {
    let output = get_command(entry)
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

fn get_command(entry: &str) -> Command {
    let repo_path = env::current_dir().expect("Failed to get current directory");

    let mut cmd = Command::cargo_bin("rusty-jvm").expect("Failed to locate rusty-jvm binary");
    cmd.env("RUSTY_JAVA_HOME", repo_path)
        .current_dir(PATH)
        .arg(entry);
    cmd
}

#[cfg(target_os = "windows")]
fn to_windows(input: &str) -> String {
    input.replace("\r\n", "\n").replace("\n", "\r\n")
}
