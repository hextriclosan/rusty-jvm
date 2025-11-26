use assert_matches::assert_matches;
use rusty_jar::error::JarError;
use rusty_jar::jarfile::JarFile;
use std::path::PathBuf;

#[test]
fn should_return_error_if_file_does_not_exist() {
    let path = PathBuf::from("non_existing.jar");
    let result = JarFile::open(&path);

    assert_matches!(result, Err(JarError::Io { .. }));
}

#[test]
fn should_list_files_in_jar() {
    let path = PathBuf::from("tests/test_data/hello/target/hello-1.0.jar");
    let jar_file = JarFile::open(&path).unwrap();
    let mut file_names = jar_file.file_names();
    file_names.sort();

    let expected_file_names = vec![
        "META-INF/MANIFEST.MF".to_string(),
        "META-INF/maven/demo/hello/pom.properties".to_string(),
        "META-INF/maven/demo/hello/pom.xml".to_string(),
        "com/example/Hello.class".to_string(),
    ];
    assert_eq!(expected_file_names, file_names);
}

#[test]
fn should_read_file_from_jar() {
    let path = PathBuf::from("tests/test_data/hello/target/hello-1.0.jar");
    let mut jar_file = JarFile::open(&path).unwrap();
    let manifest_content = jar_file.content_by_name("META-INF/MANIFEST.MF").unwrap();
    let manifest_string = String::from_utf8_lossy(&manifest_content).replace("\r\n", "\n");
    let expected_manifest_string = r#"Manifest-Version: 1.0
Created-By: Maven JAR Plugin 3.3.0
Build-Jdk-Spec: 25
Main-Class: com.example.Hello

"#;
    assert_eq!(expected_manifest_string, manifest_string);
}

#[test]
fn should_return_error_if_entry_not_exist() {
    let path = PathBuf::from("tests/test_data/hello/target/hello-1.0.jar");
    let mut jar_file = JarFile::open(&path).unwrap();
    let manifest_content = jar_file.content_by_name("does/not/exist.txt");

    assert_matches!(manifest_content, Err(JarError::EntryNotFound { .. }));
}
