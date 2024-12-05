mod utils;
use utils::assert_success;

#[test]
fn should_write_read_instance_fields_with_longs() {
    assert_success(
        "samples.fields.instance.longs.InstanceFieldsUserLong",
        "4380866642760\n",
    );
}
