mod utils;
use utils::get_long;
use utils::setup;

#[test]
fn should_do_arrays_with_longs() {
    let vm = setup();
    let last_frame_value = vm.run("samples.arrays.array.longs.ArrayLong").unwrap();
    assert_eq!(233646220932000, get_long(last_frame_value))
}
