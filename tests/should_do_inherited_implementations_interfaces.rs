mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_inherited_implementations_interfaces() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.inheritance.interfaces.inheritedimplementation.InheritedImplementationInterface")
        .unwrap();
    assert_eq!(-200, get_int(last_frame_value))
}
