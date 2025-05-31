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
    assert_success(
        "samples.reflection.trivial.ArrayClass",
        r#"Analyzing class: [[[I
----------------------------------------
Class: class [[[I
  isPrimitive: false
  isArray: true
Class: class [[I
  isPrimitive: false
  isArray: true
Class: class [I
  isPrimitive: false
  isArray: true
Class: int
  isPrimitive: true
  isArray: false
End of analysis.
========================================

Analyzing class: [[Ljava.lang.String;
----------------------------------------
Class: class [[Ljava.lang.String;
  isPrimitive: false
  isArray: true
Class: class [Ljava.lang.String;
  isPrimitive: false
  isArray: true
Class: class java.lang.String
  isPrimitive: false
  isArray: false
End of analysis.
========================================

Analyzing class: [[Ljava.lang.Runnable;
----------------------------------------
Class: class [[Ljava.lang.Runnable;
  isPrimitive: false
  isArray: true
Class: class [Ljava.lang.Runnable;
  isPrimitive: false
  isArray: true
Class: interface java.lang.Runnable
  isPrimitive: false
  isArray: false
End of analysis.
========================================

"#,
    );
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
        r#"customHashCode1: 310
customHashCode2: 620
customHashCode1 != identityHashCode1: true
customHashCode2 != identityHashCode2: true
objectHashCode1 != objectHashCode2: true
objectHashCode1 == objectIdentityHashCode1: true
objectHashCode2 == objectIdentityHashCode2: true
hashCodeOfNull: 0
"#,
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
    assert_success(
        "samples.javacore.cloneable.trivial.CloneExample",
        r#"cloneable and anotherCloneable have different references but the same content
cloneable is not affected by changes in anotherCloneable
intArray and anotherIntArray have different references but the same content
intArray is not affected by changes in anotherIntArray
objArray and anotherObjArray have different references but the same content
objArray is not affected by changes in anotherObjArray
intMatrix and anotherIntMatrix have different references but the same content
intMatrix is affected by changes in anotherIntMatrix
"#,
    );
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
    assert_success(
        "samples.javacore.strings.trivial.ToStringAndBack",
        r#"true
127
31999
ї
1999999999
999999999999
3.14
3.14
340282366920938463463374607431768211455
340282366920938463463374607431768211455.340282366920938463463374607431768211455
"#,
    );
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
    assert_success(
        "samples.javacore.bytes.trivial.ByteOperations",
        r#"b1: -10
sum: 10
b1 < b2: true
b1++: -9
b1--: -10
b2++: 21
b2--: 20
bMax: 127
bMin: -128
overflow: -128
underflow: 127
narrowedValue: -126
andResult: 1
orResult: 7
xorResult: 6
notResult: -6
shiftedLeft: 16
shiftedRight: 4
shiftedRightUnsigned: 4
shiftedRightOfNegative: -4
shiftedRightUnsignedOfNegative: 2147483644
"#,
    );
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
        r#"intArr(): [1, 2, 20, 30, 40, 6]
longArr(): [1, 2, 128849018920, 214748364860, 300647710800, 6]
intArrOverlapping(): [10, 10, 20, 30, 40, 50]
longArrOverlapping(): [42949672980, 42949672980, 128849018920, 214748364860, 300647710800, 386547056740]
upcasting(): [10, 20, 30]
"#,
    );
}

use crate::utils::{
    assert_failure, assert_failure_with_stderr, assert_file, assert_success_with_args,
    assert_success_with_stderr, get_file_separator, get_os_name, get_output,
    get_output_with_raw_args, get_path_separator, is_bigendian, line_ending, map_library_name,
};
use regex::Regex;
use serde_json::Value;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[test]
fn should_do_native_call_on_system_current_time() {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let expected_millis = since_the_epoch.as_millis() as i64;

    let output = get_output("samples.nativecall.system.NativeCallSystemCurrentTimeMillis");
    let actual_millis: i64 = output.trim().parse().expect("Not a number");

    let timeout_mins = 10;
    let timeout_millis = Duration::from_secs(timeout_mins * 60).as_millis() as i64;
    assert!((expected_millis..expected_millis + timeout_millis).contains(&actual_millis))
}

#[test]
fn should_do_operations_with_floats() {
    assert_success(
        "samples.arithmetics.operations.floats.FloatOperations",
        r#"3.4028235E38
3.4028235E38
0.0
1.7108213E8
1.5857277E30
NaN
-1.989E30
1.0
Infinity
0.0
Infinity
NaN
"#,
    );
}

#[test]
fn should_do_operations_with_doubles() {
    assert_success(
        "samples.arithmetics.operations.doubles.DoubleOperations",
        r#"1.7E308
1.7E308
0.0
8.547008547008547E277
1.338773289334918E30
NaN
-1.989E30
9.32579185520362E-74
Infinity
NaN
"#,
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
        r#"str1 and str2 have the same reference
str1 and str3 have different references
str1 and str3 have the same content
str1 and str5 have the same reference
str1 and str4 have the same reference
str1 and str6 have the same reference
str1 and str7 have different references but the same content
"#,
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
        &format!(
            r#"isBigEndian: {}
bytes: [0, 0, 0]
examinee.field3 value got by offset is: 30
examinee.field3 updated by offset: 40
examinee.field3 was not updated: 40
examinee.field4 value got by offset is: FIELD4
examinee.field4 updated by offset: FIELD4_UPDATED
examinee.field4 was not updated: FIELD4_UPDATED
examinee.field5 value got by offset is: 42949672980
examinee.field5 updated by offset: 128849018920
examinee.field5 was not updated: 128849018920
examinees[1] got by offset is `two`: true
examinees[1] updated by offset and set to `three`: true
examinees[1] was not updated and remains the same: true
one.field4 updated by offset to: FIELD4_PUT_REFERENCE_VOLATILE
"#,
            if is_bigendian() { 1 } else { 0 }
        ),
    );
}

#[test]
fn should_do_trivial_util_arrays() {
    assert_success(
        "samples.javabase.util.arrays.trivial.TrivialUtilArrays",
        r#"Binary search result: 9
Arrays are equal: false
Arrays compare: 1
Arrays compareUnsigned: -1
Mismatched index: 9
Hash code: 2117675315
List: [1, 2, 3]
String: [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
Copied: [10, 20, 30, 40, 50]
Copied of range: [60, 70, 80, 90, 100]
Filled: [10, 42, 42, 42, 42, 42, 42, 42, 42, 100]
Arrays are deep equals: true
Deep hash code: 32833
Deep string: [[1, 2], [3, 4]]
"#,
    );
}

#[test]
fn should_do_unsafe_object_field_offset() {
    assert_success(
        "samples.jdkinternal.unsafe.objectfieldoffset.UnsafeObjectFieldOffset",
        r#"int is compared and set successfully
Another int is compared and set successfully
Fields are initialized correctly
Fields are compared and set successfully
"#,
    );
}

#[test]
fn should_do_wide_instructions() {
    assert_success(
        "samples.javacore.wide.instructions.trivial.WideInstructionsExample",
        r#"20
16.0
31.0
10001000000
Updated wide test
Testing GOTO_W
"#,
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
        r#"java.util.HashMap
Map size is 3
Map contains key 'two'
Removed key 'three' with value '3'
Value for key 'four' is '4'
Sum of lengths of keys and values is 9
Map contains key 'nullKey' with null value
Merged key 'one' with value '10'
Computed key 'two' with value '2'
Replaced null value with 'DEFAULT'
One key starts with 'n'
Map is empty

java.util.TreeMap
Map size is 3
Map contains key 'two'
Removed key 'three' with value '3'
Value for key 'four' is '4'
Sum of lengths of keys and values is 9
Map contains key 'nullKey' with null value
Merged key 'one' with value '10'
Computed key 'two' with value '2'
Replaced null value with 'DEFAULT'
One key starts with 'n'
Map is empty

java.util.LinkedHashMap
Map size is 3
Map contains key 'two'
Removed key 'three' with value '3'
Value for key 'four' is '4'
Sum of lengths of keys and values is 9
Map contains key 'nullKey' with null value
Merged key 'one' with value '10'
Computed key 'two' with value '2'
Replaced null value with 'DEFAULT'
One key starts with 'n'
Map is empty

"#,
    );
}

#[test]
fn should_perform_integral_calculations_with_overflow() {
    assert_success(
        "samples.arithmetics.overflow.ArithmeticIntegralOverflow",
        r#"class java.lang.Byte:
add overflow: -128
mul overflow: -2
div overflow: -128
neg overflow: -128
rem overflow: 0
shl overflow: -2
shr overflow: -64
ushl overflow: -64
sub overflow: 127

class java.lang.Short:
add overflow: -32768
mul overflow: -2
div overflow: -32768
neg overflow: -32768
rem overflow: 0
shl overflow: -2
shr overflow: -16384
ushl overflow: -16384
sub overflow: 32767

class java.lang.Integer:
add overflow: -2147483648
mul overflow: -2
div overflow: -2147483648
neg overflow: -2147483648
rem overflow: 0
shl overflow: -2
shr overflow: -1073741824
ushl overflow: 1073741824
sub overflow: 2147483647

class java.lang.Long:
add overflow: -9223372036854775808
mul overflow: -2
div overflow: -9223372036854775808
neg overflow: -9223372036854775808
rem overflow: 0
shl overflow: -2
shr overflow: -4611686018427387904
ushl overflow: 4611686018427387904
sub overflow: 9223372036854775807

"#,
    );
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
        if is_bigendian() {
            // getting long unaligned is not endianness agnostic
            "2892188478761536005\n2530487614571152720\n2168786750380789835\n1807085886195649350\n1445385023347443265\n1083684502754443580\n722071599494282295\n382888733440751410\n5785795392284602925\n"
        } else {
            "2892188478761536005\n3253889342951919370\n3615590207142302735\n3977291071332686100\n4338991935523069465\n4700692799713452830\n5062393663903836195\n5424094528094219560\n5785795392284602925\n"
        },
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

#[test]
fn should_check_if_class_is_enum() {
    assert_success(
        "samples.reflection.trivial.isenumexample.IsEnumExample",
        r#"Is TimeUnit enum: true
Is TimeUnit enum: true
Is String enum: false
Is TimeUnit[] enum: false
Is Runnable enum: false
Is int enum: false
Is void enum: false
"#,
    );
}

#[test]
fn should_check_if_class_is_annotation() {
    assert_success(
        "samples.reflection.trivial.isannotationexample.IsAnnotationExample",
        r#"@MyAnnotation is annotation: true
String.class is annotation: false
@InheritedAnnotation is annotation: true
@MyAnnotation subclass is annotation: true
@SourceAnnotation is annotation: true
Annotation array is annotation: false
int.class is annotation: false
"#,
    );
}

#[test]
fn should_get_class_interfaces() {
    assert_success(
        "samples.reflection.trivial.getinterfacesexample.GetInterfacesExample",
        r#"Interfaces implemented by java.util.Map:
Interfaces implemented by samples.reflection.trivial.getinterfacesexample.GetInterfacesExample$ChildInterface:
	java.util.Map
	java.lang.Runnable
Interfaces implemented by java.util.HashMap:
	java.util.Map
	java.lang.Cloneable
	java.io.Serializable
Interfaces implemented by [Ljava.lang.String;:
	java.lang.Cloneable
	java.io.Serializable
Interfaces implemented by int:
"#,
    );
}

#[test]
fn should_return_declaring_class() {
    assert_success(
        "samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample",
        r#"double: null
[Ljava.lang.String;: null
samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample: null
samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample$TopLevel: class samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample
samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample$SimpleNested$Inner: class samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample$SimpleNested
samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample$SimpleNested$StaticNested: class samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample$SimpleNested
samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample$1: null
samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample$1LocalClass: null
samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample$1StaticMethodInner: null
samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample$DeepNesting$Inner: class samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample$DeepNesting
samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample$DeepNesting$Inner$DeepInner: class samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample$DeepNesting$Inner
samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample$DeepNesting$Inner$DeepInner: class samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample$DeepNesting$Inner
"#,
    );
}

#[test]
fn should_operate_with_var_args() {
    assert_success(
        "samples.javacore.varargs.trivial.VarArgsExample",
        r#"4
1000000000000
3.14
[1337]
{42=1}
"#,
    );
}

#[test]
fn should_support_getting_declared_methods() {
    assert_success(
        "samples.reflection.trivial.declaredmethods.DeclaredMethodsExample",
        r#"Information about method:sampleMethod
------------------------------------------------
Class:class samples.reflection.trivial.declaredmethods.DeclaredMethodsExample
Return Type:class [Ljava.lang.String;
Modifiers:private transient native
Parameter Count:3
Parameter Types:
	java.lang.String
	int
	[D
Generic Parameter Types:
	java.lang.String
	int
	double[]
Is Synthetic:false
Is Default:false
Is Bridge:false
Exception Types:
	java.io.IOError
	java.lang.NullPointerException
Generic Exception Types:
	java.io.IOError
	java.lang.NullPointerException
Is VarArgs:true
Generic Return Type:class [Ljava.lang.String;

Information about method:main
------------------------------------------------
Class:class samples.reflection.trivial.declaredmethods.DeclaredMethodsExample
Return Type:void
Modifiers:public static
Parameter Count:1
Parameter Types:
	[Ljava.lang.String;
Generic Parameter Types:
	java.lang.String[]
Is Synthetic:false
Is Default:false
Is Bridge:false
Exception Types:
Generic Exception Types:
Is VarArgs:false
Generic Return Type:void

"#,
    );
}

#[test]
fn should_support_getting_declared_constructors() {
    assert_success(
        "samples.reflection.trivial.declaredconstructors.DeclaredConstructorsExample",
        r#"Basic Example Constructors:
	----------------------------
	Parameter types: []
	Modifier: public
	Throws: []
	Declaring class: class samples.reflection.trivial.declaredconstructors.DeclaredConstructorsExample$BasicExample
	Name: samples.reflection.trivial.declaredconstructors.DeclaredConstructorsExample$BasicExample
	Parameter count: 0
	----------------------------
	Parameter types: [int]
	Modifier: protected
	Throws: []
	Declaring class: class samples.reflection.trivial.declaredconstructors.DeclaredConstructorsExample$BasicExample
	Name: samples.reflection.trivial.declaredconstructors.DeclaredConstructorsExample$BasicExample
	Parameter count: 1
	----------------------------
	Parameter types: [class java.lang.String]
	Modifier: private
	Throws: []
	Declaring class: class samples.reflection.trivial.declaredconstructors.DeclaredConstructorsExample$BasicExample
	Name: samples.reflection.trivial.declaredconstructors.DeclaredConstructorsExample$BasicExample
	Parameter count: 1
Edge Case Example Constructors:
	----------------------------
	Parameter types: []
	Modifier: 
	Throws: []
	Declaring class: class samples.reflection.trivial.declaredconstructors.DeclaredConstructorsExample$EdgeCaseExample
	Name: samples.reflection.trivial.declaredconstructors.DeclaredConstructorsExample$EdgeCaseExample
	Parameter count: 0
	----------------------------
	Parameter types: [class java.lang.String]
	Modifier: public
	Throws: [class java.lang.IllegalArgumentException]
	Declaring class: class samples.reflection.trivial.declaredconstructors.DeclaredConstructorsExample$EdgeCaseExample
	Name: samples.reflection.trivial.declaredconstructors.DeclaredConstructorsExample$EdgeCaseExample
	Parameter count: 1
Anonymous Class Constructors:
	----------------------------
	Parameter types: []
	Modifier: 
	Throws: []
	Declaring class: class samples.reflection.trivial.declaredconstructors.DeclaredConstructorsExample$1
	Name: samples.reflection.trivial.declaredconstructors.DeclaredConstructorsExample$1
	Parameter count: 0
"#,
    );
}

#[test]
fn should_support_class_get_enclosing_method() {
    assert_success(
        "samples.reflection.trivial.enclosingmethod.EnclosingMethodExample",
        r#"Top-level class enclosing method: null
StaticNestedClass enclosing method: null
NonStaticInnerClass enclosing method: null
LocalClass enclosing method: testEnclosingMethods
AnonymousClass enclosing method: testEnclosingMethods
Inside anonymous constructor initializer.
AnonymousClass in constructor enclosing method: testEnclosingMethods
LocalClass in constructor enclosing method: testEnclosingMethods
"#,
    );
}

#[test]
fn should_return_system_properties() {
    let output = get_output("samples.system.getpropertyexample.SystemGetPropertyExample");
    let json: Value = serde_json::from_str(&output).expect("Output is not valid JSON");

    // Check exact values
    assert_eq!(json["line.separator"], line_ending());
    assert_eq!(
        json["sun.cpu.endian"],
        if is_bigendian() { "big" } else { "little" }
    );

    // Check os.version format
    let os_version = json["os.version"]
        .as_str()
        .expect("os.version is not a string");
    let re = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
    assert!(re.is_match(os_version), "os.version format is incorrect");

    let user_dir = json["user.dir"].as_str().expect("user.dir is not a string");
    assert!(user_dir.ends_with("test_data"));

    let os_name = json["os.name"].as_str().expect("os.name is not a string");
    let os_name = os_name.to_lowercase();
    assert!(
        os_name.contains(get_os_name()),
        "os.name format is incorrect"
    );

    let file_separator = json["file.separator"]
        .as_str()
        .expect("file.separator is not a string");
    assert_eq!(file_separator, get_file_separator());

    let path_separator = json["path.separator"]
        .as_str()
        .expect("path.separator is not a string");
    assert_eq!(path_separator, get_path_separator());

    let path_separator = json["other.property"]
        .as_str()
        .expect("other.property is not a string");
    assert_eq!(path_separator, "null");

    let java_home = json["java.home"]
        .as_str()
        .expect("java.home is not a string");
    assert_eq!(java_home, "java.home_DEFAULT");
}

#[test]
fn should_return_overridden_system_properties() {
    let args = [
        "-Dother.property=other_value",
        "-Dos.version=42.42.42",
        "-Djava.home=some_dir",
        "samples.system.getpropertyexample.SystemGetPropertyExample",
    ];
    let output = get_output_with_raw_args(&args);
    let json: Value = serde_json::from_str(&output).expect("Output is not valid JSON");

    assert_eq!(json["other.property"], "other_value");
    assert_eq!(json["os.version"], "42.42.42");
    assert_eq!(json["java.home"], "some_dir");
}

#[test]
fn should_check_instanceof() {
    assert_success(
        "samples.reflection.trivial.instanceofexample.InstanceOfExample",
        r#"null is instanceof Integer: false
42 is instanceof Integer: true
42 is instanceof Number: true
3.14 is instanceof Double: true
3.14 is instanceof Number: true
3.14 is instanceof Float: false
Hello is instanceof String: true
Hello is instanceof Object: true
[1] is instanceof Object: true
[1, 2] is instanceof Object[]: false
[1, 2, 3] is instanceof int[]: true
[10, 20, 30] is instanceof Object: true
[A, B] is instanceof Object[]: true
[Meow!, Meow!] is instanceof Cat[]: true
[Meow!, Meow!] is instanceof Animal[]: true
[Meow!, Meow!] is instanceof Mammal[]: true
[Meow!, Meow!] is instanceof AbstractMammal[]: true
[Meow!, Woof!] is instanceof Cat[]: false
[Meow!, Woof!] is instanceof Animal[]: true
[Meow!, Woof!] is instanceof Mammal[]: false
[Meow!, Woof!] is instanceof AbstractMammal[]: false
[null] is instanceof List[]: true
[] is instanceof List: true
{} is instanceof Map: true
{} is instanceof AbstractMap: true
[] is instanceof Collection: true
Meow! is instanceof Animal: true
Chirp! is instanceof Animal: true
Chirp! is instanceof Mammal: false
Anonymous mammal says hi! is instanceof Animal: true
"#,
    );
}

#[test]
fn should_support_class_is_assignable_from_method() {
    assert_success(
        "samples.reflection.trivial.isassignablefromexample.IsAssignableFromExample",
        r#"int is assignable from int: true
Integer is assignable from int: false
Number is assignable from Integer: true
double is assignable from int: false
double is assignable from float: false
float is assignable from double: false
double is assignable from Double: false
Double is assignable from double: false
Cat is assignable from Animal: false
Bird is assignable from Animal: false
Bird is assignable from Mammal: false
Animal is assignable from Cat: true
Animal is assignable from Bird: true
Mammal is assignable from Bird: false
Object is assignable from int[]: true
Object is assignable from Integer[]: true
Object[] is assignable from String[]: true
Animal[] is assignable from Cat[]: true
Cat[] is assignable from Animal[]: false
Object[] is assignable from int[]: false
List[] is assignable from ArrayList[]: true
Object is assignable from Object: true
Object is assignable from String: true
String is assignable from Object: false
String is assignable from String: true
Integer is assignable from Integer: true
Integer is assignable from Number: false
Number is assignable from Integer: true
Number is assignable from Number: true
List is assignable from ArrayList: true
ArrayList is assignable from List: false
Map is assignable from HashMap: true
AbstractMap is assignable from HashMap: true
HashMap is assignable from AbstractMap: false
Collection is assignable from HashSet: true
HashSet is assignable from Collection: false
Map is assignable from HashMap: true
Animal is assignable from Dog: true
Animal is assignable from : true
"#,
    );
}

#[test]
fn should_support_dup_x2_opcode() {
    assert_success("samples.opcodes.dup_x2.DupX2GeneratedExample", "1260.0\n");
}

#[test]
fn should_support_pop2_opcode() {
    assert_success("samples.opcodes.pop2.Pop2GeneratedExample", "1000\n");
}

#[test]
fn should_support_nop_opcode() {
    assert_success("samples.opcodes.nop.NopGeneratedExample", "12\n");
}

#[test]
fn should_support_dup2_x1_opcode() {
    assert_success("samples.opcodes.dup2_x1.Dup2_X1GeneratedExample", "21321\n");
}

#[test]
fn should_support_dup2_x2_opcode() {
    assert_success(
        "samples.opcodes.dup2_x2.Dup2_X2GeneratedExample",
        "214321\n",
    );
}

#[test]
fn should_support_swap_opcode() {
    assert_success("samples.opcodes.swap.SwapGeneratedExample", "2\n");
}

#[test]
fn should_work_with_method_handle() {
    assert_success(
        "samples.reflection.methodhandleexample.MethodHandleExample",
        r#"MethodHandles Lookup: samples.reflection.methodhandleexample.MethodHandleExample
------- findStatic (Arrays.toString) -------
(int[])String - MethodHandle(int[])String: [1, 2, 3]
------- findVirtual (String.regionMatches) -------
(boolean,int,String,int,int)boolean - MethodHandle(String,boolean,int,String,int,int)boolean: true
------- findSpecial (Parent.testMethod) -------
()String - MethodHandle(Child)String: Parent method invoked
------- findConstructor (ArrayList) -------
()void - MethodHandle()ArrayList: [1337]
------- findConstructor (StringBuilder(String)) -------
(String)void - MethodHandle(String)StringBuilder: 1 + 1 = 2
------- findGetter / findSetter (SampleClass.value) -------
(SampleClass)int - MethodHandle(SampleClass)int: 42
------- findStaticGetter / findStaticSetter (staticValue) -------
()int - MethodHandle()int: 500
"#,
    )
}

#[test]
fn should_support_class_is_instance_method() {
    assert_success(
        "samples.reflection.trivial.classisinstanceexample.ClassIsInstanceExample",
        r#"null is instance of Integer.class: false
42 is instance of Integer.class: true
42 is instance of Number.class: true
3.14 is instance of Double.class: true
3.14 is instance of Number.class: true
3.14 is instance of Float.class: false
Hello is instance of String.class: true
Hello is instance of Object.class: true
[1] is instance of Object.class: true
[1, 2] is instance of Object[].class: false
[1, 2, 3] is instance of int[].class: true
[10, 20, 30] is instance of Object.class: true
[A, B] is instance of Object[].class: true
[Meow!, Meow!] is instance of Cat[].class: true
[Meow!, Meow!] is instance of Animal[].class: true
[Meow!, Meow!] is instance of Mammal[].class: true
[Meow!, Meow!] is instance of AbstractMammal[].class: true
[Meow!, Woof!] is instance of Cat[].class: false
[Meow!, Woof!] is instance of Animal[].class: true
[Meow!, Woof!] is instance of Mammal[].class: false
[Meow!, Woof!] is instance of AbstractMammal[].class: false
[null] is instance of List[].class: true
[] is instance of List.class: true
{} is instance of Map.class: true
{} is instance of AbstractMap.class: true
[] is instance of Collection.class: true
Meow! is instance of Animal.class: true
Chirp! is instance of Animal.class: true
Chirp! is instance of Mammal.class: false
Anonymous mammal says hi! is instance of Animal.class: true
"#,
    )
}

#[test]
fn should_support_unsafe_put_reference_volatile() {
    assert_success(
        "samples.jdkinternal.unsafe.putreferencevolatile.UnsafePutReferenceVolatileExample",
        "[5000000000, 5000000001, 5000000002]\n",
    );
}

// This test class is put to java.lang package for calling package-private method
// It's not so nice but bad test is better than no test
#[test]
fn should_support_system_get_constant_pool() {
    assert_success(
        "java.lang.ConstantPoolExample",
        r#"Constant pool size: 15
1: CLASS ()
2: UTF8 (java/lang/SuppressWarnings)
3: CLASS ()
4: UTF8 (java/lang/Object)
5: CLASS ()
6: UTF8 (java/lang/annotation/Annotation)
7: UTF8 (value)
8: UTF8 (()[Ljava/lang/String;)
9: UTF8 (SourceFile)
10: UTF8 (SuppressWarnings.java)
11: UTF8 (RuntimeVisibleAnnotations)
12: UTF8 (Ljava/lang/annotation/Retention;)
13: UTF8 (Ljava/lang/annotation/RetentionPolicy;)
14: UTF8 (SOURCE)
"#,
    );
}

#[test]
fn should_support_assertions() {
    assert_success(
        "samples.javacore.assertions.trivial.AssertionExample",
        "Assertions: enabled\n",
    );
}

#[test]
fn should_support_lazy_static_initialization() {
    assert_success(
        "samples.fields.staticinitialization.lazy.LazyInitializationExample",
        r#"LazyInitializationExample class loaded.
Before accessing LazyHolder.VALUE
Triggering another method, but NOT accessing LazyHolder.
Now accessing LazyHolder.VALUE: LazyHolder class loaded.
Initializing VALUE...
Lazy Loaded Value
Accessing LazyHolder.VALUE again: Lazy Loaded Value
"#,
    );
}

#[test]
fn should_support_child_via_parent_static_init() {
    assert_success(
        "samples.javacore.abstractclasses.abstractclassstaticinit.AbstractClassStaticInitExample",
        r#"Abstract static init
Data constructor
Data getValue
42
"#,
    );
}

#[test]
fn should_do_trivial_operations_on_enums() {
    assert_success(
        "samples.javacore.enums.trivial.EnumsExample",
        r#"FormatStyle values: [FULL, LONG, MEDIUM, SHORT]
FormatStyle.MEDIUM name: MEDIUM
FormatStyle.MEDIUM ordinal: 2
medium == anotherMedium: true
"#,
    );
}

#[test]
fn should_support_class_get_nest_host() {
    assert_success(
        "samples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample",
        r#"ClassGetNestHostExample (class samples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample): class samples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample
NestedClass (class samples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample$NestedClass): class samples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample
InnerClass (class samples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample$InnerClass): class samples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample
LocalClass (class samples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample$1LocalClass): class samples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample
Runnable (class samples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample$1): class samples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample
InnerInterface (class samples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample$2): class samples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample
String (class java.lang.String): class java.lang.String
int (int): int
ClassGetNestHostExample[] (class [Lsamples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample;): class [Lsamples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample;
"#,
    );
}

#[test]
fn should_support_reflection_are_nest_mates() {
    assert_success(
        "samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample",
        r#"class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample and class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample are nest mates: true
class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample and class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample$NestedClass are nest mates: true
class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample and class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample$InnerClass are nest mates: true
class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample$NestedClass and class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample$InnerClass are nest mates: true
class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample and class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample$1LocalClass are nest mates: true
class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample and class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample$1 are nest mates: true
class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample and class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample$2 are nest mates: true
class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample$1 and class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample$2 are nest mates: true
class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample and class java.lang.String are nest mates: false
class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample and class [Lsamples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample; are nest mates: false
"#,
    );
}

#[test]
fn should_support_class_for_name_method() {
    assert_success(
        "samples.reflection.trivial.forname.ClassForNameExample",
        r#"First static block
class samples.reflection.trivial.forname.First
class samples.reflection.trivial.forname.Second
"#,
    );
}

#[test]
fn should_calling_parent_static_method_via_child_class() {
    assert_success(
        "samples.inheritance.staticmethod.ParentStaticMethodCalledViaChild",
        "Parent method called\n",
    );
}

#[test]
fn should_calling_parent_static_block_before_child() {
    assert_success(
        "samples.inheritance.parentstaticinit.InitParentStaticBlockBeforeChild",
        r#"Parent static block
Child static block
Child method called
"#,
    );
}

#[test]
fn should_call_static_init_before_constructor_invocation() {
    assert_success(
        "samples.staticinit.byconstructor.StaticInitializationByConstructorExample",
        r#"main starts
SomeClass static block
SomeClass constructor
main ends
"#,
    );
}

#[test]
fn should_init_only_parent_class_when_its_field_is_accessed() {
    assert_success(
        "samples.staticinit.parentstaticfield.OnlyTheClassThatDeclaresStaticFieldIsInitialized",
        r#"main starts
1729
main ends
"#,
    );
}

#[test]
fn should_call_static_init_by_method_handle() {
    assert_success(
        "samples.staticinit.bymethodhandle.MethodHandleStaticInitCases",
        r#"Main method started
=== Case 1: Invoking a static method (triggers initialization) ===
MethodHandle obtained, but class not initialized yet
Static block executed: Class1 initialized
Class1: Static method invoked

=== Case 2: Invoking a constructor (triggers initialization) ===
Constructor MethodHandle obtained, but class not initialized yet
Static block executed: Class2 initialized
Class2: Constructor executed

=== Case 3: Accessing a static field getter (triggers initialization) ===
Static field MethodHandle obtained, but class not initialized yet
Static block executed: Class3 initialized
Static field value: 42

=== Case 4: Accessing a static field setter (triggers initialization) ===
Static field setter MethodHandle obtained, but class not initialized yet
Static block executed: Class4 initialized
Initial STATIC_FIELD value: 10
Updated STATIC_FIELD: 100

Main method complete
"#,
    );
}

#[test]
fn should_not_initialize_super_interfaces() {
    assert_success(
        "samples.staticinit.interfaceinitializationdoesnotinitializesuperinterfaces.InterfaceInitializationDoesNotInitializeSuperinterfaces",
        r#"main starts
1
j=3
jj=4
3
main ends
"#,
    );
}

#[test]
fn should_use_static_fields_from_parents() {
    assert_success(
        "samples.staticinit.parentstaticfieldaccess.StaticFieldAccessFromParentExample",
        r#"PARENT_FIELD: 1
CHILD_FIELD: 2
PARENT_INTERFACE_FIELD: 3
CHILD_INTERFACE_FIELD: 4
"#,
    );
}

#[test]
fn should_not_call_static_block_for_const_fields_should_call_for_non_constant() {
    assert_success(
        "samples.staticinit.const_vs_nonconst_fields.ConstVsNonConstFieldsExample",
        r#"CONST_INT_FIELD: 1337
CONST_STRING_FIELD: Hello, Constant!
COMPILE_TIME_CONST_FILED: 12.56
CONST_FILED_FROM_ANOTHER_CLASS: 25.12
NonConst1 static block
NON_CONST_INT_FIELD: 1337
NonConst2 static block
NON_CONST_STRING_FIELD: Hello, Non-Constant!
NonConst3 static block
RUNTIME_INITIALIZED_NON_CONST_FILED: 42
NonConst4 static block
NON_CONST_FILED_DEPENDS_ON_ANOTHER: 12.56
NonConst5 static block
NON_CONST_FILED_DEPENDS_ON_ANOTHER_CLASS: 25.12
NonConst6 static block
NON_CONST_FILED_CONDITIONAL: 10
NonConst7 static block
NON_CONST_FILED_ASSIGNED_IN_STATIC_BLOCK: 1337
"#,
    );
}

#[test]
fn should_skip_initialization_of_child_static_field_from_parent() {
    assert_success(
        "samples.staticinit.child_field_in_parent_block_accessed_by_child.ChildFieldInParentBlockAccessedByChild",
        r#"entering main
Parent static block
Child static block
null-parent-child
exiting main
"#,
    );
}

#[test]
fn should_perform_initialization_of_child_static_field_when_called_from_parent() {
    assert_success(
        "samples.staticinit.child_field_in_parent_block_accessed_by_parent.ChildFieldInParentBlockAccessedByParent",
        r#"entering main
Parent static block
Child static block
null-child-parent
exiting main
"#,
    );
}

#[test]
fn should_use_value_from_child_static_block() {
    assert_success(
        "samples.staticinit.initialized_child_field_in_parent_block_accessed_by_child.InitializedChildFieldInParentBlockAccessedByChild",
        r#"entering main
Parent static block
Child static block
initial-child
exiting main
"#,
    );
}

#[test]
fn should_perform_initialization_of_child_static_field_and_parent_when_called_from_parent() {
    assert_success(
        "samples.staticinit.initialized_child_field_in_parent_block_accessed_by_parent.InitializedChildFieldInParentBlockAccessedByParent",
        r#"entering main
Parent static block
Child static block
initial-child-parent
exiting main
"#,
    );
}

#[test]
#[ignore = "implement Class.getDeclaredFields0(...)"]
fn should_return_random_number() {
    assert_success("samples.javautil.random.RandomExample", "666\n");
}

#[test]
#[ignore = "implement invokedynamic opcode"]
fn should_work_with_records() {
    assert_success(
        "samples.javacore.recordexample.RecordExample",
        r#"Rcd[first=10, second=20]
200
true"#,
    );
}

#[test]
fn should_support_constant_call_site() {
    assert_success(
        "samples.reflection.constantcallsiteexample.ConstantCallSiteExample",
        "Hello from CallSite!\n",
    );
}

#[test]
#[ignore = "BoundMethodHandle:53 - assert(speciesData() == speciesDataFor(form))"]
fn should_support_mutable_call_site() {
    assert_success(
        "samples.reflection.mutablecallsiteexample.MutableCallSiteExample",
        "Hello from targetMethod1!\nHello from targetMethod2!\n",
    );
}

#[test]
fn should_map_library_name() {
    assert_success(
        "samples.system.maplibraryname.MapLibraryNameExample",
        &format!("{}\n", map_library_name("name")),
    );
}

#[test]
fn should_return_default_filesystem() {
    let expected = if cfg!(windows) {
        r#"Default FileSystem class: WindowsFileSystem
FileSystem provider: sun.nio.fs.WindowsFileSystemProvider
Separator: \
Supported file attribute views:[acl, basic, dos, owner, user]
Is open: true
Is read-only: false
"#
    } else if cfg!(target_os = "macos") {
        r#"Default FileSystem class: MacOSXFileSystem
FileSystem provider: sun.nio.fs.MacOSXFileSystemProvider
Separator: /
Supported file attribute views:[basic, owner, posix, unix, user]
Is open: true
Is read-only: false
"#
    } else if cfg!(target_os = "linux") {
        r#"Default FileSystem class: LinuxFileSystem
FileSystem provider: sun.nio.fs.LinuxFileSystemProvider
Separator: /
Supported file attribute views:[basic, dos, owner, posix, unix, user]
Is open: true
Is read-only: false
"#
    } else {
        unreachable!("Unsupported OS")
    };

    assert_success(
        "samples.filesystem.getdefaultfilesystem.GetDefaultFileSystem",
        &format!("{}", expected),
    );
}

#[test]
fn should_print_out_program_args() {
    assert_success_with_args(
        "samples.system.programargs.ProgramArgsExample",
        &["Hello", "from", "Java", "!"],
        "[Hello, from, Java, !]\n",
    );
}

#[test]
fn should_handle_operand_stack_overflow() {
    assert_failure(
        "samples.invalidprograms.operandstackoverflow.OperandStackOverflowExample",
        r#"VM execution failed: Execution Error: Reason: Execution Error: Exceeded max stack size; Current Frame: StackFrame { index: 11435, method_name: "returnInt:()I", pc: 5, ex_pc: None, locals: [], operand_stack: Stack { max_size: 2, data: [10, 20] }, bytecode_ref: [18, 7, 18, 8, 18, 9, 172], current_class_name: "samples/invalidprograms/operandstackoverflow/OperandStackOverflowExample", line_numbers: {}, exception_table: ExceptionTable { table: [] } }"#,
    );
}

#[test]
fn should_support_file_dispatcher_for_various_os() {
    let expected_file_content = "Hello from FileChannel!";
    assert_file(
        "samples.nio.filedispatcherexample.FileDispatcherExample",
        "tests/tmp/file_dispatcher_example.txt",
        expected_file_content,
    );
}

#[test]
// todo: https://github.com/hextriclosan/rusty-jvm/issues/317
fn should_support_exceptions() {
    let expected_error_text = if cfg!(windows) {
        "non_existing\\non_existing.txt (The system cannot find the path specified. (os error 3))"
    } else {
        "non_existing/non_existing.txt (No such file or directory (os error 2))"
    };
    assert_success(
        "samples.javacore.exceptionexample.ExceptionExample",
        r#"Running case: FewTriesInOneMethod
Inside try block
Caught as Throwable: java.lang.Error: This is an error. cause=null stackTrace=[samples.javacore.exceptionexample.FewTriesInOneMethod.thrower(ExceptionExample.java:73), samples.javacore.exceptionexample.FewTriesInOneMethod.runImpl(ExceptionExample.java:55), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:33), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:22)] suppressed=[]
Inside another try block
Caught as Throwable second time: java.lang.IndexOutOfBoundsException: This is an index out of bounds exception. cause=null stackTrace=[samples.javacore.exceptionexample.FewTriesInOneMethod.runImpl(ExceptionExample.java:66), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:33), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:22)] suppressed=[]

Running case: TryWithResources
Inside try-with-resources block
Doing something with the resource
Custom resource is about to be closed
Custom resource is closed
Caught try-with-resources exception: java.lang.RuntimeException: An error occurred while using the resource. cause=null stackTrace=[samples.javacore.exceptionexample.CustomResource.doSomethingAndThrow(ExceptionExample.java:248), samples.javacore.exceptionexample.TryWithResources.tryWithResourcesWithNoCatch(ExceptionExample.java:96), samples.javacore.exceptionexample.TryWithResources.runImpl(ExceptionExample.java:88), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:33), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:22)] suppressed=[]

Running case: TryWithResources
Inside try-with-resources block
Doing something with the resource
Custom resource is about to be closed
Caught try-with-resources exception: java.lang.RuntimeException: An error occurred while using the resource. cause=null stackTrace=[samples.javacore.exceptionexample.CustomResource.doSomethingAndThrow(ExceptionExample.java:248), samples.javacore.exceptionexample.TryWithResources.tryWithResourcesWithNoCatch(ExceptionExample.java:96), samples.javacore.exceptionexample.TryWithResources.runImpl(ExceptionExample.java:88), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:33), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:22)] suppressed=[java.lang.IllegalStateException: An error occurred while closing the resource]

Running case: TryWithResourcesMimic
Inside try-with-resources block
Doing something with the resource
Custom resource is about to be closed
Custom resource is closed
Caught try-with-resources exception: java.lang.RuntimeException: An error occurred while using the resource. cause=null stackTrace=[samples.javacore.exceptionexample.CustomResource.doSomethingAndThrow(ExceptionExample.java:248), samples.javacore.exceptionexample.TryWithResourcesMimic.tryWithResourcesMimic(ExceptionExample.java:122), samples.javacore.exceptionexample.TryWithResourcesMimic.runImpl(ExceptionExample.java:112), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:33), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:22)] suppressed=[]

Running case: TryWithResourcesMimic
Inside try-with-resources block
Doing something with the resource
Custom resource is about to be closed
Caught try-with-resources exception: java.lang.RuntimeException: An error occurred while using the resource. cause=null stackTrace=[samples.javacore.exceptionexample.CustomResource.doSomethingAndThrow(ExceptionExample.java:248), samples.javacore.exceptionexample.TryWithResourcesMimic.tryWithResourcesMimic(ExceptionExample.java:122), samples.javacore.exceptionexample.TryWithResourcesMimic.runImpl(ExceptionExample.java:112), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:33), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:22)] suppressed=[java.lang.IllegalStateException: An error occurred while closing the resource]

Running case: ReThrowWithCause
Inside try block
Caught as Throwable: java.lang.RuntimeException: This is a runtime exception. cause=null stackTrace=[samples.javacore.exceptionexample.ReThrowWithCause.runImpl(ExceptionExample.java:143), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:33), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:22)] suppressed=[]
Caught as IllegalStateException: java.lang.IllegalStateException: java.lang.RuntimeException: This is a runtime exception. cause=java.lang.RuntimeException: This is a runtime exception stackTrace=[samples.javacore.exceptionexample.ReThrowWithCause.runImpl(ExceptionExample.java:146), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:33), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:22)] suppressed=[]

Running case: FinallyIllustration
Executing try block
Executing finally block
No exception in try, finally still executes: try-finally
============================
Executing try block
Executing catch blockjava.lang.RuntimeException: Exception in try
Executing finally block
Caught exception in try, finally still executes: try-catch-finally
============================
Executing try block
Executing finally block
Caught as Throwable: java.lang.RuntimeException: Exception in try. cause=null stackTrace=[samples.javacore.exceptionexample.FinallyIllustration.withUncaughtException(ExceptionExample.java:209), samples.javacore.exceptionexample.FinallyIllustration.runImpl(ExceptionExample.java:168), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:33), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:22)] suppressed=[]
Uncaught exception in try, finally still executes: try-finally

Running case: ExceptionFromNativeMethod
Caught as IOException: java.io.FileNotFoundException: {ERROR_TEXT}. cause=null stackTrace=[] suppressed=[]

"#.replace("{ERROR_TEXT}", expected_error_text).as_str(),
    );
}

#[test]
fn should_support_both_stdout_and_stderr() {
    assert_success_with_stderr(
        "samples.system.normaloutputwitherroutput.NormalOutputWithErrorOutput",
        r#"This is normal output.
This is another normal output.
"#,
        r#"This is error output.
This is another error output.
"#,
    );
}

#[cfg(not(windows))] // todo: fix this test on Windows
#[test]
fn should_print_info_about_unhandled_exception() {
    assert_failure_with_stderr(
        "samples.javacore.unhandledexception.UnhandledExceptionExample",
        r#""#,
        r#"Exception in thread "system" java.lang.StringIndexOutOfBoundsException: Range [2, 1) out of bounds for length 5
	at jdk.internal.util.Preconditions$1.apply(Preconditions.java:55)
	at jdk.internal.util.Preconditions$1.apply(Preconditions.java:52)
	at jdk.internal.util.Preconditions$4.apply(Preconditions.java:213)
	at jdk.internal.util.Preconditions$4.apply(Preconditions.java:210)
	at jdk.internal.util.Preconditions.outOfBounds(Preconditions.java:98)
	at jdk.internal.util.Preconditions.outOfBoundsCheckFromToIndex(Preconditions.java:112)
	at jdk.internal.util.Preconditions.checkFromToIndex(Preconditions.java:349)
	at java.lang.String.checkBoundsBeginEnd(String.java:4963)
	at java.lang.String.substring(String.java:2925)
	at samples.javacore.unhandledexception.UnhandledExceptionExample.fun3(UnhandledExceptionExample.java:18)
	at samples.javacore.unhandledexception.UnhandledExceptionExample.fun2(UnhandledExceptionExample.java:13)
	at samples.javacore.unhandledexception.UnhandledExceptionExample.fun1(UnhandledExceptionExample.java:9)
	at samples.javacore.unhandledexception.UnhandledExceptionExample.main(UnhandledExceptionExample.java:5)

"#,
    );
}

#[cfg(not(windows))] // todo: fix this test on Windows
#[test]
fn should_print_help_message() {
    let expected_stdout = r#"Usage: rusty-jvm [options] <mainclass> [args...]

Options:
    -D<name>=<value>  Set a system property
    -X<option>        JVM options
    -XX:<option>      Advanced JVM options
    --<option>        Java launcher options
    -<option>         Java standard options
    -h, --help        Show this help message

Installation options:
    --install         Download and install standard libraries
    --purge           Remove all versions of installed rusty-jvm standard libraries
    --yes             Automatically say "yes" to prompts
"#;
    utils::assert_with_all_args(
        &["--help"],
        "",
        &[],
        expected_stdout,
        "",
        utils::ExecutionResult::Success,
    );
}
