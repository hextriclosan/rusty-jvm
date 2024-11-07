mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_arrays() {
    let last_frame_value = VM::run("samples.arrays.array.ints.ArrayInt").unwrap();
    assert_eq!(740, get_int(last_frame_value))
}
