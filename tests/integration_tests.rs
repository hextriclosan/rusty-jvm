mod utils;
use utils::assert_success;

#[test]
fn should_deal_with_abstract_class_without_interface_implementation() {
    assert_success(
        "samples.inheritance.abstractclasswithoutimpl.AbstractClassWithoutInterfaceImplementation",
        "2\n",
    );
}

#[test]
fn should_create_array_class_type() {
    assert_success("samples.reflection.trivial.ArrayClass", "16777215\n");
}

#[test]
fn empty_string_in_cpool() {
    assert_success(
        "samples.javacore.strings.trivial.EmptyStringInCPool",
        "Empty string has length 0\n",
    );
}

#[test]
fn should_calculate_hashcodes() {
    assert_success(
        "samples.javacore.hashcodes.trivial.HashCodeExample",
        "255\n",
    );
}

#[test]
fn should_call_interface_default_method() {
    assert_success(
        "samples.inheritance.interfacedefaultmethoddirectcall.InterfaceDefaultMethodDirectCall",
        "31\n",
    );
}

#[test]
fn should_call_object_methods() {
    assert_success("samples.javacore.object.trivial.ObjectMethods", "1\n");
}

#[test]
fn should_check_if_class_is_interface() {
    assert_success(
        "samples.reflection.trivial.classisinterface.ClassIsInterfaceExample",
        "1\n",
    );
}

#[test]
fn should_clone_cloneables() {
    assert_success("samples.javacore.cloneable.trivial.CloneExample", "511\n");
}

#[test]
fn should_convert_long_to_double_and_back() {
    assert_success(
        "samples.javacore.doubles.trivial.LongToDoubleAndBack",
        "2\n",
    );
}

#[test]
fn should_convert_to_string_and_back() {
    assert_success("samples.javacore.strings.trivial.ToStringAndBack", "511\n");
}

#[test]
fn should_do_3d_arrays() {
    assert_success("samples.arrays.array3d.Array3D", "780\n360\n");
}

#[test]
fn should_do_abstract_classes() {
    assert_success("samples.inheritance.abstractclass.AbstractClass", "145\n");
}

#[test]
fn should_do_adding() {
    assert_success("samples.arithmetics.adder.ints.AdderInt", "55\n");
}

#[test]
fn should_do_adding_with_longs() {
    assert_success(
        "samples.arithmetics.adder.longs.AdderLong",
        "171798691900\n",
    );
}

#[test]
fn should_do_adding_with_negative_longs() {
    assert_success(
        "samples.arithmetics.addernegative.AdderNegativeLong",
        "-1990000000000000\n",
    );
}

#[test]
fn should_do_arrays() {
    assert_success("samples.arrays.array.ints.ArrayInt", "740\n");
}

#[test]
fn should_do_arrays_with_longs() {
    assert_success("samples.arrays.array.longs.ArrayLong", "233646220932000\n");
}

#[test]
fn should_do_byte_operations() {
    assert_success("samples.javacore.bytes.trivial.ByteOperations", "2097151\n");
}

#[test]
fn should_do_calculate_fibonacci_iteratively() {
    assert_success(
        "samples.arithmetics.fibonacci.iterative.FibonacciIterative",
        "55\n",
    );
}

#[test]
fn should_do_calculate_fibonacci_recursively() {
    assert_success(
        "samples.arithmetics.fibonacci.recursive.FibonacciRecursive",
        "55\n",
    );
}

#[test]
fn should_do_class_static_initialization() {
    assert_success(
        "samples.fields.staticinitialization.array.StaticInitializationArray",
        "257\n",
    );
}

#[test]
fn should_do_class_static_initialization_advanced() {
    assert_success(
        "samples.fields.staticinitialization.advanced.StaticInitializationAdvanced",
        "826\n",
    );
}

#[test]
fn should_do_class_static_initialization_circular() {
    assert_success(
        "samples.fields.staticinitialization.circular.StaticInitializationCircular",
        "700\n",
    );
}

