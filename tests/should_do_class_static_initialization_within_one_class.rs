mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_class_static_initialization_within_one_class() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.fields.staticinitialization.oneclass.StaticInitializationWithinOneClass")
        .unwrap();
    assert_eq!(100, get_int(last_frame_value))
}
