use ctor::ctor;
use std::env;
use std::sync::Once;
use vm::vm::VM;

static INIT: Once = Once::new();

const PATH: &str = "tests/test_data";

#[ctor]
fn setup() {
    INIT.call_once(|| {
        env::set_current_dir(PATH).expect("Failed to change working directory");
    })
}

#[test]
fn should_do_adding() {
    let mut vm = VM::new("std");
    let last_frame_value = vm.run("samples.arithmetics.adder.ints.AdderInt").unwrap();
    assert_eq!(55, get_int(last_frame_value))
}

#[test]
fn should_do_adding_with_longs() {
    let mut vm = VM::new("std");
    let last_frame_value = vm.run("samples.arithmetics.adder.longs.AdderLong").unwrap();
    assert_eq!(171798691900, get_long(last_frame_value))
}

#[test]
fn should_do_adding_with_negative_longs() {
    let mut vm = VM::new("std");
    let last_frame_value = vm
        .run("samples.arithmetics.addernegative.AdderNegativeLong")
        .unwrap();
    assert_eq!(-1990000000000000, get_long(last_frame_value))
}

#[test]
fn should_do_subtraction() {
    let mut vm = VM::new("std");
    let last_frame_value = vm.run("samples.arithmetics.sub.ints.SubInts").unwrap();
    assert_eq!(-999, get_int(last_frame_value))
}

#[test]
fn should_do_subtraction_with_longs() {
    let mut vm = VM::new("std");
    let last_frame_value = vm.run("samples.arithmetics.sub.longs.SubLongs").unwrap();
    assert_eq!(-1_000_000_000, get_long(last_frame_value))
}

#[test]
fn should_write_read_instance_fields() {
    let mut vm = VM::new("std");
    let last_frame_value = vm
        .run("samples.fields.instance.ints.InstanceFieldsUserInts")
        .unwrap();
    assert_eq!(110022, get_int(last_frame_value))
}

#[test]
fn should_write_read_instance_fields_with_longs() {
    let mut vm = VM::new("std");
    let last_frame_value = vm
        .run("samples.fields.instance.longs.InstanceFieldsUserLong")
        .unwrap();
    assert_eq!(4_380_866_642_760, get_long(last_frame_value))
}

#[test]
fn should_write_read_static_fields() {
    let mut vm = VM::new("std");
    let last_frame_value = vm
        .run("samples.fields.staticinitialization.ints.StaticFieldsUserInts")
        .unwrap();
    assert_eq!(110022, get_int(last_frame_value))
}

#[test]
fn should_do_extreme_stack_operations() {
    let mut vm = VM::new("std");
    let last_frame_value = vm
        .run("samples.arithmetics.extremestack.ints.ExtremeStackInt")
        .unwrap();
    assert_eq!(454, get_int(last_frame_value))
}

#[test]
fn should_do_extreme_stack_operations_with_longs() {
    let mut vm = VM::new("std");
    let last_frame_value = vm
        .run("samples.arithmetics.extremestack.longs.ExtremeStackLong")
        .unwrap();
    assert_eq!(454, get_long(last_frame_value))
}

#[test]
fn should_do_calculate_fibonacci_iteratively() {
    let mut vm = VM::new("std");
    let last_frame_value = vm
        .run("samples.arithmetics.fibonacci.iterative.FibonacciIterative")
        .unwrap();
    assert_eq!(55, get_int(last_frame_value))
}

#[test]
fn should_do_calculate_fibonacci_recursively() {
    let mut vm = VM::new("std");
    let last_frame_value = vm
        .run("samples.arithmetics.fibonacci.recursive.FibonacciRecursive")
        .unwrap();
    assert_eq!(55, get_int(last_frame_value))
}

#[test]
fn should_do_arrays() {
    let mut vm = VM::new("std");
    let last_frame_value = vm.run("samples.arrays.array.ints.ArrayInt").unwrap();
    assert_eq!(740, get_int(last_frame_value))
}

#[test]
fn should_do_arrays_with_longs() {
    let mut vm = VM::new("std");
    let last_frame_value = vm.run("samples.arrays.array.longs.ArrayLong").unwrap();
    assert_eq!(233646220932000, get_long(last_frame_value))
}

#[test]
fn should_do_3d_arrays() {
    let mut vm = VM::new("std");
    let last_frame_value = vm.run("samples.arrays.array3d.Array3D").unwrap();
    assert_eq!(780, get_int(last_frame_value))
}

#[test]
fn should_do_class_static_initialization() {
    let mut vm = VM::new("std");
    let last_frame_value = vm
        .run("samples.fields.staticinitialization.array.StaticInitializationArray")
        .unwrap();
    assert_eq!(257, get_int(last_frame_value))
}

#[test]
fn should_do_class_static_initialization_multiple_classes() {
    let mut vm = VM::new("std");
    let last_frame_value = vm
        .run("samples.fields.staticinitialization.chain.StaticInitializationChain")
        .unwrap();
    assert_eq!(350, get_int(last_frame_value))
}

#[test]
fn should_do_class_static_initialization_within_one_class() {
    let mut vm = VM::new("std");
    let last_frame_value = vm
        .run("samples.fields.staticinitialization.oneclass.StaticInitializationWithinOneClass")
        .unwrap();
    assert_eq!(100, get_int(last_frame_value))
}

#[test]
fn should_do_class_static_initialization_advanced() {
    let mut vm = VM::new("std");
    let last_frame_value = vm
        .run("samples.fields.staticinitialization.advanced.StaticInitializationAdvanced")
        .unwrap();
    assert_eq!(826, get_int(last_frame_value))
}

#[test]
fn should_do_class_static_initialization_circular() {
    let mut vm = VM::new("std");
    let last_frame_value = vm
        .run("samples.fields.staticinitialization.circular.StaticInitializationCircular")
        .unwrap();
    assert_eq!(700, get_int(last_frame_value))
}

#[test]
fn should_do_inherited_static_fields() {
    let mut vm = VM::new("std");
    let last_frame_value = vm
        .run("samples.inheritance.staticfield.InheritanceStaticField")
        .unwrap();
    assert_eq!(128, get_int(last_frame_value))
}

#[test]
fn should_do_trivial_interfaces() {
    let mut vm = VM::new("std");
    let last_frame_value = vm
        .run("samples.inheritance.interfaces.trivial.TrivialInterface")
        .unwrap();
    assert_eq!(-200, get_int(last_frame_value))
}

#[test]
fn should_do_inherited_implementations_interfaces() {
    let mut vm = VM::new("std");
    let last_frame_value = vm
        .run("samples.inheritance.interfaces.inheritedimplementation.InheritedImplementationInterface")
        .unwrap();
    assert_eq!(-200, get_int(last_frame_value))
}

#[test]
fn should_support_one_interface_extends_another() {
    let mut vm = VM::new("std");
    let last_frame_value = vm
        .run("samples.inheritance.interfaces.oneinterfaceextendsanother.OneInterfaceExtendsAnother")
        .unwrap();
    assert_eq!(-400, get_long(last_frame_value))
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
