mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_class_static_initialization_within_one_class() {
    let last_frame_value =
        VM::run("samples.fields.staticinitialization.oneclass.StaticInitializationWithinOneClass")
            .unwrap();
    assert_eq!(100, get_int(last_frame_value))
}
