mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_arrays() {
    let mut vm = setup();
    let last_frame_value = vm.run("samples.arrays.array.ints.ArrayInt").unwrap();
    assert_eq!(740, get_int(last_frame_value))
}
