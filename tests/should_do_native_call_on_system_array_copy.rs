mod utils;
use utils::get_long;
use utils::setup;

#[test]
#[ignore]
fn should_do_native_call_on_system_array_copy() {
    let mut vm = setup();
    let last_frame_value = vm
        .run("samples.nativecall.system.NativeCallSystemArrayCopy")
        .unwrap();
    assert_eq!(644_245_094_908, get_long(last_frame_value))
}
