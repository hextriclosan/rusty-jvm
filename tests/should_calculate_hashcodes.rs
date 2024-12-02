mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_calculate_hashcodes() {
    let last_frame_value = VM::run("samples.javacore.hashcodes.trivial.HashCodeExample").unwrap();
    assert_eq!(255, get_int(last_frame_value))
}
