mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_extreme_stack_operations() {
    let last_frame_value =
        VM::run("samples.arithmetics.extremestack.ints.ExtremeStackInt").unwrap();
    assert_eq!(528, get_int(last_frame_value))
}
