mod utils;
use utils::assert_success;

#[test]
fn should_do_native_call_on_system_array_copy() {
    assert_success(
        "samples.nativecall.system.NativeCallSystemArrayCopy",
        "15\n",
    );
}
