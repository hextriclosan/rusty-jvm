mod utils;
use utils::get_double;
use vm::vm::VM;

#[test]
fn should_do_operations_with_doubles() {
    let last_frame_value =
        VM::run("samples.arithmetics.operations.doubles.DoubleOperations").unwrap();
    assert_eq!(2.8547008547008547E278, get_double(last_frame_value))
}
