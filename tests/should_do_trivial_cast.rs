mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_trivial_cast() {
    let mut vm = setup();
    let last_frame_value = vm.run("samples.javacore.cast.trivial.TrivialCast").unwrap();
    assert_eq!(1337, get_int(last_frame_value))
}
