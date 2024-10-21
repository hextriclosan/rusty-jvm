mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_strings_cpool_advanced() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.javacore.strings.cpool.advanced.StringPoolAdvanced")
        .unwrap();
    assert_eq!(127, get_int(last_frame_value))
}
