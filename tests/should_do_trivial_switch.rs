mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_trivial_switch() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.javacore.switches.trivial.SwitchExample")
        .unwrap();
    assert_eq!(300, get_int(last_frame_value))
}
