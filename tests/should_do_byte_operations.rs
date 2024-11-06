mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_byte_operations() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.javacore.bytes.trivial.ByteOperations")
        .unwrap();
    assert_eq!(0b111111111111111111111, get_int(last_frame_value))
}
