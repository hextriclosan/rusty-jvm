mod utils;
use utils::get_long;
use utils::setup;

#[test]
fn should_support_one_interface_extends_another() {
    let mut vm = setup();
    let last_frame_value = vm
        .run(
            "samples.inheritance.interfaces.oneinterfaceextendsanother.OneInterfaceExtendsAnother",
        )
        .unwrap();
    assert_eq!(-400, get_long(last_frame_value))
}
