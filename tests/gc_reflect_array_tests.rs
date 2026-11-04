mod utils;

fn output_text(output: &[u8]) -> String {
    String::from_utf8_lossy(output).replace("\r\n", "\n")
}

#[test]
fn gc_reflect_array_native_semantics_match_jdk() {
    let output = utils::get_command_with_vm_options(
        &["-Xmx64m", "-XX:GCAllocThreshold=128"],
        "samples.gc.GcCyclesDemo",
        &["reflection"],
    )
    .output()
    .expect("failed to launch reflective Array fixture");
    assert!(
        output.status.success(),
        "status={:?}\nstdout={}\nstderr={}",
        output.status,
        output_text(&output.stdout),
        output_text(&output.stderr)
    );
    assert_eq!(output_text(&output.stdout), "reflect-array-ok\n");
    assert!(
        output.stderr.is_empty(),
        "stderr: {}",
        output_text(&output.stderr)
    );
}
