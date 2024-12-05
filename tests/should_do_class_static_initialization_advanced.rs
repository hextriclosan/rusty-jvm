mod utils;
use utils::assert_success;

#[test]
fn should_do_class_static_initialization_advanced() {
    assert_success(
        "samples.fields.staticinitialization.advanced.StaticInitializationAdvanced",
        "826\n",
    );
}
