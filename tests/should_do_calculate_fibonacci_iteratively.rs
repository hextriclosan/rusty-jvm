mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_calculate_fibonacci_iteratively() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.arithmetics.fibonacci.iterative.FibonacciIterative")
        .unwrap();
    assert_eq!(55, get_int(last_frame_value))
}
