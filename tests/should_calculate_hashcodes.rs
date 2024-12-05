mod utils;
use utils::assert_success;

#[test]
fn should_calculate_hashcodes() {
    assert_success(
        "samples.javacore.hashcodes.trivial.HashCodeExample",
        "255\n",
    );
}
