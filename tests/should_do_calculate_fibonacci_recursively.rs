mod utils;
use utils::assert_success;

#[test]
fn should_do_calculate_fibonacci_recursively() {
    assert_success(
        "samples.arithmetics.fibonacci.recursive.FibonacciRecursive",
        "55\n",
    );
}
