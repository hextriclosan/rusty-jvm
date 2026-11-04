mod utils;

use std::process::Output;

fn run(options: &[&str], entry: &str) -> Output {
    utils::get_command_with_vm_options(options, entry, &[])
        .output()
        .expect("failed to launch GC fixture")
}

fn output_text(output: &[u8]) -> String {
    String::from_utf8_lossy(output).replace("\r\n", "\n")
}

#[test]
fn gc_budget_enforces_array_clone_and_concurrent_reservations_with_recovery() {
    let status = std::process::Command::new("cargo")
        .args(["build", "-p", "jni_test_lib"])
        .status()
        .expect("failed to build JNI test library");
    assert!(status.success(), "JNI test library build failed");
    let library = format!("-Djava.library.path={}", env!("JNI_TEST_LIB_PATH"));
    let output = run(&["-Xmx64m", library.as_str()], "samples.gc.GcBudgetDemo");
    assert!(
        output.status.success(),
        "status={:?}\nstdout={}\nstderr={}",
        output.status,
        output_text(&output.stdout),
        output_text(&output.stderr)
    );
    let stdout = output_text(&output.stdout);
    assert!(stdout.starts_with("bootstrap-used="), "{stdout}");
    for path in [
        "reflective-array-oom-recovered",
        "jni-array-oom-recovered",
        "concurrent-oom-recovered",
        "clone-oom-recovered",
        "recovery-oom-recovered",
    ] {
        assert!(
            stdout.lines().any(|line| line == path),
            "missing {path}: {stdout}"
        );
    }
}

#[test]
fn gc_log_demo_logs_explicit_collection_only_when_requested() {
    let quiet = run(&[], "samples.gc.GcLogDemo");
    assert!(quiet.status.success(), "{}", output_text(&quiet.stderr));
    assert_eq!(output_text(&quiet.stdout), "gc-log-ok\n");
    assert!(
        quiet.stderr.is_empty(),
        "stderr: {}",
        output_text(&quiet.stderr)
    );

    let logged = run(&["-Xlog:gc"], "samples.gc.GcLogDemo");
    assert!(logged.status.success(), "{}", output_text(&logged.stderr));
    assert_eq!(output_text(&logged.stdout), "gc-log-ok\n");
    let stderr = output_text(&logged.stderr);
    assert!(
        stderr.lines().any(|line| line.starts_with("[gc]")),
        "{stderr}"
    );
}
