mod utils;
use utils::assert_success;

#[test]
fn should_call_object_methods() {
    assert_success("samples.javacore.object.trivial.ObjectMethods", "1\n");
}
