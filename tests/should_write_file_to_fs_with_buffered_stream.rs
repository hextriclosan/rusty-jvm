mod utils;
use std::fs;
use vm::vm::VM;

#[test]
fn should_write_file_to_fs_with_buffered_stream() {
    let file_path = "../tmp/buffered_output.txt";
    VM::run("samples.io.bufferedoutputstreamchunkingexample.BufferedOutputStreamChunkingExample")
        .unwrap();

    assert!(fs::metadata(file_path).is_ok(), "File does not exist");
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    assert_eq!(
        content, "This is a test for BufferedOutputStream chunking.",
        "File content does not match"
    );
    fs::remove_file(file_path).expect("Failed to delete file");
}
