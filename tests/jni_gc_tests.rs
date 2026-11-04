mod utils;

use std::process::Output;
use std::time::Duration;

fn ensure_jni_test_lib_is_built() {
    let status = std::process::Command::new("cargo")
        .args(["build", "-p", "jni_test_lib"])
        .status()
        .expect("failed to build JNI test library");
    assert!(status.success(), "JNI test library build failed");
}

fn run_jni_gc_demo() -> Output {
    ensure_jni_test_lib_is_built();
    let library_path = format!("-Djava.library.path={}", env!("JNI_TEST_LIB_PATH"));
    let options = [
        "-Xmx64m",
        "-Xlog:gc",
        "-XX:GCAllocThreshold=128",
        "-Djava.system.class.loader=samples.gc.GcJniDemo$TestSystemClassLoader",
        library_path.as_str(),
    ];
    utils::get_command_with_vm_options(&options, "samples.gc.GcJniDemo", &[])
        .timeout(Duration::from_secs(180))
        .output()
        .expect("failed to launch JNI GC fixture")
}

#[test]
fn jni_reference_handoffs_and_buffer_leases_survive_collection() {
    let output = run_jni_gc_demo();
    assert!(
        output.status.success(),
        "VM failed: status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "find-class-subclass-ok\nreferences-ok\nstring-lease-ok\nlease-gc-ok\nrelease-modes-ok\njni-gc-ok\n"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.lines().any(|line| line.contains("[gc] Explicit:")),
        "expected explicit collection under JNI references: {stderr}"
    );
}
