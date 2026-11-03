mod utils;

#[test]
fn gc_budget_bytecodes_reflection_clone_and_emergency_handlers() {
    utils::assert_success(
        "samples.gc.GcBudgetAllocationDemo",
        "negative-dimensions-ok\noversized-arrays-ok\nclone-ok\n",
    );
}

#[test]
fn gc_budget_too_small_limit_fails_emergency_bootstrap_explicitly() {
    let output =
        utils::get_command_with_vm_options(&["-Xmx1"], "samples.gc.GcBudgetAllocationDemo", &[])
            .output()
            .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("too small for emergency OutOfMemoryError bootstrap"),
        "{stderr}"
    );
}
