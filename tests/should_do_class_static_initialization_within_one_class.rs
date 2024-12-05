mod utils;
use utils::assert_success;

#[test]
fn should_do_class_static_initialization_within_one_class() {
    assert_success(
        "samples.fields.staticinitialization.oneclass.StaticInitializationWithinOneClass",
        "100\n",
    );
}
