mod utils;

use utils::get_command_with_vm_options;

const ENTRY_POINT: &str = "samples.system.programargs.ProgramArgsExample";

#[test]
fn should_reject_invalid_heap_option_before_entry() {
    let output = get_command_with_vm_options(&["-Xmx12q"], ENTRY_POINT, &[])
        .output()
        .expect("Failed to run rusty-jvm");

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");
    assert!(stderr.contains("-Xmx12q"), "unexpected stderr: {stderr}");
}

#[test]
fn should_pass_heap_like_argument_after_entry_to_java() {
    let output = get_command_with_vm_options(&[], ENTRY_POINT, &["-Xmx12q"])
        .output()
        .expect("Failed to run rusty-jvm");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "[-Xmx12q]\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn should_accept_valid_heap_option_before_entry() {
    let output = get_command_with_vm_options(&["-Xmx64m"], ENTRY_POINT, &[])
        .output()
        .expect("Failed to run rusty-jvm");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "[]\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn should_accept_gc_logging_option_before_entry_without_emitting_a_fake_event() {
    let output = get_command_with_vm_options(&["-Xlog:gc"], ENTRY_POINT, &[])
        .output()
        .expect("Failed to run rusty-jvm");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "[]\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn should_accept_gc_advanced_options_before_entry() {
    let output = get_command_with_vm_options(
        &["-XX:GCAllocThreshold=1", "-XX:+GcStressAtSafepoints"],
        ENTRY_POINT,
        &[],
    )
    .output()
    .expect("Failed to run rusty-jvm");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "[]\n");
    assert!(output.stderr.is_empty());
}
