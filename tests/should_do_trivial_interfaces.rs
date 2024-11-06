mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_trivial_interfaces() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.inheritance.interfaces.trivial.TrivialInterface")
        .unwrap();
    assert_eq!(-200, get_int(last_frame_value))
}
