mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_check_if_class_is_interface() {
    let last_frame_value =
        VM::run("samples.reflection.trivial.classisinterface.ClassIsInterfaceExample").unwrap();
    assert_eq!(1, get_int(last_frame_value))
}
