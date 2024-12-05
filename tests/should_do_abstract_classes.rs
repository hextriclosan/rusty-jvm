mod utils;
use utils::assert_success;

#[test]
fn should_do_abstract_classes() {
    assert_success("samples.inheritance.abstractclass.AbstractClass", "145\n");
}
