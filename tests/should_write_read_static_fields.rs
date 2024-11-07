mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_write_read_static_fields() {
    let last_frame_value =
        VM::run("samples.fields.staticinitialization.ints.StaticFieldsUserInts").unwrap();
    assert_eq!(110022, get_int(last_frame_value))
}
