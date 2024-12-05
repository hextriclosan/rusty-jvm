mod utils;
use utils::assert_success;

#[test]
fn should_do_trivial_strings_cpool() {
    assert_success(
        "samples.javacore.strings.cpool.trivial.TrivialStringsCPool",
        "8\n",
    );
}
