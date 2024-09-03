use vm::vm::VM;

#[test]
fn should_do_adding() {
    let vm = VM::new("tests/test_data/Adder.class", "tests/test_data/std").unwrap();
    let last_frame_value = vm.run().unwrap();
    assert_eq!(55, last_frame_value.unwrap())
}

#[test]
fn should_do_subtraction() {
    let vm = VM::new("tests/test_data/Sub.class", "tests/test_data/std").unwrap();
    let last_frame_value = vm.run().unwrap();
    assert_eq!(-999, last_frame_value.unwrap())
}

#[test]
fn should_write_read_instance_fields() {
    let vm = VM::new(
        "tests/test_data/InstanceFields.class",
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run().unwrap();
    assert_eq!(11022, last_frame_value.unwrap())
}

#[test]
fn should_write_read_static_fields() {
    let vm = VM::new("tests/test_data/StaticFields.class", "tests/test_data/std").unwrap();
    let last_frame_value = vm.run().unwrap();
    assert_eq!(11022, last_frame_value.unwrap())
}

#[test]
fn should_do_extreme_stack_operations() {
    let vm = VM::new(
        "tests/test_data/ExtremeStackTest.class",
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run().unwrap();
    assert_eq!(454, last_frame_value.unwrap())
}

#[test]
fn should_do_calculate_fibonacci_iteratively() {
    let vm = VM::new(
        "tests/test_data/FibonacciIterative.class",
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run().unwrap();
    assert_eq!(55, last_frame_value.unwrap())
}

#[test]
fn should_do_calculate_fibonacci_recursively() {
    let vm = VM::new(
        "tests/test_data/FibonacciRecursive.class",
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run().unwrap();
    assert_eq!(55, last_frame_value.unwrap())
}
