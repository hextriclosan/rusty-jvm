mod utils;
use crate::utils::get_int;
use utils::setup;

#[test]
fn should_convert_to_string_and_back() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.javacore.strings.trivial.ToStringAndBack")
        .unwrap();
    assert_eq!(255, get_int(last_frame_value))
}
