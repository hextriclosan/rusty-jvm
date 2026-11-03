mod utils;

use std::collections::HashMap;
use std::process::Command;
use utils::ExecutionResult::Success;

#[test]
fn should_preserve_independent_jni_identities_and_native_handoffs() {
    let status = Command::new("cargo")
        .args(["build", "-p", "jni_test_lib"])
        .status()
        .expect("failed to build JNI test library");
    assert!(status.success());
    let library_path = format!("-Djava.library.path={}", env!("JNI_TEST_LIB_PATH"));
    utils::assert_with_all_args(
        &[&library_path],
        "samples.gc.GcJniIdentityDemo",
        &[],
        "aliases-ok\nframes-ok\ncapacity-ok\nbuffers-ok\ncallbacks-ok\n",
        "WARNING: A restricted method in java.lang.System has been called\n\
WARNING: java.lang.System::loadLibrary has been called by samples.gc.GcJniIdentityDemo in an unnamed module\n\
WARNING: Use --enable-native-access=ALL-UNNAMED to avoid a warning for callers in this module\n\
WARNING: Restricted methods will be blocked in a future release unless native access is enabled\n\n",
        Success,
        0,
        HashMap::new(),
    );
}
