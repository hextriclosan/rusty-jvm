mod utils;
use utils::get_long;
use vm::vm::VM;

#[test]
fn should_do_subtraction_with_longs() {
    let last_frame_value = VM::run("samples.arithmetics.sub.longs.SubLongs").unwrap();
    assert_eq!(-1_000_000_000, get_long(last_frame_value))
}
