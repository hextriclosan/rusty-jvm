mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_class_static_initialization() {
    let last_frame_value =
        VM::run("samples.fields.staticinitialization.array.StaticInitializationArray").unwrap();
    assert_eq!(257, get_int(last_frame_value))
}
