mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_trivial_reflection() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.reflection.trivial.TrivialReflection")
        .unwrap();
    assert_eq!(2578, get_int(last_frame_value))
}
