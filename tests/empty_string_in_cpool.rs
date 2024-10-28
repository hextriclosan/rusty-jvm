mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn empty_string_in_cpool() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.javacore.strings.trivial.EmptyStringInCPool")
        .unwrap();
    assert_eq!(0, get_int(last_frame_value))
}
