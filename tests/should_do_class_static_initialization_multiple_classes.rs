mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_class_static_initialization_multiple_classes() {
    let last_frame_value =
        VM::run("samples.fields.staticinitialization.chain.StaticInitializationChain").unwrap();
    assert_eq!(350, get_int(last_frame_value))
}
