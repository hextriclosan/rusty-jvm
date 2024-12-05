mod utils;
use utils::assert_success;

#[test]
fn should_do_class_static_initialization_multiple_classes() {
    assert_success(
        "samples.fields.staticinitialization.chain.StaticInitializationChain",
        "350\n",
    );
}
