mod utils;
use utils::get_float;
use vm::vm::VM;

#[test]
fn should_do_subtraction_with_floats() {
    let last_frame_value = VM::run("samples.arithmetics.sub.floats.SubFloats").unwrap();
    assert_eq!(-998.9, get_float(last_frame_value))
}
