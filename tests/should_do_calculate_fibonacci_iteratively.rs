mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_calculate_fibonacci_iteratively() {
    let last_frame_value =
        VM::run("samples.arithmetics.fibonacci.iterative.FibonacciIterative").unwrap();
    assert_eq!(55, get_int(last_frame_value))
}
