use crate::utils::{assert_success_with_args, get_output_with_args, TEST_PATH};
use derive_new::new;
use std::env;
use std::fs::remove_dir_all;
use std::path::PathBuf;

mod utils;

const ENTRY_POINT_ARG: &str = "samples.nio.niofileexample.NioFileExample";

#[test]
fn should_support_nio_file() {
    let mut file_path = PathBuf::new();
    file_path.push("..");
    file_path.push("tmp");
    file_path.push("nio");
    file_path.push("example");
    file_path.push("nio_file_example.txt");
    let dir_path = file_path.parent().unwrap();
    let grand_parent_dir = dir_path.parent().unwrap();

    let _guard = CleanUpOnPanic::new(grand_parent_dir.to_path_buf());

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

#[derive(new)]
struct CleanUpOnPanic {
    temp_dir: PathBuf,
}

impl Drop for CleanUpOnPanic {
    fn drop(&mut self) {
        if std::thread::panicking() {
            env::set_current_dir(TEST_PATH.as_path()).unwrap();
            remove_dir_all(&self.temp_dir).unwrap_or_else(|e| {
                eprintln!(
                    "Failed to remove temp dir: {} ({e})",
                    self.temp_dir.display()
                )
            });
        }
    }
}
