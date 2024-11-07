mod utils;
use utils::get_double;
use vm::vm::VM;

#[test]
fn should_do_subtraction_with_doubles() {
    let last_frame_value = VM::run("samples.arithmetics.sub.doubles.SubDoubles").unwrap();
    assert_eq!(-8.76543211E200, get_double(last_frame_value))
}
