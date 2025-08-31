use crate::utils::{assert_success_with_args, get_output_with_args, TEST_PATH};
use std::ops::Deref;
use std::path::PathBuf;
use tempfile::TempDir;

mod utils;

const ENTRY_POINT_ARG: &str = "samples.nio.niofileexample.NioFileExample";

#[test]
fn should_support_nio_file() {
    let temp_dir = TempDir::new_in(TEST_PATH.deref()).expect("failed to create temp dir");
    let dir_name = temp_dir
        .path()
        .file_name()
        .expect("dir_name is not a string")
        .to_str()
        .expect("dir_name is not a string");
    let mut file_path = PathBuf::new();
    file_path.push(dir_name);
    file_path.push("nio");
    file_path.push("example");
    file_path.push("nio_file_example.txt");

    let dir_path = file_path.parent().unwrap();
    let grand_parent_dir = dir_path.parent().unwrap();

    create_dirs(dir_path.to_str().unwrap());
    is_writable(dir_path.to_str().unwrap());
    let content = "Some content";
    write_file(file_path.to_str().unwrap(), content);
    //read_file(file_path.to_str().unwrap(), content); todo: uncomment when Unsafe memory methods are fixed
    delete_file(file_path.to_str().unwrap());
    delete_file(dir_path.to_str().unwrap());
    delete_file(grand_parent_dir.to_str().unwrap());
}

fn create_dirs(path: &str) {
    assert_success_with_args(
        ENTRY_POINT_ARG,
        &["--create-dirs", path],
        &format!("Created directories: {path}\n"),
    );
}

fn is_writable(path: &str) {
    assert_success_with_args(
        ENTRY_POINT_ARG,
        &["--is-writable", path],
        &format!("{path} is writable: true\n"),
    );
}

fn write_file(file_path: &str, content: &str) {
    assert_success_with_args(
        ENTRY_POINT_ARG,
        &["--write-file", file_path, content],
        &format!("Written: {file_path}\n"),
    );
}

#[allow(dead_code)]
fn read_file(file_path: &str, content: &str) {
    let actual_content = get_output_with_args(ENTRY_POINT_ARG, &["--read-file", file_path]);
    assert_eq!(actual_content, content);
}

fn delete_file(path: &str) {
    assert_success_with_args(
        ENTRY_POINT_ARG,
        &["--delete", path],
        &format!("Deleted: {path}\n"),
    );
}
