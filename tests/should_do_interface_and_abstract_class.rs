mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_interface_and_abstract_class() {
    let last_frame_value =
        VM::run("samples.inheritance.interfaceandabstractclass.InterfaceAndAbstractClass")
            .unwrap();
    assert_eq!(36630, get_int(last_frame_value))
}
