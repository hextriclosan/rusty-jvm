mod utils;
use utils::assert_success;

#[test]
fn should_check_if_class_is_interface() {
    assert_success(
        "samples.reflection.trivial.classisinterface.ClassIsInterfaceExample",
        "1\n",
    );
}
