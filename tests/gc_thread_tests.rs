use std::process::Output;
use std::sync::Once;
use std::time::Duration;

static BUILD_JNI_TEST_LIBRARY: Once = Once::new();

fn run_fixture(class_name: &str) -> Output {
    run_fixture_with_args(class_name, &[])
}

fn run_fixture_with_args(class_name: &str, args: &[&str]) -> Output {
    BUILD_JNI_TEST_LIBRARY.call_once(|| {
        let output = assert_cmd::Command::new("cargo")
            .args(["build", "-p", "jni_test_lib"])
            .timeout(Duration::from_secs(180))
            .output()
            .expect("failed to build JNI test library");
        assert!(
            output.status.success(),
            "JNI test library build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    });

    let class_path = utils::TEST_PATH
        .canonicalize()
        .expect("compiled Java fixture directory is missing")
        .to_string_lossy()
        .into_owned();
    let library_path = format!("-Djava.library.path={}", env!("JNI_TEST_LIB_PATH"));
    let options = [
        "-Xmx64m",
        "-Xlog:gc",
        "-XX:GCAllocThreshold=1024",
        "-cp",
        &class_path,
        &library_path,
    ];
    utils::get_command_with_vm_options(&options, class_name, args)
        .timeout(Duration::from_secs(180))
        .output()
        .expect("failed to launch rusty-jvm fixture")
}

fn assert_collected(output: &Output, token: &str, minimum_reclaiming_cycles: usize) {
    assert!(
        output.status.success(),
        "VM failed: status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(token), "expected {token:?} in {stdout:?}");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let reclaiming_cycles = stderr
        .lines()
        .filter(|line| {
            line.contains("[gc] Explicit:")
                && line.contains("reclaimed=")
                && line
                    .split("reclaimed=")
                    .nth(1)
                    .and_then(|reclaimed| reclaimed.split_whitespace().next())
                    .and_then(|bytes| bytes.parse::<u64>().ok())
                    .is_some_and(|bytes| bytes > 0)
        })
        .count();
    assert!(
        reclaiming_cycles >= minimum_reclaiming_cycles,
        "expected {minimum_reclaiming_cycles} reclaiming cycles, saw {reclaiming_cycles}; stderr={stderr}"
    );
}

#[test]
fn java_root_survival_fixture_runs_real_reclaiming_collections() {
    assert_collected(&run_fixture("samples.gc.GcRootsDemo"), "roots-ok", 12);
}

#[test]
fn blocked_thread_and_class_loading_races_finish_under_gc() {
    assert_collected(&run_fixture("samples.gc.GcThreadsDemo"), "threads-ok", 2);
}

#[test]
fn concurrent_class_for_name_loading_survives_collection() {
    assert_collected(
        &run_fixture_with_args("samples.gc.GcThreadsDemo", &["class-loading"]),
        "class-loading-ok",
        1,
    );
}

#[test]
fn concurrent_cleaner_registration_survives_collection() {
    assert_collected(
        &run_fixture_with_args("samples.gc.GcThreadsDemo", &["cleaner"]),
        "cleaners-ok",
        1,
    );
}

mod utils;
