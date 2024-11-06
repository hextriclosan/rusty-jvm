mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_class_static_initialization_multiple_classes() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.fields.staticinitialization.chain.StaticInitializationChain")
        .unwrap();
    assert_eq!(350, get_int(last_frame_value))
}
