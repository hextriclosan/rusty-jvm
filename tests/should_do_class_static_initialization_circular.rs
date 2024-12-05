mod utils;
use utils::assert_success;

#[test]
fn should_do_class_static_initialization_circular() {
    assert_success(
        "samples.fields.staticinitialization.circular.StaticInitializationCircular",
        "700\n",
    );
}
