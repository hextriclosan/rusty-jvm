mod utils;
use utils::assert_success;

#[test]
fn should_do_subtraction_with_floats() {
    assert_success("samples.arithmetics.sub.floats.SubFloats", "-998.9\n");
}
