mod utils;

use std::time::Duration;

fn run(options: &[&str], mode: &str, stdout: &str, logging: bool) {
    // This fixture's full debug-build run performs over 11,000 collections
    // (measured at 177 seconds without competing test processes).
    // This is only a hang detector, not a throughput assertion.
    let deadline = if options.contains(&"-XX:+GcStressAtSafepoints") {
        Duration::from_secs(900)
    } else {
        Duration::from_secs(180)
    };
    let output = utils::get_command_with_vm_options(options, "samples.gc.GcCollectDemo", &[mode])
        .timeout(deadline)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout)
            .unwrap()
            .replace("\r\n", "\n"),
        stdout
    );
    if logging {
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr
                .lines()
                .any(|line| line.starts_with("[gc] Explicit:")),
            "{stderr}"
        );
        assert!(stderr.contains("reclaimed="), "{stderr}");
    } else {
        assert!(
            output.stderr.is_empty(),
            "stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn gc_collect_system_gc_reclaims_and_reports_budget_without_stderr() {
    run(&["-Xmx64m"], "basic", "collect-ok\n", false);
}

#[test]
fn gc_collect_logging_contains_literal_gc_in_message_text() {
    run(&["-Xmx64m", "-Xlog:gc"], "basic", "collect-ok\n", true);
}

#[test]
fn gc_collect_real_jdk_weak_phantom_soft_and_cleaner_delivery() {
    run(&["-Xmx64m"], "references", "references-ok\n", false);
}

#[test]
fn gc_collect_pressure_clears_soft_references_and_recovers_from_oom() {
    run(&["-Xmx16m"], "pressure", "pressure-ok\n", false);
}

#[test]
fn gc_collect_engine_and_native_stress_boundaries_keep_live_objects() {
    run(
        &[
            "-Xmx64m",
            "-XX:GCAllocThreshold=128",
            "-XX:+GcStressAtSafepoints",
        ],
        "basic",
        "collect-ok\n",
        false,
    );
}
