mod utils;

use crate::utils::{assert_success_with_args, REPO_PATH};
use derive_new::new;
use once_cell::sync::Lazy;
use path_absolutize::Absolutize;
use std::fs::remove_file;
use std::path::{Path, PathBuf};
use std::string::ToString;

#[test]
fn should_support_java_io_file() {
    let _guard = CleanUpOnPanic {};

    file_info_when_does_not_exist();
    create_file_when_does_not_exist();
    create_file_when_exists();
    file_info_when_exists();
    delete_file_when_exists();
    delete_file_when_does_not_exist();
}

fn file_info_when_does_not_exist() {
    let template_values = TemplateValues::new(
        to_string(&file_path()),
        to_string(&absolute_path()),
        to_string(&canonical_path()),
        to_string(&file_path()),
        FILE_NAME.to_string(),
        to_string(&DIR_PATH),
        to_string(&DIR_PATH),
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    );
    let expected_output = resolve_template(&template_values);

    assert_success_with_args(
        ENTRY_POINT_ARG,
        &["--info", &FILE_PATH_ARG],
        &expected_output,
    );
}

fn create_file_when_does_not_exist() {
    assert_success_with_args(
        ENTRY_POINT_ARG,
        &["--create", &FILE_PATH_ARG],
        &format!("{FILE_NAME}: created\n"),
    );
}

fn create_file_when_exists() {
    assert_success_with_args(
        ENTRY_POINT_ARG,
        &["--create", &FILE_PATH_ARG],
        &format!("{FILE_NAME}: already exists\n"),
    );
}

fn file_info_when_exists() {
    let (is_hidden, is_writable, is_readable, is_executable) =
        file_properties(&canonical_path()).expect("Failed to get file properties");

    let template_values = TemplateValues::new(
        to_string(&file_path()),
        to_string(&absolute_path()),
        to_string(&canonical_path()),
        to_string(&file_path()),
        FILE_NAME.to_string(),
        to_string(&DIR_PATH),
        to_string(&DIR_PATH),
        false,
        true,
        true,
        false,
        is_hidden,
        is_writable,
        is_readable,
        is_executable,
    );
    let expected_output = resolve_template(&template_values);

    assert_success_with_args(
        ENTRY_POINT_ARG,
        &["--info", &FILE_PATH_ARG],
        &expected_output,
    );
}

fn delete_file_when_exists() {
    assert_success_with_args(
        ENTRY_POINT_ARG,
        &["--delete", &FILE_PATH_ARG],
        &format!("{FILE_NAME}: deleted\n"),
    );
}

fn delete_file_when_does_not_exist() {
    assert_success_with_args(
        ENTRY_POINT_ARG,
        &["--delete", &FILE_PATH_ARG],
        &format!("{FILE_NAME}: does not exist\n"),
    );
}

const ENTRY_POINT_ARG: &str = "samples.io.fileexample.FileExample";
const FILE_PATH_ARG: &str = "../tmp/file_example_test.txt";

static TEST_DIR_PATH: Lazy<PathBuf> = Lazy::new(|| create_path(&["tests", "test_data"]));
static DIR_PATH: Lazy<PathBuf> = Lazy::new(|| create_path(&["..", "tmp"]));
const FILE_NAME: &str = "file_example_test.txt";

static REPO_PATH_STR: Lazy<String> =
    Lazy::new(|| REPO_PATH.as_path().to_string_lossy().to_string());

const INFO_TEMPLATE: &str = r#"File info: {{FILE_PATH}}
Absolute path: {{ABSOLUTE_PATH}}
Canonical path: {{CANONICAL_PATH}}
Path: {{PATH}}
Name: {{NAME}}
Parent: {{PARENT}}
Parent file: {{PARENT_FILE}}
Is absolute: {{IS_ABSOLUTE}}
File exists: {{FILE_EXISTS}}
Is file: {{IS_FILE}}
Is directory: {{IS_DIRECTORY}}
Is hidden: {{IS_HIDDEN}}
Is writable: {{IS_WRITABLE}}
Is readable: {{IS_READABLE}}
Is executable: {{IS_EXECUTABLE}}
"#;

