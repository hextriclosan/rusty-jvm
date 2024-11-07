mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_call_object_methods() {
    let last_frame_value = VM::run("samples.javacore.object.trivial.ObjectMethods").unwrap();
    assert_eq!(1, get_int(last_frame_value))
}
