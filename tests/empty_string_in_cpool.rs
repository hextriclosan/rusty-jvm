mod utils;
use utils::assert_success;

#[test]
fn empty_string_in_cpool() {
    assert_success(
        "samples.javacore.strings.trivial.EmptyStringInCPool",
        "Empty string has length 0\n",
    );
}
