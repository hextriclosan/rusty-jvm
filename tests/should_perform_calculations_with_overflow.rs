mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_perform_calculations_with_overflow() {
    let last_frame_value = VM::run("samples.arithmetics.overflow.ArithmeticOverflow").unwrap();
    assert_eq!(1, get_int(last_frame_value))
}
