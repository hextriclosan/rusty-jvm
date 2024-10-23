mod utils;
use crate::utils::get_long;
use utils::setup;

#[test]
fn should_convert_to_string_and_back() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.javacore.strings.trivial.ToStringAndBack")
        .unwrap();
    assert_eq!(1002000033239, get_long(last_frame_value))
}
