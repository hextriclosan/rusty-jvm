mod utils;
use std::fs;
use utils::assert_success;

#[test]
fn should_write_file_to_fs() {
    let file_path = "tests/tmp/test.txt";

    assert_success(
        "samples.io.fileoutputstreamexample.FileOutputStreamExample",
        "",
    );

    assert!(fs::metadata(file_path).is_ok(), "File does not exist");
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    assert_eq!(content, "CAFEBABE", "File content does not match");
    fs::remove_file(file_path).expect("Failed to delete file");
}
