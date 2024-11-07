mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_byte_operations() {
    let last_frame_value = VM::run("samples.javacore.bytes.trivial.ByteOperations").unwrap();
    assert_eq!(0b111111111111111111111, get_int(last_frame_value))
}
