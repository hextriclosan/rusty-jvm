mod utils;
use utils::get_long;
use vm::vm::VM;

#[test]
fn should_do_extreme_stack_operations_with_longs() {
    let last_frame_value =
        VM::run("samples.arithmetics.extremestack.longs.ExtremeStackLong").unwrap();
    assert_eq!(454, get_long(last_frame_value))
}
