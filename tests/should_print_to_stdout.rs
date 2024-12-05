mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_print_to_stdout() {
    let last_frame_value = VM::run("samples.system.outexample.SystemOutExample").unwrap();
    assert_eq!(42, get_int(last_frame_value))
}
