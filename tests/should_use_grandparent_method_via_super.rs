mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_use_grandparent_method_via_super() {
    let last_frame_value = VM::run("samples.inheritance.usegrandparentmethodviasuper.UseGrandParentMethodViaSuperExample").unwrap();
    assert_eq!(1337, get_int(last_frame_value))
}
