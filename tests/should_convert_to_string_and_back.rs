mod utils;
use utils::assert_success;

#[test]
fn should_convert_to_string_and_back() {
    assert_success("samples.javacore.strings.trivial.ToStringAndBack", "255\n");
}
