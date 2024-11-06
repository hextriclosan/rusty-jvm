mod utils;
use utils::get_double;
use utils::setup;

#[test]
fn should_do_subtraction_with_doubles() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.arithmetics.sub.doubles.SubDoubles")
        .unwrap();
    assert_eq!(-8.76543211E200, get_double(last_frame_value))
}
