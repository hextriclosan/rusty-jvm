mod utils;
use utils::get_long;
use vm::vm::VM;

#[test]
fn should_support_one_interface_extends_another() {
    let last_frame_value = VM::run(
        "samples.inheritance.interfaces.oneinterfaceextendsanother.OneInterfaceExtendsAnother",
    )
    .unwrap();
    assert_eq!(-400, get_long(last_frame_value))
}
