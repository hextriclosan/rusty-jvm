mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_inherited_static_fields() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.inheritance.staticfield.InheritanceStaticField")
        .unwrap();
    assert_eq!(128, get_int(last_frame_value))
}
