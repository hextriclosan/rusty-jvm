mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_unsafe_object_field_offset() {
    let last_frame_value =
        VM::run("samples.jdkinternal.unsafe.objectfieldoffset.UnsafeObjectFieldOffset").unwrap();
    assert_eq!(127, get_int(last_frame_value))
}
