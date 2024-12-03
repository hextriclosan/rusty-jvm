mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_return_class_name() {
    let last_frame_value =
        VM::run("samples.reflection.trivial.classgetname.ClassGetNameExample").unwrap();
    assert_eq!(1, get_int(last_frame_value))
}