#[test]
fn should_do_class_static_initialization_multiple_classes() {
    assert_success(
        "samples.fields.staticinitialization.chain.StaticInitializationChain",
        "350\n",
    );
}

#[test]
fn should_do_class_static_initialization_within_one_class() {
    assert_success(
        "samples.fields.staticinitialization.oneclass.StaticInitializationWithinOneClass",
        "100\n",
    );
}

#[test]
fn should_do_composite_pattern() {
    assert_success(
        "samples.inheritance.interfaces.compositepattern.CompositePattern",
        "700\n",
    );
}

#[test]
fn should_do_extreme_stack_operations() {
    assert_success(
        "samples.arithmetics.extremestack.ints.ExtremeStackInt",
        "528\n",
    );
}

#[test]
fn should_do_extreme_stack_operations_with_longs() {
    assert_success(
        "samples.arithmetics.extremestack.longs.ExtremeStackLong",
        "454\n",
    );
}

#[test]
fn should_do_inherited_implementations_interfaces() {
    assert_success(
        "samples.inheritance.interfaces.inheritedimplementation.InheritedImplementationInterface",
        "-200\n",
    );
}

#[test]
fn should_do_inherited_instance_fields() {
    assert_success(
        "samples.inheritance.instancefield.InheritanceInstanceField",
        "128\n",
    );
}

#[test]
fn should_do_inherited_static_fields() {
    assert_success(
        "samples.inheritance.staticfield.InheritanceStaticField",
        "128\n",
    );
}

#[test]
fn should_do_interface_and_abstract_class() {
    assert_success(
        "samples.inheritance.interfaceandabstractclass.InterfaceAndAbstractClass",
        "36630\n",
    );
}

#[test]
fn should_do_native_call_on_system_array_copy() {
    assert_success(
        "samples.nativecall.system.NativeCallSystemArrayCopy",
        "15\n",
    );
}

use crate::utils::{assert_file, get_output};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn should_do_native_call_on_system_current_time() {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let expected_millis = since_the_epoch.as_millis() as i64;

    let output = get_output("samples.nativecall.system.NativeCallSystemCurrentTimeMillis");
    let actual_millis: i64 = output.trim().parse().expect("Not a number");

    assert!((expected_millis..expected_millis + 2000).contains(&actual_millis))
}

#[test]
fn should_do_operations_with_doubles() {
    assert_success(
        "samples.arithmetics.operations.doubles.DoubleOperations",
        "2.8547008547008547E278\n",
    );
}

#[test]
fn should_do_strings_concat_inline() {
    assert_success(
        "samples.javacore.strings.concat.trivial.StringConcatInline",
        "112788\n",
    );
}

#[test]
fn should_do_strings_cpool_advanced() {
    assert_success(
        "samples.javacore.strings.cpool.advanced.StringPoolAdvanced",
        "127\n",
    );
}

#[test]
fn should_do_subtraction() {
    assert_success("samples.arithmetics.sub.ints.SubInts", "-999\n");
}

#[test]
fn should_do_subtraction_with_doubles() {
    assert_success(
        "samples.arithmetics.sub.doubles.SubDoubles",
        "-8.76543211E200\n",
    );
}

#[test]
fn should_do_subtraction_with_floats() {
    assert_success("samples.arithmetics.sub.floats.SubFloats", "-998.9\n");
}

#[test]
fn should_do_subtraction_with_longs() {
    assert_success("samples.arithmetics.sub.longs.SubLongs", "-1000000000\n");
}

#[test]
fn should_do_trivial_cast() {
    assert_success("samples.javacore.cast.trivial.TrivialCast", "1337\n");
}

#[test]
fn should_do_trivial_concurrent_hash_maps() {
    assert_success(
        "samples.javabase.util.concurrent.hashmap.trivial.TrivialConcurrentHashMap",
        "97\n",
    );
}

