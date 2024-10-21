mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_adding() {
    let mut vm = setup();
    let last_frame_value = vm.run("samples.arithmetics.adder.ints.AdderInt").unwrap();
    assert_eq!(55, get_int(last_frame_value))
}
