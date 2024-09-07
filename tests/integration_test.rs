use vm::vm::VM;

#[test]
fn should_do_adding() {
    let vm = VM::new(vec!["tests/test_data/Adder.class"], "tests/test_data/std").unwrap();
    let last_frame_value = vm.run("Adder").unwrap();
    assert_eq!(55, last_frame_value.unwrap())
}

#[test]
fn should_do_subtraction() {
    let vm = VM::new(vec!["tests/test_data/Sub.class"], "tests/test_data/std").unwrap();
    let last_frame_value = vm.run("Sub").unwrap();
    assert_eq!(-999, last_frame_value.unwrap())
}

#[test]
fn should_write_read_instance_fields() {
    let vm = VM::new(
        vec![
            "tests/test_data/InstanceFieldsUser.class",
            "tests/test_data/InstanceFields.class",
        ],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("InstanceFieldsUser").unwrap();
    assert_eq!(110022, last_frame_value.unwrap())
}

#[test]
fn should_write_read_static_fields() {
    let vm = VM::new(
        vec![
            "tests/test_data/StaticFieldsUser.class",
            "tests/test_data/StaticFields.class",
        ],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("StaticFieldsUser").unwrap();
    assert_eq!(110022, last_frame_value.unwrap())
}

#[test]
fn should_do_extreme_stack_operations() {
    let vm = VM::new(
        vec!["tests/test_data/ExtremeStackTest.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("ExtremeStackTest").unwrap();
    assert_eq!(454, last_frame_value.unwrap())
}

#[test]
fn should_do_calculate_fibonacci_iteratively() {
    let vm = VM::new(
        vec!["tests/test_data/FibonacciIterative.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("FibonacciIterative").unwrap();
    assert_eq!(55, last_frame_value.unwrap())
}

#[test]
fn should_do_calculate_fibonacci_recursively() {
    let vm = VM::new(
        vec!["tests/test_data/FibonacciRecursive.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("FibonacciRecursive").unwrap();
    assert_eq!(55, last_frame_value.unwrap())
}

#[test]
fn should_do_arrays() {
    let vm = VM::new(vec!["tests/test_data/Array.class"], "tests/test_data/std").unwrap();
    let last_frame_value = vm.run("Array").unwrap();
    assert_eq!(740, last_frame_value.unwrap())
}

#[test]
fn should_do_class_static_initialization() {
    let vm = VM::new(
        vec!["tests/test_data/StaticInitialization.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("StaticInitialization").unwrap();
    assert_eq!(257, last_frame_value.unwrap())
}

#[ignore]
#[test]
fn should_do_class_static_initialization_multiple_classes() {
    let vm = VM::new(
        vec![
            "tests/test_data/Dependable.class",
            "tests/test_data/DependsOnDependable.class",
            "tests/test_data/StaticInitializationUser.class",
        ],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("StaticInitializationUser").unwrap();
    assert_eq!(350, last_frame_value.unwrap())
}
