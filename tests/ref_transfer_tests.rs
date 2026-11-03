mod utils;

#[test]
fn ref_transfer_java_arguments_results_exceptions_and_allocations() {
    utils::assert_success(
        "samples.gc.RefTransfer",
        &format!("ref-transfer-ok{}", utils::line_ending()),
    );
}
