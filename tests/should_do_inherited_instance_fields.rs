mod utils;
use utils::assert_success;

#[test]
fn should_do_inherited_instance_fields() {
    assert_success(
        "samples.inheritance.instancefield.InheritanceInstanceField",
        "128\n",
    );
}
