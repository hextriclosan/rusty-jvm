mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_create_array_class_type() {
    let mut vm = setup();
    let last_frame_value = vm.run("samples.reflection.trivial.ArrayClass").unwrap();
    assert_eq!(262143, get_int(last_frame_value))
}
