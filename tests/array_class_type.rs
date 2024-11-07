mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_create_array_class_type() {
    let last_frame_value = VM::run("samples.reflection.trivial.ArrayClass").unwrap();
    assert_eq!(262143, get_int(last_frame_value))
}
