mod utils;

use std::time::{SystemTime, UNIX_EPOCH};
use utils::get_long;
use utils::setup;

#[test]
fn should_do_native_call_on_system_current_time() {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let expected_millis = since_the_epoch.as_millis() as i64;
    let vm = setup();
    let last_frame_value = vm
        .run("samples.nativecall.system.NativeCallSystemCurrentTimeMillis")
        .unwrap();
    let actual_millis = get_long(last_frame_value);
    assert!((expected_millis..expected_millis + 2000).contains(&actual_millis))
}
