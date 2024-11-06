mod utils;
use utils::get_long;
use utils::setup;

#[test]
fn should_write_read_instance_fields_with_longs() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.fields.instance.longs.InstanceFieldsUserLong")
        .unwrap();
    assert_eq!(4_380_866_642_760, get_long(last_frame_value))
}
