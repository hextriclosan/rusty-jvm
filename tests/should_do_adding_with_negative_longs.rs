mod utils;
use utils::get_long;
use vm::vm::VM;

#[test]
fn should_do_adding_with_negative_longs() {
    let last_frame_value = VM::run("samples.arithmetics.addernegative.AdderNegativeLong").unwrap();
    assert_eq!(-1990000000000000, get_long(last_frame_value))
}
