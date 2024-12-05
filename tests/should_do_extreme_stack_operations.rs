mod utils;
use utils::assert_success;

#[test]
fn should_do_extreme_stack_operations() {
    assert_success(
        "samples.arithmetics.extremestack.ints.ExtremeStackInt",
        "528\n",
    );
}
