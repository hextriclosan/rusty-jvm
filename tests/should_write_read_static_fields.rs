mod utils;
use utils::assert_success;

#[test]
fn should_write_read_static_fields() {
    assert_success(
        "samples.fields.staticinitialization.ints.StaticFieldsUserInts",
        "110022\n",
    );
}
