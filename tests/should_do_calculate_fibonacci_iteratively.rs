mod utils;
use utils::assert_success;

#[test]
fn should_do_calculate_fibonacci_iteratively() {
    assert_success(
        "samples.arithmetics.fibonacci.iterative.FibonacciIterative",
        "55\n",
    );
}
