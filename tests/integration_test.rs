use vm::vm::VM;

#[test]
fn should_do_adding() {
    let vm = VM::new(vec!["tests/test_data/Adder.class"], "tests/test_data/std").unwrap();
    let last_frame_value = vm.run("Adder").unwrap();
    assert_eq!(55, get_int(last_frame_value))
}

#[test]
fn should_do_adding_with_longs() {
    let vm = VM::new(
        vec!["tests/test_data/AdderLong.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("AdderLong").unwrap();
    assert_eq!(171798691900, get_long(last_frame_value))
}

#[test]
fn should_do_adding_with_negative_longs() {
    let vm = VM::new(
        vec!["tests/test_data/AdderNegativeLong.class"],
        "tests/test_data/std",
    )
        .unwrap();
    let last_frame_value = vm.run("AdderNegativeLong").unwrap();
    assert_eq!(-1990000000000000, get_long(last_frame_value))
}

#[test]
fn should_do_subtraction() {
    let vm = VM::new(vec!["tests/test_data/Sub.class"], "tests/test_data/std").unwrap();
    let last_frame_value = vm.run("Sub").unwrap();
    assert_eq!(-999, get_int(last_frame_value))
}

#[test]
fn should_do_subtraction_with_longs() {
    let vm = VM::new(vec!["tests/test_data/SubLong.class"], "tests/test_data/std").unwrap();
    let last_frame_value = vm.run("SubLong").unwrap();
    assert_eq!(-1_000_000_000, get_long(last_frame_value))
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
    assert_eq!(110022, get_int(last_frame_value))
}

#[test]
fn should_write_read_instance_fields_with_longs() {
    let vm = VM::new(
        vec![
            "tests/test_data/InstanceFieldsUserLong.class",
            "tests/test_data/InstanceFieldsLong.class",
        ],
        "tests/test_data/std",
    )
        .unwrap();
    let last_frame_value = vm.run("InstanceFieldsUserLong").unwrap();
    assert_eq!(4_380_866_642_760, get_long(last_frame_value))
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
    assert_eq!(110022, get_int(last_frame_value))
}

#[test]
fn should_do_extreme_stack_operations() {
    let vm = VM::new(
        vec!["tests/test_data/ExtremeStackTest.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("ExtremeStackTest").unwrap();
    assert_eq!(454, get_int(last_frame_value))
}

#[test]
fn should_do_extreme_stack_operations_with_longs() {
    let vm = VM::new(
        vec!["tests/test_data/ExtremeStackTestLong.class"],
        "tests/test_data/std",
    )
        .unwrap();
    let last_frame_value = vm.run("ExtremeStackTestLong").unwrap();
    assert_eq!(454, get_long(last_frame_value))
}

#[test]
fn should_do_calculate_fibonacci_iteratively() {
    let vm = VM::new(
        vec!["tests/test_data/FibonacciIterative.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("FibonacciIterative").unwrap();
    assert_eq!(55, get_int(last_frame_value))
}

#[test]
fn should_do_calculate_fibonacci_recursively() {
    let vm = VM::new(
        vec!["tests/test_data/FibonacciRecursive.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("FibonacciRecursive").unwrap();
    assert_eq!(55, get_int(last_frame_value))
}

#[test]
fn should_do_arrays() {
    let vm = VM::new(vec!["tests/test_data/Array.class"], "tests/test_data/std").unwrap();
    let last_frame_value = vm.run("Array").unwrap();
    assert_eq!(740, get_int(last_frame_value))
}

#[test]
fn should_do_arrays_with_longs() {
    let vm = VM::new(vec!["tests/test_data/ArrayLong.class"], "tests/test_data/std").unwrap();
    let last_frame_value = vm.run("ArrayLong").unwrap();
    assert_eq!(233646220932000, get_long(last_frame_value))
}

#[test]
fn should_do_class_static_initialization() {
    let vm = VM::new(
        vec!["tests/test_data/StaticInitialization.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("StaticInitialization").unwrap();
    assert_eq!(257, get_int(last_frame_value))
}

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
    assert_eq!(350, get_int(last_frame_value))
}

fn get_int(locals: Option<Vec<i32>>) -> i32 {
    *locals.unwrap().last().unwrap()
}

fn get_long(locals_opt: Option<Vec<i32>>) -> i64 {
    let locals = locals_opt.unwrap();

    let two = &locals[locals.len().saturating_sub(2)..];
    let low = two[0];
    let high = two[1];

    ((high as i64) << 32) | (low as i64)
}
