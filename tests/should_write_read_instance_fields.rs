mod utils;
use utils::assert_success;

#[test]
fn should_write_read_instance_fields() {
    assert_success(
        "samples.fields.instance.ints.InstanceFieldsUserInts",
        "110022\n",
    );
}
