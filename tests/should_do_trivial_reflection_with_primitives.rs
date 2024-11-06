mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_trivial_reflection_with_primitives() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.reflection.trivial.synthetic.classes.SyntheticPrimitiveClasses")
        .unwrap();
    assert_eq!(9369, get_int(last_frame_value))
}
