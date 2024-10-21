mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_class_static_initialization_circular() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.fields.staticinitialization.circular.StaticInitializationCircular")
        .unwrap();
    assert_eq!(700, get_int(last_frame_value))
}
