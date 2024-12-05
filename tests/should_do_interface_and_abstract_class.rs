mod utils;
use utils::assert_success;

#[test]
fn should_do_interface_and_abstract_class() {
    assert_success(
        "samples.inheritance.interfaceandabstractclass.InterfaceAndAbstractClass",
        "36630\n",
    );
}
