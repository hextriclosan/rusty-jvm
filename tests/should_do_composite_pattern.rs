mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_composite_pattern() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.inheritance.interfaces.compositepattern.CompositePattern")
        .unwrap();
    assert_eq!(700, get_int(last_frame_value))
}
