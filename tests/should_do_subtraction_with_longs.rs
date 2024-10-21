mod utils;
use utils::get_long;
use utils::setup;

#[test]
fn should_do_subtraction_with_longs() {
    let mut vm = setup();
    let last_frame_value = vm.run("samples.arithmetics.sub.longs.SubLongs").unwrap();
    assert_eq!(-1_000_000_000, get_long(last_frame_value))
}
