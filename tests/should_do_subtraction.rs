mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_subtraction() {
    let vm = setup();
    let last_frame_value = vm.run("samples.arithmetics.sub.ints.SubInts").unwrap();
    assert_eq!(-999, get_int(last_frame_value))
}
