mod utils;
use utils::get_int;
use utils::setup;

#[test]
fn should_do_native_call_on_system_array_copy() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.nativecall.system.NativeCallSystemArrayCopy")
        .unwrap();
    assert_eq!(15, get_int(last_frame_value))
}
