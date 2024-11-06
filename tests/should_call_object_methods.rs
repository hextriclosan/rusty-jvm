mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_call_object_methods() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.javacore.object.trivial.ObjectMethods")
        .unwrap();
    assert_eq!(1, get_int(last_frame_value))
}
