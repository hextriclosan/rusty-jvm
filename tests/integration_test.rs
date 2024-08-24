
use vm::vm::VM;

#[test]
fn should_do_adding() {
    let vm = VM::new("tests/test_data/Adder.class").unwrap();
    let last_frame_value = vm.run().unwrap();
    assert_eq!(55, last_frame_value.unwrap())
}

#[test]
fn should_do_subtraction() {
    let vm = VM::new("tests/test_data/Sub.class").unwrap();
    let last_frame_value = vm.run().unwrap();
    assert_eq!(-999, last_frame_value.unwrap())
}

#[test]
fn should_do_extreme_stack_operations() {
    let vm = VM::new("tests/test_data/ExtremeStackTest.class").unwrap();
    let last_frame_value = vm.run().unwrap();
    assert_eq!(454, last_frame_value.unwrap())
}

#[test]
fn should_do_calculate_fibonacci_iteratively() {
    let vm = VM::new("tests/test_data/FibonacciIterative.class").unwrap();
    let last_frame_value = vm.run().unwrap();
    assert_eq!(55, last_frame_value.unwrap())
}

#[test]
fn should_do_calculate_fibonacci_recursively() {
    let vm = VM::new("tests/test_data/FibonacciRecursive.class").unwrap();
    let last_frame_value = vm.run().unwrap();
    assert_eq!(55, last_frame_value.unwrap())
}
