mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_class_static_initialization_advanced() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.fields.staticinitialization.advanced.StaticInitializationAdvanced")
        .unwrap();
    assert_eq!(826, get_int(last_frame_value))
}
