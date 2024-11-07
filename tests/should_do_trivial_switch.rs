mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_trivial_switch() {
    let last_frame_value = VM::run("samples.javacore.switches.trivial.SwitchExample").unwrap();
    assert_eq!(1300, get_int(last_frame_value))
}
