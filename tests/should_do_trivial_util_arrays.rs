mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_trivial_util_arrays() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.javabase.util.arrays.trivial.TrivialUtilArrays")
        .unwrap();
    assert_eq!(9, get_int(last_frame_value))
}