#[test]
fn should_do_trivial_hashmaps() {
    assert_success(
        "samples.javabase.util.hashmap.trivial.TrivialHashMap",
        "1\n",
    );
}

#[test]
fn should_do_trivial_interfaces() {
    assert_success(
        "samples.inheritance.interfaces.trivial.TrivialInterface",
        "-200\n",
    );
}

#[test]
fn should_do_trivial_properties() {
    assert_success(
        "samples.javabase.util.properties.trivial.PropertiesTrivial",
        "60\n",
    );
}

#[test]
fn should_do_trivial_reflection() {
    assert_success("samples.reflection.trivial.TrivialReflection", "2578\n");
}

#[test]
fn should_do_trivial_reflection_with_primitives() {
    assert_success(
        "samples.reflection.trivial.synthetic.classes.SyntheticPrimitiveClasses",
        "9369\n",
    );
}

#[test]
fn should_do_trivial_strings() {
    assert_success("samples.javacore.strings.trivial.TrivialStrings", "8\n");
}

#[test]
fn should_do_trivial_strings_cpool() {
    assert_success(
        "samples.javacore.strings.cpool.trivial.TrivialStringsCPool",
        "8\n",
    );
}

#[test]
fn should_do_trivial_switch() {
    assert_success("samples.javacore.switches.trivial.SwitchExample", "1300\n");
}

#[test]
fn should_do_trivial_treemaps() {
    assert_success(
        "samples.javabase.util.treemap.trivial.TrivialTreeMap",
        "1\n",
    );
}

#[test]
fn should_do_trivial_unsafe_things() {
    assert_success(
        "samples.jdkinternal.unsafe.trivial.UnsafeUsage",
        "2097151\n",
    );
}

#[test]
fn should_do_trivial_util_arrays() {
    assert_success(
        "samples.javabase.util.arrays.trivial.TrivialUtilArrays",
        "9\n",
    );
}

#[test]
fn should_do_unsafe_object_field_offset() {
    assert_success(
        "samples.jdkinternal.unsafe.objectfieldoffset.UnsafeObjectFieldOffset",
        "127\n",
    );
}

#[test]
fn should_do_wide_instructions() {
    assert_success(
        "samples.javacore.wide.instructions.trivial.WideInstructionsExample",
        "31\n",
    );
}

#[test]
fn should_get_caller_class() {
    assert_success(
        "samples.jdkinternal.reflection.getcallerclass.ReflectionGetCallerClassExample",
        "1\n",
    );
}

#[test]
fn should_initialize_system_on_load() {
    assert_success("samples.system.load.SystemLoad", "4321\n");
}

#[test]
fn should_operate_with_map_interface() {
    assert_success(
        "samples.javabase.util.mapinterface.usage.AdvancedMapInterfaceUsage",
        "1\n",
    );
}

#[test]
fn should_perform_calculations_with_overflow() {
    assert_success("samples.arithmetics.overflow.ArithmeticOverflow", "1\n");
}

#[test]
fn should_print_to_stdout() {
    assert_success("samples.system.outexample.SystemOutExample", "42\n");
}

