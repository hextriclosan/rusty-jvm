mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_trivial_properties() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.javabase.util.properties.trivial.PropertiesTrivial")
        .unwrap();
    assert_eq!(60, get_int(last_frame_value))
}
