mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_extreme_stack_operations() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.arithmetics.extremestack.ints.ExtremeStackInt")
        .unwrap();
    assert_eq!(528, get_int(last_frame_value))
}
