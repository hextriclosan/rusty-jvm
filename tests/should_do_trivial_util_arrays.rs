mod utils;
use utils::assert_success;

#[test]
fn should_do_trivial_util_arrays() {
    assert_success(
        "samples.javabase.util.arrays.trivial.TrivialUtilArrays",
        "9\n",
    );
}
