mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn empty_string_in_cpool() {
    let last_frame_value = VM::run("samples.javacore.strings.trivial.EmptyStringInCPool").unwrap();
    assert_eq!(0, get_int(last_frame_value))
}
