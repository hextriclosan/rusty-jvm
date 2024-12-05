mod utils;
use utils::assert_success;

#[test]
fn should_return_class_name() {
    assert_success(
        "samples.reflection.trivial.classgetname.ClassGetNameExample",
        "1\n",
    );
}
