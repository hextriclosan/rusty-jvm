mod utils;
use utils::get_int;
use vm::vm::VM;

#[test]
fn should_do_strings_cpool_advanced() {
    let last_frame_value =
        VM::run("samples.javacore.strings.cpool.advanced.StringPoolAdvanced").unwrap();
    assert_eq!(127, get_int(last_frame_value))
}
