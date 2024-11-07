mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_wide_instructions() {
    let last_frame_value =
        VM::run("samples.javacore.wide.instructions.trivial.WideInstructionsExample").unwrap();
    assert_eq!(31, get_int(last_frame_value))
}
