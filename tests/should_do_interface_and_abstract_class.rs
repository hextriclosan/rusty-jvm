mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_interface_and_abstract_class() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.inheritance.interfaceandabstractclass.InterfaceAndAbstractClass")
        .unwrap();
    assert_eq!(36630, get_int(last_frame_value))
}
