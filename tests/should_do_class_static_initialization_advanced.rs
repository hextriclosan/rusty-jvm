mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_class_static_initialization_advanced() {
    let last_frame_value =
        VM::run("samples.fields.staticinitialization.advanced.StaticInitializationAdvanced")
            .unwrap();
    assert_eq!(826, get_int(last_frame_value))
}
