mod utils;

use crate::utils::{get_output_with_args, REPO_PATH};
use once_cell::sync::Lazy;
use path_absolutize::Absolutize;
use std::path::{Path, PathBuf};
use std::string::ToString;

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

impl TemplateValues {
    pub fn new(
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
    ) -> Self {
        Self {
            file_path,
            absolute_path,
            canonical_path,
            path,
            name,
            parent,
            parent_file,
            is_absolute,
            file_exists,
            is_file,
            is_directory,
            is_hidden,
            is_writable,
            is_readable,
            is_executable,
        }
    }
}

#[test]
fn should_support_java_io_file() {
    non_existing_file_info();
}

fn non_existing_file_info() {
    let output = get_output_with_args(ENTRY_POINT_ARG, &["--info", &FILE_PATH_ARG]);

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

    assert_eq!(output, expected_output);
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
