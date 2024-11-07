mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_initialize_system_on_load() {
    let last_frame_value = VM::run("samples.system.load.SystemLoad").unwrap();
    assert_eq!(4321, get_int(last_frame_value))
}
