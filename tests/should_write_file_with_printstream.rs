mod utils;
use std::fs;
use utils::assert_success;

#[test]
fn should_write_file_with_print_stream() {
    let file_path = "tests/tmp/print_stream_test.txt";

    assert_success("samples.io.printstreamexample.PrintStreamExample", "");

    assert!(fs::metadata(file_path).is_ok(), "File does not exist");
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    assert_eq!(
        content,
        r#"Hello, PrintStream!
First Line
Second Line
Third Line
Hello as raw bytes
This is written immediately. This follows after flush.
This is an example of chaining PrintStreams.
"#,
        "File content does not match"
    );
    fs::remove_file(file_path).expect("Failed to delete file");
}
