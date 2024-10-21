mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_trivial_strings_cpool() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.javacore.strings.cpool.trivial.TrivialStringsCPool")
        .unwrap();
    assert_eq!(8, get_int(last_frame_value))
}
