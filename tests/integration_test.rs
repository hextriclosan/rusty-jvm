use vm::vm::VM;

#[test]
fn should_do_adding() {
    let vm = VM::new(
        vec!["tests/test_data/samples/arithmetics/adder/ints/AdderInt.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("samples.arithmetics.adder.ints.AdderInt").unwrap();
    assert_eq!(55, get_int(last_frame_value))
}

#[test]
fn should_do_adding_with_longs() {
    let vm = VM::new(
        vec!["tests/test_data/samples/arithmetics/adder/longs/AdderLong.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("samples.arithmetics.adder.longs.AdderLong").unwrap();
    assert_eq!(171798691900, get_long(last_frame_value))
}

#[test]
fn should_do_adding_with_negative_longs() {
    let vm = VM::new(
        vec!["tests/test_data/samples/arithmetics/addernegative/AdderNegativeLong.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm
        .run("samples.arithmetics.addernegative.AdderNegativeLong")
        .unwrap();
    assert_eq!(-1990000000000000, get_long(last_frame_value))
}

#[test]
fn should_do_subtraction() {
    let vm = VM::new(
        vec!["tests/test_data/samples/arithmetics/sub/ints/SubInts.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("samples.arithmetics.sub.ints.SubInts").unwrap();
    assert_eq!(-999, get_int(last_frame_value))
}

#[test]
fn should_do_subtraction_with_longs() {
    let vm = VM::new(
        vec!["tests/test_data/samples/arithmetics/sub/longs/SubLongs.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("samples.arithmetics.sub.longs.SubLongs").unwrap();
    assert_eq!(-1_000_000_000, get_long(last_frame_value))
}

#[test]
fn should_write_read_instance_fields() {
    let vm = VM::new(
        vec![
            "tests/test_data/samples/fields/instance/ints/InstanceFieldsUserInts.class",
            "tests/test_data/samples/fields/instance/ints/InstanceFields.class",
        ],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm
        .run("samples.fields.instance.ints.InstanceFieldsUserInts")
        .unwrap();
    assert_eq!(110022, get_int(last_frame_value))
}

#[test]
fn should_write_read_instance_fields_with_longs() {
    let vm = VM::new(
        vec![
            "tests/test_data/samples/fields/instance/longs/InstanceFieldsUserLong.class",
            "tests/test_data/samples/fields/instance/longs/InstanceFields.class",
        ],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm
        .run("samples.fields.instance.longs.InstanceFieldsUserLong")
        .unwrap();
    assert_eq!(4_380_866_642_760, get_long(last_frame_value))
}

#[test]
fn should_write_read_static_fields() {
    let vm = VM::new(
        vec![
            "tests/test_data/samples/fields/staticinitialization/ints/StaticFieldsUserInts.class",
            "tests/test_data/samples/fields/staticinitialization/ints/StaticFields.class",
        ],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm
        .run("samples.fields.staticinitialization.ints.StaticFieldsUserInts")
        .unwrap();
    assert_eq!(110022, get_int(last_frame_value))
}

#[test]
fn should_do_extreme_stack_operations() {
    let vm = VM::new(
        vec!["tests/test_data/samples/arithmetics/extremestack/ints/ExtremeStackInt.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm
        .run("samples.arithmetics.extremestack.ints.ExtremeStackInt")
        .unwrap();
    assert_eq!(454, get_int(last_frame_value))
}

#[test]
fn should_do_extreme_stack_operations_with_longs() {
    let vm = VM::new(
        vec!["tests/test_data/samples/arithmetics/extremestack/longs/ExtremeStackLong.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm
        .run("samples.arithmetics.extremestack.longs.ExtremeStackLong")
        .unwrap();
    assert_eq!(454, get_long(last_frame_value))
}

#[test]
fn should_do_calculate_fibonacci_iteratively() {
    let vm = VM::new(
        vec!["tests/test_data/samples/arithmetics/fibonacci/iterative/FibonacciIterative.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm
        .run("samples.arithmetics.fibonacci.iterative.FibonacciIterative")
        .unwrap();
    assert_eq!(55, get_int(last_frame_value))
}

#[test]
fn should_do_calculate_fibonacci_recursively() {
    let vm = VM::new(
        vec!["tests/test_data/samples/arithmetics/fibonacci/recursive/FibonacciRecursive.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm
        .run("samples.arithmetics.fibonacci.recursive.FibonacciRecursive")
        .unwrap();
    assert_eq!(55, get_int(last_frame_value))
}

#[test]
fn should_do_arrays() {
    let vm = VM::new(
        vec!["tests/test_data/samples/arrays/array/ints/ArrayInt.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("samples.arrays.array.ints.ArrayInt").unwrap();
    assert_eq!(740, get_int(last_frame_value))
}

#[test]
fn should_do_arrays_with_longs() {
    let vm = VM::new(
        vec!["tests/test_data/samples/arrays/array/longs/ArrayLong.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("samples.arrays.array.longs.ArrayLong").unwrap();
    assert_eq!(233646220932000, get_long(last_frame_value))
}

#[test]
fn should_do_3d_arrays() {
    let vm = VM::new(
        vec!["tests/test_data/samples/arrays/array3d/Array3D.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm.run("samples.arrays.array3d.Array3D").unwrap();
    assert_eq!(780, get_int(last_frame_value))
}

#[test]
fn should_do_class_static_initialization() {
    let vm = VM::new(
        vec!["tests/test_data/samples/fields/staticinitialization/array/StaticInitializationArray.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm
        .run("samples.fields.staticinitialization.array.StaticInitializationArray")
        .unwrap();
    assert_eq!(257, get_int(last_frame_value))
}

#[test]
fn should_do_class_static_initialization_multiple_classes() {
    let vm = VM::new(
        vec![
            "tests/test_data/samples/fields/staticinitialization/chain/Dependable.class",
            "tests/test_data/samples/fields/staticinitialization/chain/DependsOnDependable.class",
            "tests/test_data/samples/fields/staticinitialization/chain/StaticInitializationChain.class",
        ],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm
        .run("samples.fields.staticinitialization.chain.StaticInitializationChain")
        .unwrap();
    assert_eq!(350, get_int(last_frame_value))
}

#[test]
fn should_do_class_static_initialization_within_one_class() {
    let vm = VM::new(
        vec!["tests/test_data/samples/fields/staticinitialization/oneclass/StaticInitializationWithinOneClass.class"],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm
        .run("samples.fields.staticinitialization.oneclass.StaticInitializationWithinOneClass")
        .unwrap();
    assert_eq!(100, get_int(last_frame_value))
}

#[test]
fn should_do_class_static_initialization_advanced() {
    let vm = VM::new(
        vec![
            "tests/test_data/samples/fields/staticinitialization/advanced/StaticInitializationAdvanced.class",
            "tests/test_data/samples/fields/staticinitialization/advanced/ClassC.class",
            "tests/test_data/samples/fields/staticinitialization/advanced/ClassD.class",
            "tests/test_data/samples/fields/staticinitialization/advanced/ClassE.class",
            "tests/test_data/samples/fields/staticinitialization/advanced/Helper.class",
        ],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm
        .run("samples.fields.staticinitialization.advanced.StaticInitializationAdvanced")
        .unwrap();
    assert_eq!(826, get_int(last_frame_value))
}

#[test]
fn should_do_class_static_initialization_circular() {
    let vm = VM::new(
        vec![
            "tests/test_data/samples/fields/staticinitialization/circular/StaticInitializationCircular.class",
            "tests/test_data/samples/fields/staticinitialization/circular/ClassACircular.class",
            "tests/test_data/samples/fields/staticinitialization/circular/ClassBCircular.class",
        ],
        "tests/test_data/std",
    )
    .unwrap();
    let last_frame_value = vm
        .run("samples.fields.staticinitialization.circular.StaticInitializationCircular")
        .unwrap();
    assert_eq!(700, get_int(last_frame_value))
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
