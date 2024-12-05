mod utils;
use utils::assert_success;

#[test]
fn should_do_inherited_static_fields() {
    assert_success(
        "samples.inheritance.staticfield.InheritanceStaticField",
        "128\n",
    );
}
