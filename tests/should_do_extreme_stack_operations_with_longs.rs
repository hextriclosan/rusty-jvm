mod utils;
use utils::get_long;
use utils::setup;

#[test]
fn should_do_extreme_stack_operations_with_longs() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.arithmetics.extremestack.longs.ExtremeStackLong")
        .unwrap();
    assert_eq!(454, get_long(last_frame_value))
}
