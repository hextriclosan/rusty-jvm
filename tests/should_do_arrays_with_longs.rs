mod utils;
use utils::get_long;
use vm::vm::VM;

#[test]
fn should_do_arrays_with_longs() {
    let last_frame_value = VM::run("samples.arrays.array.longs.ArrayLong").unwrap();
    assert_eq!(233646220932000, get_long(last_frame_value))
}
