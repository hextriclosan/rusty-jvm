mod utils;
use utils::get_long;
use utils::setup;

#[test]
fn should_do_adding_with_longs() {
    let vm = setup();
    let last_frame_value = vm.run("samples.arithmetics.adder.longs.AdderLong").unwrap();
    assert_eq!(171798691900, get_long(last_frame_value))
}
