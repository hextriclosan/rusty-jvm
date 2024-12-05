mod utils;
use utils::assert_success;

#[test]
fn should_support_one_interface_extends_another() {
    assert_success(
        "samples.inheritance.interfaces.oneinterfaceextendsanother.OneInterfaceExtendsAnother",
        "-400\n",
    );
}
