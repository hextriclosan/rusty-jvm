mod utils;
use utils::assert_success;

#[test]
fn should_do_inherited_implementations_interfaces() {
    assert_success(
        "samples.inheritance.interfaces.inheritedimplementation.InheritedImplementationInterface",
        "-200\n",
    );
}
