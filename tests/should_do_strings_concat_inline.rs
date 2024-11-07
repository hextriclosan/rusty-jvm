mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_strings_concat_inline() {
    let last_frame_value =
        VM::run("samples.javacore.strings.concat.trivial.StringConcatInline").unwrap();
    assert_eq!(112788, get_int(last_frame_value))
}
