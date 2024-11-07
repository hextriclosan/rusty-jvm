mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_abstract_classes() {
    let last_frame_value = VM::run("samples.inheritance.abstractclass.AbstractClass").unwrap();
    assert_eq!(145, get_int(last_frame_value))
}
