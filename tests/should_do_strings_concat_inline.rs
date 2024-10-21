mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_strings_concat_inline() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.javacore.strings.concat.trivial.StringConcatInline")
        .unwrap();
    assert_eq!(112788, get_int(last_frame_value))
}
