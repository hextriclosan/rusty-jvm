mod utils;
use utils::get_long;
use vm::vm::VM;

#[test]
fn should_write_read_instance_fields_with_longs() {
    let last_frame_value =
        VM::run("samples.fields.instance.longs.InstanceFieldsUserLong").unwrap();
    assert_eq!(4_380_866_642_760, get_long(last_frame_value))
}
