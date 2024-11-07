mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_trivial_reflection() {
    let last_frame_value = VM::run("samples.reflection.trivial.TrivialReflection").unwrap();
    assert_eq!(2578, get_int(last_frame_value))
}
