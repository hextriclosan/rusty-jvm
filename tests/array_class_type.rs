mod utils;
use utils::assert_success;

#[test]
fn should_create_array_class_type() {
    assert_success("samples.reflection.trivial.ArrayClass", "262143\n");
}
