mod utils;
use crate::utils::get_int;
use vm::vm::VM;

#[test]
fn should_convert_to_string_and_back() {
    let last_frame_value =
        VM::run("samples.javacore.doubles.trivial.LongToDoubleAndBack").unwrap();
    assert_eq!(2, get_int(last_frame_value))
}