#[test]
fn should_return_class_name() {
    assert_success(
        "samples.reflection.trivial.classgetname.ClassGetNameExample",
        r#"java.lang.String
java.lang.Character$UnicodeBlock
byte
[Ljava.lang.Object;
[[[[[[[I
[[[[[[I
"#,
    );
}

#[test]
fn should_support_one_interface_extends_another() {
    assert_success(
        "samples.inheritance.interfaces.oneinterfaceextendsanother.OneInterfaceExtendsAnother",
        "-400\n",
    );
}

#[test]
fn should_use_grandparent_method_via_super() {
    assert_success(
        "samples.inheritance.usegrandparentmethodviasuper.UseGrandParentMethodViaSuperExample",
        "1337\n",
    );
}

#[test]
fn should_write_file_to_fs() {
    assert_file(
        "samples.io.fileoutputstreamexample.FileOutputStreamExample",
        "tests/tmp/test.txt",
        "CAFEBABE",
    )
}

#[test]
fn should_write_file_to_fs_with_buffered_stream() {
    assert_file(
        "samples.io.bufferedoutputstreamchunkingexample.BufferedOutputStreamChunkingExample",
        "tests/tmp/buffered_output.txt",
        "This is a test for BufferedOutputStream chunking.",
    )
}

#[test]
fn should_write_file_with_print_stream() {
    let expected_file_content = r#"Hello, PrintStream!
First Line
Second Line
Third Line
Hello as raw bytes
This is written immediately. This follows after flush.
This is an example of chaining PrintStreams.
"#;
    assert_file(
        "samples.io.printstreamexample.PrintStreamExample",
        "tests/tmp/print_stream_test.txt",
        expected_file_content,
    );
}

#[test]
fn should_write_read_instance_fields() {
    assert_success(
        "samples.fields.instance.ints.InstanceFieldsUserInts",
        "110022\n",
    );
}

#[test]
fn should_write_read_instance_fields_with_longs() {
    assert_success(
        "samples.fields.instance.longs.InstanceFieldsUserLong",
        "4380866642760\n",
    );
}

#[test]
fn should_write_read_static_fields() {
    assert_success(
        "samples.fields.staticinitialization.ints.StaticFieldsUserInts",
        "110022\n",
    );
}

#[test]
fn should_work_with_arrays_of_long_with_unsafe() {
    assert_success(
        "samples.jdkinternal.unsafe.getlongunaligned.UnsafeGetLongUnalignedExample",
        r#"2892188478761536005
3253889342951919370
3615590207142302735
3977291071332686100
4338991935523069465
4700692799713452830
5062393663903836195
5424094528094219560
5785795392284602925
"#,
    );
}

#[test]
fn should_initialize_arrays_with_default_values() {
    assert_success(
        "samples.arrays.defaultvalues.ArrayDefaultValues",
        r#"Default value in int array: 0
Default value in long array: 0
Default value in byte array: 0
Default value in short array: 0
Default value in float array: 0.0
Default value in double array: 0.0
Default value in char array: 0
Default value in boolean array: false
Default value in String array: null
Default value in 2D int array: 0
Default value in 2D float array: 0.0
"#,
    );
}

#[test]
fn should_support_returning_interface_from_static_method() {
    assert_success(
        "samples.inheritance.staticmethodreturnsinterface.StaticMethodReturnsInterface",
        "[some string]\n",
    );
}

#[test]
fn should_cast_arrays_when_possible() {
    assert_success(
        "samples.javacore.cast.arrays.ArrayCastExample",
        r#"[Ljava.lang.String;
[I
HelloWorld!
"#,
    );
}

#[test]
fn should_support_getting_supper_class() {
    assert_success(
        "samples.reflection.trivial.classgetsuperclassexample.GetSuperclassExample",
        r#"Superclass of java.lang.String: class java.lang.Object
Superclass of java.lang.Integer: class java.lang.Number
Superclass of java.lang.Object: null
Superclass of java.lang.Runnable: null
Superclass of samples.reflection.trivial.classgetsuperclassexample.GetSuperclassExample$ExtendedRunnable: null
Superclass of int: null
Superclass of void: null
Superclass of [Ljava.lang.String;: class java.lang.Object
Superclass of [I: class java.lang.Object
Superclass of [Ljava.lang.Runnable;: class java.lang.Object
Superclass of samples.reflection.trivial.classgetsuperclassexample.GetSuperclassExample$1: class java.lang.Object
Superclass of samples.reflection.trivial.classgetsuperclassexample.GetSuperclassExample$1LocalClass: class java.lang.Object
Superclass of samples.reflection.trivial.classgetsuperclassexample.GetSuperclassExample$InnerClass: class java.lang.Object
Superclass of java.util.concurrent.TimeUnit: class java.lang.Enum
Superclass of java.util.HashMap: class java.util.AbstractMap
"#,
    );
}
