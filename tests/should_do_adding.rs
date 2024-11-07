mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_adding() {
    let last_frame_value = VM::run("samples.arithmetics.adder.ints.AdderInt").unwrap();
    assert_eq!(55, get_int(last_frame_value))
}
