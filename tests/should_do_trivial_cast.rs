mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_trivial_cast() {
    let last_frame_value = VM::run("samples.javacore.cast.trivial.TrivialCast").unwrap();
    assert_eq!(1337, get_int(last_frame_value))
}
