mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_write_read_instance_fields() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.fields.instance.ints.InstanceFieldsUserInts")
        .unwrap();
    assert_eq!(110022, get_int(last_frame_value))
}
