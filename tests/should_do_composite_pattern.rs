mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_composite_pattern() {
    let last_frame_value =
        VM::run("samples.inheritance.interfaces.compositepattern.CompositePattern").unwrap();
    assert_eq!(700, get_int(last_frame_value))
}
