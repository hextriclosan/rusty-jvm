mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_deal_with_abstract_class_without_interface_implementation() {
    let last_frame_value = VM::run(
        "samples.inheritance.abstractclasswithoutimpl.AbstractClassWithoutInterfaceImplementation",
    )
    .unwrap();
    assert_eq!(2, get_int(last_frame_value))
}
