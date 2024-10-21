mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_abstract_classes() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.inheritance.abstractclass.AbstractClass")
        .unwrap();
    assert_eq!(145, get_int(last_frame_value))
}
