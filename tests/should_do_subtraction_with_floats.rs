mod utils;
use utils::get_float;
use utils::setup;

#[test]
fn should_do_subtraction_with_floats() {
    let mut vm = setup();
    let last_frame_value = vm.run("samples.arithmetics.sub.floats.SubFloats").unwrap();
    assert_eq!(-998.9, get_float(last_frame_value))
}
