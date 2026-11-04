mod utils;

use std::process::Output;

fn run(options: &[&str], entry: &str, args: &[&str]) -> Output {
    utils::get_command_with_vm_options(options, entry, args)
        .output()
        .expect("failed to launch GC stress fixture")
}

fn output_text(output: &[u8]) -> String {
    String::from_utf8_lossy(output).replace("\r\n", "\n")
}

#[test]
fn gc_stress_fixed_live_graph_survives_increasing_lifetime_keys() {
    let cycles = if utils::is_bigendian() { 12 } else { 48 };
    let cycle_argument = cycles.to_string();
    let output = run(
        &["-Xmx64m", "-XX:GCAllocThreshold=128"],
        "samples.gc.GcCyclesDemo",
        &[cycle_argument.as_str()],
    );
    assert!(
        output.status.success(),
        "status={:?}\nstdout={}\nstderr={}",
        output.status,
        output_text(&output.stdout),
        output_text(&output.stderr)
    );
    let stdout = output_text(&output.stdout);
    let observed = stdout
        .lines()
        .filter(|line| line.starts_with("cycle="))
        .count();
    assert_eq!(observed, cycles, "{stdout}");
    assert!(stdout.ends_with("cycles-ok\n"), "{stdout}");
    assert!(
        output.stderr.is_empty(),
        "stderr: {}",
        output_text(&output.stderr)
    );
}

#[test]
fn gc_stress_nested_jni_pointer_returns_and_exceptions_survive() {
    let status = std::process::Command::new("cargo")
        .args(["build", "-p", "jni_test_lib"])
        .status()
        .expect("failed to build JNI test library");
    assert!(status.success(), "JNI test library build failed");
    let library = format!("-Djava.library.path={}", env!("JNI_TEST_LIB_PATH"));
    let output = run(
        &[
            "-Xmx64m",
            "-XX:GCAllocThreshold=128",
            "-Djava.system.class.loader=samples.gc.GcJniDemo$TestSystemClassLoader",
            library.as_str(),
        ],
        "samples.gc.GcJniDemo",
        &[],
    );
    assert!(
        output.status.success(),
        "status={:?}\nstdout={}\nstderr={}",
        output.status,
        output_text(&output.stdout),
        output_text(&output.stderr)
    );
    assert!(output_text(&output.stdout).ends_with("jni-gc-ok\n"));
}
