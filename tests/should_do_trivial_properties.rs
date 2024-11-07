mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_trivial_properties() {
    let last_frame_value =
        VM::run("samples.javabase.util.properties.trivial.PropertiesTrivial").unwrap();
    assert_eq!(60, get_int(last_frame_value))
}
