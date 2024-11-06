mod utils;
use utils::get_long;
use utils::setup;
#[test]
fn should_do_adding_with_negative_longs() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.arithmetics.addernegative.AdderNegativeLong")
        .unwrap();
    assert_eq!(-1990000000000000, get_long(last_frame_value))
}
