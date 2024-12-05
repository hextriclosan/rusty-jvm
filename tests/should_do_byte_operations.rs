mod utils;
use utils::assert_success;

#[test]
fn should_do_byte_operations() {
    assert_success("samples.javacore.bytes.trivial.ByteOperations", "2097151\n");
}
