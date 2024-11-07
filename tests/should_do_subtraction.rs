mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_subtraction() {
    let last_frame_value = VM::run("samples.arithmetics.sub.ints.SubInts").unwrap();
    assert_eq!(-999, get_int(last_frame_value))
}
