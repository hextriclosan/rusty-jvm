mod utils;
use utils::assert_success;

#[test]
fn should_do_operations_with_doubles() {
    assert_success(
        "samples.arithmetics.operations.doubles.DoubleOperations",
        "2.8547008547008547E278\n",
    );
}
