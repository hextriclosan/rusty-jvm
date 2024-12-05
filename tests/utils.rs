use assert_cmd::Command;
use std::env;

const PATH: &str = "tests/test_data";

fn get_command(entry: &str) -> Command {
    let repo_path = env::current_dir().expect("Failed to get current directory");

    let mut cmd = Command::cargo_bin("rusty-jvm").expect("Failed to locate rusty-jvm binary");
    cmd.env("RUSTY_JAVA_HOME", repo_path)
        .current_dir(PATH)
        .arg(entry);
    cmd
}

#[allow(dead_code)]
pub fn assert_success(entry: &str, expected: &str) {
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
