mod utils;
use utils::assert_success;

#[test]
fn should_deal_with_abstract_class_without_interface_implementation() {
    assert_success(
        "samples.inheritance.abstractclasswithoutimpl.AbstractClassWithoutInterfaceImplementation",
        "2\n",
    );
}
