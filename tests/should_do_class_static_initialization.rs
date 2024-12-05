mod utils;
use utils::assert_success;

#[test]
fn should_do_class_static_initialization() {
    assert_success(
        "samples.fields.staticinitialization.array.StaticInitializationArray",
        "257\n",
    );
}