#[derive(new)]
struct TemplateValues {
    file_path: String,
    absolute_path: String,
    canonical_path: String,
    path: String,
    name: String,
    parent: String,
    parent_file: String,
    is_absolute: bool,
    file_exists: bool,
    is_file: bool,
    is_directory: bool,
    is_hidden: bool,
    is_writable: bool,
    is_readable: bool,
    is_executable: bool,
}

fn resolve_template(template_values: &TemplateValues) -> String {
    let result = INFO_TEMPLATE
        .replace("{{FILE_PATH}}", &template_values.file_path)
        .replace("{{ABSOLUTE_PATH}}", &template_values.absolute_path)
        .replace("{{CANONICAL_PATH}}", &template_values.canonical_path)
        .replace("{{PATH}}", &template_values.path)
        .replace("{{NAME}}", &template_values.name)
        .replace("{{PARENT}}", &template_values.parent)
        .replace("{{PARENT_FILE}}", &template_values.parent_file)
        .replace("{{IS_ABSOLUTE}}", &template_values.is_absolute.to_string())
        .replace("{{FILE_EXISTS}}", &template_values.file_exists.to_string())
        .replace("{{IS_FILE}}", &template_values.is_file.to_string())
        .replace(
            "{{IS_DIRECTORY}}",
            &template_values.is_directory.to_string(),
        )
        .replace("{{IS_HIDDEN}}", &template_values.is_hidden.to_string())
        .replace("{{IS_WRITABLE}}", &template_values.is_writable.to_string())
        .replace("{{IS_READABLE}}", &template_values.is_readable.to_string())
        .replace(
            "{{IS_EXECUTABLE}}",
            &template_values.is_executable.to_string(),
        );

    #[cfg(target_os = "windows")]
    let result = utils::to_windows(&result);

    result
}

fn create_path(args: &[&str]) -> PathBuf {
    args.iter().collect::<PathBuf>()
}

fn file_path() -> PathBuf {
    let mut file_path = DIR_PATH.clone();
    file_path.push(FILE_NAME);

    file_path
}

fn absolute_path() -> PathBuf {
    let mut absolute_path = PathBuf::from(&*REPO_PATH_STR);
    absolute_path.push(TEST_DIR_PATH.as_path());
    absolute_path.push(file_path());

    absolute_path
}
fn canonical_path() -> PathBuf {
    let absolute = absolute_path();

    Path::new(&absolute)
        .absolutize()
        .expect("Failed to normalize path")
        .to_path_buf()
}

fn to_string(path: &PathBuf) -> String {
    path.to_string_lossy().to_string()
}

#[cfg(unix)]
fn file_properties(path: &Path) -> std::io::Result<(bool, bool, bool, bool)> {
    use std::fs;
    use std::os::unix::fs::MetadataExt;
    let metadata = fs::metadata(path)?;
    let mode = metadata.mode();

    // Owner permissions
    let is_readable = mode & 0o400 != 0;
    let is_writable = mode & 0o200 != 0;
    let is_executable = mode & 0o100 != 0;

    let is_hidden = path.file_name().map_or(false, |name| {
        name.to_str().map_or(false, |s| s.starts_with("."))
    });

    Ok((is_hidden, is_writable, is_readable, is_executable))
}
#[cfg(windows)]
fn file_properties(_path: &Path) -> std::io::Result<(bool, bool, bool, bool)> {
    Ok((false, true, true, true))
}

struct CleanUpOnPanic;

impl Drop for CleanUpOnPanic {
    fn drop(&mut self) {
        if std::thread::panicking() {
            remove_file(canonical_path()).unwrap_or_else(|e| {
                eprintln!(
                    "Failed to remove file during cleanup: {} ({e})",
                    canonical_path().display()
                );
            });
        }
    }
}
