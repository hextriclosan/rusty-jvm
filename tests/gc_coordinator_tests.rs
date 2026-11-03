mod utils;

#[test]
fn gc_coordinator_java_class_initialization_and_failure() {
    utils::assert_success(
        "samples.gc.GcCoordinatorInitDemo",
        "recursive-ok\ncontended-ok\nfailure-ok\n",
    );
}
