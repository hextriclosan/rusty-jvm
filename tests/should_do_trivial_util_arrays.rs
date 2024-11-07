mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_trivial_util_arrays() {
    let last_frame_value =
        VM::run("samples.javabase.util.arrays.trivial.TrivialUtilArrays").unwrap();
    assert_eq!(9, get_int(last_frame_value))
}
