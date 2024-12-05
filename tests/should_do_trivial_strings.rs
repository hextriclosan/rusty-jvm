mod utils;
use utils::assert_success;

#[test]
fn should_do_trivial_strings() {
    assert_success("samples.javacore.strings.trivial.TrivialStrings", "8\n");
}
