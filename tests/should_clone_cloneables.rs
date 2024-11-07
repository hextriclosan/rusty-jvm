mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_clone_cloneables() {
    let last_frame_value = VM::run("samples.javacore.cloneable.trivial.CloneExample").unwrap();
    assert_eq!(511, get_int(last_frame_value))
}
