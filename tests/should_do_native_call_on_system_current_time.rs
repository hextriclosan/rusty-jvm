mod utils;
use crate::utils::get_output;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn should_do_native_call_on_system_current_time() {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let expected_millis = since_the_epoch.as_millis() as i64;

    let output = get_output("samples.nativecall.system.NativeCallSystemCurrentTimeMillis");
    let actual_millis: i64 = output.trim().parse().expect("Not a number");

    assert!((expected_millis..expected_millis + 2000).contains(&actual_millis))
}
