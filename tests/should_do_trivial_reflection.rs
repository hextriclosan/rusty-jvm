mod utils;
use utils::assert_success;

#[test]
fn should_do_trivial_reflection() {
    assert_success("samples.reflection.trivial.TrivialReflection", "2578\n");
}
