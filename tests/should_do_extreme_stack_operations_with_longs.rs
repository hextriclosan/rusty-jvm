mod utils;
use utils::assert_success;

#[test]
fn should_do_extreme_stack_operations_with_longs() {
    assert_success(
        "samples.arithmetics.extremestack.longs.ExtremeStackLong",
        "454\n",
    );
}
