mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_trivial_strings_cpool() {
    let last_frame_value =
        VM::run("samples.javacore.strings.cpool.trivial.TrivialStringsCPool").unwrap();
    assert_eq!(8, get_int(last_frame_value))
}
