mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_calculate_fibonacci_recursively() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.arithmetics.fibonacci.recursive.FibonacciRecursive")
        .unwrap();
    assert_eq!(55, get_int(last_frame_value))
}
