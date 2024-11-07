mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_native_call_on_system_array_copy() {
    let last_frame_value = VM::run("samples.nativecall.system.NativeCallSystemArrayCopy").unwrap();
    assert_eq!(15, get_int(last_frame_value))
}
