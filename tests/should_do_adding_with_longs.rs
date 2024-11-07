mod utils;
use utils::get_long;
use vm::vm::VM;

#[test]
fn should_do_adding_with_longs() {
    let last_frame_value = VM::run("samples.arithmetics.adder.longs.AdderLong").unwrap();
    assert_eq!(171798691900, get_long(last_frame_value))
}
