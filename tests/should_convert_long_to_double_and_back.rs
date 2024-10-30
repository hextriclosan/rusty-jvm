mod utils;
use crate::utils::get_int;
use utils::setup;

#[test]
fn should_convert_to_string_and_back() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.javacore.doubles.trivial.LongToDoubleAndBack")
        .unwrap();
    assert_eq!(2, get_int(last_frame_value))
}
