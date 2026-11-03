mod utils;

use std::time::Duration;

#[test]
fn gc_reference_processing_and_cleaners_follow_jdk_semantics() {
    let output =
        utils::get_command_with_vm_options(&["-Xmx64m"], "samples.gc.GcReferencesDemo", &[])
            .timeout(Duration::from_secs(180))
            .output()
            .unwrap();

    assert_eq!(
        output.status.code(),
        Some(0),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout)
            .unwrap()
            .replace("\r\n", "\n"),
        "references-ok\n"
    );
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
