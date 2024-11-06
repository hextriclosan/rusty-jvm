mod utils;
use utils::get_double;
use utils::setup;

#[test]
fn should_do_operations_with_doubles() {
    let vm = setup();
    let last_frame_value = vm
        .run("samples.arithmetics.operations.doubles.DoubleOperations")
        .unwrap();
    assert_eq!(2.8547008547008547E278, get_double(last_frame_value))
}
