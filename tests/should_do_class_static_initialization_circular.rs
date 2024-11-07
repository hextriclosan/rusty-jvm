mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_class_static_initialization_circular() {
    let last_frame_value =
        VM::run("samples.fields.staticinitialization.circular.StaticInitializationCircular")
            .unwrap();
    assert_eq!(700, get_int(last_frame_value))
}
