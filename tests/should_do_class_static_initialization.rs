mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_class_static_initialization() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.fields.staticinitialization.array.StaticInitializationArray")
        .unwrap();
    assert_eq!(257, get_int(last_frame_value))
}
