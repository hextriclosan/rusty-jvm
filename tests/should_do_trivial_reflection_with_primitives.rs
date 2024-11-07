mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_trivial_reflection_with_primitives() {
    let last_frame_value =
        VM::run("samples.reflection.trivial.synthetic.classes.SyntheticPrimitiveClasses").unwrap();
    assert_eq!(9369, get_int(last_frame_value))
}
