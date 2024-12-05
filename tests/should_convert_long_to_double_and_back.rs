mod utils;
use utils::assert_success;

#[test]
fn should_convert_to_string_and_back() {
    assert_success(
        "samples.javacore.doubles.trivial.LongToDoubleAndBack",
        "2\n",
    );
}
