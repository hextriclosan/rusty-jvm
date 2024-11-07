mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_trivial_interfaces() {
    let last_frame_value =
        VM::run("samples.inheritance.interfaces.trivial.TrivialInterface").unwrap();
    assert_eq!(-200, get_int(last_frame_value))
}
