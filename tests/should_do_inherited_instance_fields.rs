mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_inherited_instance_fields() {
    let last_frame_value =
        VM::run("samples.inheritance.instancefield.InheritanceInstanceField").unwrap();
    assert_eq!(128, get_int(last_frame_value))
}
