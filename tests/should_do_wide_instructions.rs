mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_wide_instructions() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.javacore.wide.instructions.trivial.WideInstructionsExample")
        .unwrap();
    assert_eq!(31, get_int(last_frame_value))
}
