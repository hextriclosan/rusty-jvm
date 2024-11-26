mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_trivial_unsafe_things() {
    let last_frame_value = VM::run("samples.jdkinternal.unsafe.trivial.UnsafeUsage").unwrap();
    assert_eq!(2097151, get_int(last_frame_value))
}
