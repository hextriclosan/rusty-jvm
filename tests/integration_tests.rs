mod utils;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
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
        r#"=== POSITIVE CASES ===
positive_intArrayCopy: [1, 2, 20, 30, 40]
positive_longArrayCopy: [1, 2, 128849018920, 214748364860, 300647710800, 6]
positive_overlappingInt: [10, 10, 20, 30, 40, 50]
positive_overlappingLong: [42949672980, 42949672980, 128849018920, 214748364860, 300647710800, 386547056740]
positive_upcasting: [10, 20, 30]
positive_runtimeCompatible: [3.14, 2.71, 1.41]
=== NEGATIVE CASES ===
negative_nullSrc: java.lang.NullPointerException
negative_nullDest: java.lang.NullPointerException
negative_srcNotArray: java.lang.ArrayStoreException: arraycopy: source type java.lang.String is not an array
negative_destNotArray: java.lang.ArrayStoreException: arraycopy: destination type java.lang.Long is not an array
negative_srcPrimitive_destReference: java.lang.ArrayStoreException: arraycopy: type mismatch: can not copy long[] into object array[]
negative_srcReference_destPrimitive: java.lang.ArrayStoreException: arraycopy: type mismatch: can not copy object array[] into long[]
negative_differentPrimitiveTypes: java.lang.ArrayStoreException: arraycopy: type mismatch: can not copy long[] into int[]
negative_incompatibleReferences: java.lang.ArrayStoreException: arraycopy: element type mismatch: can not cast one of the elements of java.lang.Number[] to the type of the destination array, java.lang.Double destination after copying: [3.0, 2.0, 1.41]
negative_srcPosNegative: java.lang.ArrayIndexOutOfBoundsException: arraycopy: source index -1 out of bounds for long[3]
negative_destPosNegative: java.lang.ArrayIndexOutOfBoundsException: arraycopy: destination index -1 out of bounds for long[3]
negative_lengthNegative: java.lang.ArrayIndexOutOfBoundsException: arraycopy: length -1 is negative
negative_srcPosPlusLengthTooBig: java.lang.ArrayIndexOutOfBoundsException: arraycopy: last source index 4 out of bounds for long[3]
negative_destPosPlusLengthTooBig: java.lang.ArrayIndexOutOfBoundsException: arraycopy: last destination index 4 out of bounds for long[3]
"#,
    );
}

use crate::utils::{
    assert_failure, assert_file_with_args, assert_success_with_args, assert_success_with_stderr,
    get_file_separator, get_os_name, get_output, get_output_with_raw_args, get_path_separator,
    is_bigendian, line_ending, map_library_name, tmp_file, ExecutionResult,
};
use itertools::Itertools;
use rand::Rng;
use regex::Regex;
use rstest::rstest;
use serde_json::Value;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tempfile::NamedTempFile;

#[test]
fn should_do_native_call_on_system_current_time() {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let expected_millis = since_the_epoch.as_millis() as i64;
    let expected_nanos = since_the_epoch.as_nanos() as i64;

    let output = get_output("samples.nativecall.system.NativeCallSystemCurrentTimeMillis");
    let json: Value = serde_json::from_str(&output).expect("Output is not valid JSON");

    let actual_millis = json["currentTimeMillis"]
        .as_i64()
        .expect("currentTimeMillis is not an integer");
    let actual_nanos = json["nanoTime"]
        .as_i64()
        .expect("nanoTime is not an integer");

    let timeout_mins = 10;
    let timeout_millis = Duration::from_secs(timeout_mins * 60).as_millis() as i64;
    let timeout_nanos = Duration::from_secs(timeout_mins * 60).as_nanos() as i64;
    assert!((expected_millis..expected_millis + timeout_millis).contains(&actual_millis));
    assert!((expected_nanos..expected_nanos + timeout_nanos).contains(&actual_nanos));
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
fn should_support_advanced_strings_concat() {
    assert_success(
        "samples.javacore.strings.concat.advanced.StringConcatTest",
        r#"Compile-time constant folding: Hello, World!
Concatenation with primitives: Value: 42, Pi: 3.14, Flag: true, Letter: ☺
Concatenation in loop: 01234
Concatenation with null: null test
Concatenation with custom object: Result: CustomObject
Concatenation with final variables: foobar
Concatenation with multiple plus: abcde
Compile-time constant folding passed: true
"#,
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
    assert_success(
        "samples.reflection.trivial.TrivialReflection",
        r#"Class: class java.lang.Class
  modifiers: 17
  isPrimitive: false
  isArray: false
  isInterface: false
  isEnum: false
  isAnnotation: false
  isSynthetic: false
  isRecord: false
  isEnumeration: false
Class: class java.lang.Object
  modifiers: 1
  isPrimitive: false
  isArray: false
  isInterface: false
  isEnum: false
  isAnnotation: false
  isSynthetic: false
  isRecord: false
  isEnumeration: false
Class: class samples.reflection.trivial.Examinee
  modifiers: 1024
  isPrimitive: false
  isArray: false
  isInterface: false
  isEnum: false
  isAnnotation: false
  isSynthetic: false
  isRecord: false
  isEnumeration: false
Class: interface samples.reflection.trivial.ExamineeInterface
  modifiers: 1536
  isPrimitive: false
  isArray: false
  isInterface: true
  isEnum: false
  isAnnotation: false
  isSynthetic: false
  isRecord: false
  isEnumeration: false
Class: class samples.reflection.trivial.Rcd
  modifiers: 16
  isPrimitive: false
  isArray: false
  isInterface: false
  isEnum: false
  isAnnotation: false
  isSynthetic: false
  isRecord: true
  isEnumeration: false
Class: class samples.reflection.trivial.Enm
  modifiers: 16400
  isPrimitive: false
  isArray: false
  isInterface: false
  isEnum: true
  isAnnotation: false
  isSynthetic: false
  isRecord: false
  isEnumeration: true
Class: int
  modifiers: 1041
  isPrimitive: true
  isArray: false
  isInterface: false
  isEnum: false
  isAnnotation: false
  isSynthetic: false
  isRecord: false
  isEnumeration: false
"#,
    );
}

#[test]
fn should_do_trivial_reflection_with_primitives() {
    assert_success(
        "samples.reflection.trivial.synthetic.classes.SyntheticPrimitiveClasses",
        r#"==== int ====
Name: int
Type name: int
Simple name: int
Modifiers: 1041 (public abstract final)
isPrimitive: true
isSynthetic: false
Package: null
Declaring class: null
Wrapper type: class java.lang.Integer

==== double ====
Name: double
Type name: double
Simple name: double
Modifiers: 1041 (public abstract final)
isPrimitive: true
isSynthetic: false
Package: null
Declaring class: null
Wrapper type: class java.lang.Double

==== boolean ====
Name: boolean
Type name: boolean
Simple name: boolean
Modifiers: 1041 (public abstract final)
isPrimitive: true
isSynthetic: false
Package: null
Declaring class: null
Wrapper type: class java.lang.Boolean

==== char ====
Name: char
Type name: char
Simple name: char
Modifiers: 1041 (public abstract final)
isPrimitive: true
isSynthetic: false
Package: null
Declaring class: null
Wrapper type: class java.lang.Character

==== byte ====
Name: byte
Type name: byte
Simple name: byte
Modifiers: 1041 (public abstract final)
isPrimitive: true
isSynthetic: false
Package: null
Declaring class: null
Wrapper type: class java.lang.Byte

==== short ====
Name: short
Type name: short
Simple name: short
Modifiers: 1041 (public abstract final)
isPrimitive: true
isSynthetic: false
Package: null
Declaring class: null
Wrapper type: class java.lang.Short

==== long ====
Name: long
Type name: long
Simple name: long
Modifiers: 1041 (public abstract final)
isPrimitive: true
isSynthetic: false
Package: null
Declaring class: null
Wrapper type: class java.lang.Long

==== float ====
Name: float
Type name: float
Simple name: float
Modifiers: 1041 (public abstract final)
isPrimitive: true
isSynthetic: false
Package: null
Declaring class: null
Wrapper type: class java.lang.Float

==== void ====
Name: void
Type name: void
Simple name: void
Modifiers: 1041 (public abstract final)
isPrimitive: true
isSynthetic: false
Package: null
Declaring class: null
Wrapper type: class java.lang.Void

=== Proof of materialization ===
c1 == c2: true
System.identityHashCode(c1) == System.identityHashCode(c2): true
Lookup by c2: Primitive int
o.getClass(): class java.lang.Class
"#,
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
    let big_endian = if is_bigendian() { 1 } else { 0 };
    let short_bytes = get_ne_bytes_as_string(&24583i16.to_ne_bytes());
    let char_bytes = get_ne_bytes_as_string(&('ї' as u16).to_ne_bytes());
    let int_bytes = get_ne_bytes_as_string(&1611079230i32.to_ne_bytes());
    let long_bytes = get_ne_bytes_as_string(&6919532605457772126i64.to_ne_bytes());
    assert_success(
        "samples.jdkinternal.unsafe.trivial.UnsafeUsage",
        &format!(
            r#"isBigEndian: {big_endian}
bytes: [0, 0, 0]
examinee.field3 value got by offset is: 30
examinee.field3 updated by offset: 40
examinee.field3 was not updated: 40
examinee.field4 value got by offset is: FIELD4
examinee.field4 updated by offset: FIELD4_UPDATED
examinee.field4 was not updated: FIELD4_UPDATED
examinee.field5 value got by offset is: 42949672980
compareAndSetLong on field examinee.field5: updated=true currentValue=128849018920
compareAndSetLong on field examinee.field5: updated=false currentValue=128849018920
examinee.field5 value got by offset is: 42949672980
compareAndExchangeLong on field examinee.field5: oldValue=42949672980 currentValue=128849018920
compareAndExchangeLong on field examinee.field5: oldValue=128849018920 currentValue=128849018920
examinees[1] got by offset is `two`: true
examinees[1] updated by offset and set to `three`: true
examinees[1] was not updated and remains the same: true
one.field4 updated by offset to: FIELD4_PUT_REFERENCE_VOLATILE
Class<Integer>.name update new value is java.lang.Positron
State restored. Name is now: java.lang.Integer
Examinee.staticField as java.lang.reflect.Field: static java.lang.String samples.jdkinternal.unsafe.trivial.Examinee.staticField
staticFieldBase: class samples.jdkinternal.unsafe.trivial.Examinee
Current static value: staticFieldValue
New value set via putReference(...): staticFieldNewValue
State restored. Static value is now: staticFieldValue
examinee.field3 value got by offset is: 30
putInt on field examinee.field3: currentValue=1337
examinee.field3 value got by offset is: 30
putIntVolatile on field examinee.field3: currentValue=1337
examinee.field5 value got by offset is: 42949672980
putLong on field examinee.field5: currentValue=128849018920
[0, 0, 0, 0, 0, 100, 0, 0, 0, 0, 0]
byte at index 5 is: 100
[0, 0, 0, 0, 0, 10000, 0, 0, 0, 0, 0]
short at index 5 is: 10000
[0, {short_bytes}, 0]
short at index 1 is: 24583
[a, a, a, a, a, b, a, a, a, a, a]
char at index 5 is: b
[0, {char_bytes}, 0]
char at index 1 is: ї
[0, 0, 0, 0, 0, 1000000000, 0, 0, 0, 0, 0]
int at index 5 is: 1000000000
[0, {int_bytes}, 0]
int at index 1 is: 1611079230
[0, 0, 0, 0, 0, 1000000000000, 0, 0, 0, 0, 0]
long at index 5 is: 1000000000000
[0, {long_bytes}, 0]
long at index 1 is: 6919532605457772126
"#,
        ),
    );
}
fn get_ne_bytes_as_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| b.to_string()).join(", ")
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
Filled: [10, -42, -42, -42, -42, -42, -42, -42, -42, 100]
Sorted: [-42, -42, -42, -42, -42, -42, -42, -42, 10, 100]
Integer[] third: [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
Sorted Integer[] third in reverse order: [100, 90, 80, 70, 60, 50, 40, 30, 20, 10]
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
    let (file_path, _guard) = tmp_file("test.txt");
    assert_file_with_args(
        "samples.io.fileoutputstreamexample.FileOutputStreamExample",
        &[&file_path],
        &file_path,
        "CAFEBABE",
        "",
    )
}

#[test]
fn should_support_file_output_stream_exceptions() {
    let (file_path, tmp_dir) = tmp_file("test.txt");
    let dir_path = tmp_dir.as_ref().display().to_string();

    let msg = if cfg!(windows) {
        "Access is denied. (os error 5)"
    } else {
        "Is a directory (os error 21)"
    };
    let expected_stdout = format!(
        r#"openFile: java.io.FileNotFoundException: {dir_path} ({msg})
writeByte: java.io.IOException: Stream Closed
writeBytes: java.io.IOException: Stream Closed
"#
    );
    assert_file_with_args(
        "samples.io.fileoutputstreamthrowexample.FileOutputStreamThrowExample",
        &[&file_path],
        &file_path,
        "A",
        &expected_stdout,
    )
}

#[test]
fn should_write_file_to_fs_with_buffered_stream() {
    let (file_path, _guard) = tmp_file("buffered_output.txt");
    assert_file_with_args(
        "samples.io.bufferedoutputstreamchunkingexample.BufferedOutputStreamChunkingExample",
        &[&file_path],
        &file_path,
        "This is a test for BufferedOutputStream chunking.",
        "",
    )
}

#[test]
fn should_write_file_with_print_stream() {
    let (file_path, _guard) = tmp_file("print_stream_test.txt");
    let expected_file_content = r#"Hello, PrintStream!
First Line
Second Line
Third Line
Hello Alice, you are 30 years old.
This will go to the file instead of the console.
Hello as raw bytes
Person{name='John', age=25}
This is written immediately. This follows after flush.
This is an example of chaining PrintStreams.
"#;
    assert_file_with_args(
        "samples.io.printstreamexample.PrintStreamExample",
        &[&file_path],
        &file_path,
        expected_file_content,
        "",
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
Is TimeUnit enum: true
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
samples.reflection.trivial.classgetdeclaringclassexample.GetDeclaringClassExample$$Lambda/0x0000: null
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
        r#"Class: samples.reflection.trivial.declaredmethods.DeclaredMethodsExample
All declared methods:
Information about method:sampleMethod
------------------------------------------------
String representation:private native java.lang.String[] samples.reflection.trivial.declaredmethods.DeclaredMethodsExample.sampleMethod(java.lang.String,int,double[]) throws java.io.IOError,java.lang.NullPointerException
Class:class samples.reflection.trivial.declaredmethods.DeclaredMethodsExample
Return Type:class [Ljava.lang.String;
Modifiers:private transient native
Parameter Count:3
Parameter Types:
 - java.lang.String
 - int
 - [D
Generic Parameter Types:
 - java.lang.String
 - int
 - double[]
Is Synthetic:false
Is Default:false
Is Bridge:false
Exception Types:
 - java.io.IOError
 - java.lang.NullPointerException
Generic Exception Types:
 - java.io.IOError
 - java.lang.NullPointerException
Is VarArgs:true

Information about method:lambda$print$0
------------------------------------------------
String representation:private static int samples.reflection.trivial.declaredmethods.DeclaredMethodsExample.lambda$print$0(java.lang.reflect.Method,java.lang.reflect.Method)
Class:class samples.reflection.trivial.declaredmethods.DeclaredMethodsExample
Return Type:int
Modifiers:private static
Parameter Count:2
Parameter Types:
 - java.lang.reflect.Method
 - java.lang.reflect.Method
Generic Parameter Types:
 - java.lang.reflect.Method
 - java.lang.reflect.Method
Is Synthetic:true
Is Default:false
Is Bridge:false
Exception Types:
Generic Exception Types:
Is VarArgs:false

Information about method:print
------------------------------------------------
String representation:private static void samples.reflection.trivial.declaredmethods.DeclaredMethodsExample.print(java.lang.reflect.Method)
Class:class samples.reflection.trivial.declaredmethods.DeclaredMethodsExample
Return Type:void
Modifiers:private static
Parameter Count:1
Parameter Types:
 - java.lang.reflect.Method
Generic Parameter Types:
 - java.lang.reflect.Method
Is Synthetic:false
Is Default:false
Is Bridge:false
Exception Types:
Generic Exception Types:
Is VarArgs:false

Information about method:print
------------------------------------------------
String representation:private static void samples.reflection.trivial.declaredmethods.DeclaredMethodsExample.print(java.lang.reflect.Method[])
Class:class samples.reflection.trivial.declaredmethods.DeclaredMethodsExample
Return Type:void
Modifiers:private static
Parameter Count:1
Parameter Types:
 - [Ljava.lang.reflect.Method;
Generic Parameter Types:
 - java.lang.reflect.Method[]
Is Synthetic:false
Is Default:false
Is Bridge:false
Exception Types:
Generic Exception Types:
Is VarArgs:false

Information about method:main
------------------------------------------------
String representation:public static void samples.reflection.trivial.declaredmethods.DeclaredMethodsExample.main(java.lang.String[])
Class:class samples.reflection.trivial.declaredmethods.DeclaredMethodsExample
Return Type:void
Modifiers:public static
Parameter Count:1
Parameter Types:
 - [Ljava.lang.String;
Generic Parameter Types:
 - java.lang.String[]
Is Synthetic:false
Is Default:false
Is Bridge:false
Exception Types:
Generic Exception Types:
Is VarArgs:false


Public only methods:
Information about method:equals
------------------------------------------------
String representation:public boolean java.lang.Object.equals(java.lang.Object)
Class:class java.lang.Object
Return Type:boolean
Modifiers:public
Parameter Count:1
Parameter Types:
 - java.lang.Object
Generic Parameter Types:
 - java.lang.Object
Is Synthetic:false
Is Default:false
Is Bridge:false
Exception Types:
Generic Exception Types:
Is VarArgs:false

Information about method:getClass
------------------------------------------------
String representation:public final native java.lang.Class java.lang.Object.getClass()
Class:class java.lang.Object
Return Type:class java.lang.Class
Modifiers:public final native
Parameter Count:0
Parameter Types:
Generic Parameter Types:
Is Synthetic:false
Is Default:false
Is Bridge:false
Exception Types:
Generic Exception Types:
Is VarArgs:false

Information about method:notify
------------------------------------------------
String representation:public final native void java.lang.Object.notify()
Class:class java.lang.Object
Return Type:void
Modifiers:public final native
Parameter Count:0
Parameter Types:
Generic Parameter Types:
Is Synthetic:false
Is Default:false
Is Bridge:false
Exception Types:
Generic Exception Types:
Is VarArgs:false

Information about method:notifyAll
------------------------------------------------
String representation:public final native void java.lang.Object.notifyAll()
Class:class java.lang.Object
Return Type:void
Modifiers:public final native
Parameter Count:0
Parameter Types:
Generic Parameter Types:
Is Synthetic:false
Is Default:false
Is Bridge:false
Exception Types:
Generic Exception Types:
Is VarArgs:false

Information about method:wait
------------------------------------------------
String representation:public final void java.lang.Object.wait() throws java.lang.InterruptedException
Class:class java.lang.Object
Return Type:void
Modifiers:public final
Parameter Count:0
Parameter Types:
Generic Parameter Types:
Is Synthetic:false
Is Default:false
Is Bridge:false
Exception Types:
 - java.lang.InterruptedException
Generic Exception Types:
 - java.lang.InterruptedException
Is VarArgs:false

Information about method:wait
------------------------------------------------
String representation:public final void java.lang.Object.wait(long) throws java.lang.InterruptedException
Class:class java.lang.Object
Return Type:void
Modifiers:public final
Parameter Count:1
Parameter Types:
 - long
Generic Parameter Types:
 - long
Is Synthetic:false
Is Default:false
Is Bridge:false
Exception Types:
 - java.lang.InterruptedException
Generic Exception Types:
 - java.lang.InterruptedException
Is VarArgs:false

Information about method:wait
------------------------------------------------
String representation:public final void java.lang.Object.wait(long,int) throws java.lang.InterruptedException
Class:class java.lang.Object
Return Type:void
Modifiers:public final
Parameter Count:2
Parameter Types:
 - long
 - int
Generic Parameter Types:
 - long
 - int
Is Synthetic:false
Is Default:false
Is Bridge:false
Exception Types:
 - java.lang.InterruptedException
Generic Exception Types:
 - java.lang.InterruptedException
Is VarArgs:false

Information about method:toString
------------------------------------------------
String representation:public java.lang.String java.lang.Object.toString()
Class:class java.lang.Object
Return Type:class java.lang.String
Modifiers:public
Parameter Count:0
Parameter Types:
Generic Parameter Types:
Is Synthetic:false
Is Default:false
Is Bridge:false
Exception Types:
Generic Exception Types:
Is VarArgs:false

Information about method:hashCode
------------------------------------------------
String representation:public native int java.lang.Object.hashCode()
Class:class java.lang.Object
Return Type:int
Modifiers:public native
Parameter Count:0
Parameter Types:
Generic Parameter Types:
Is Synthetic:false
Is Default:false
Is Bridge:false
Exception Types:
Generic Exception Types:
Is VarArgs:false

Information about method:main
------------------------------------------------
String representation:public static void samples.reflection.trivial.declaredmethods.DeclaredMethodsExample.main(java.lang.String[])
Class:class samples.reflection.trivial.declaredmethods.DeclaredMethodsExample
Return Type:void
Modifiers:public static
Parameter Count:1
Parameter Types:
 - [Ljava.lang.String;
Generic Parameter Types:
 - java.lang.String[]
Is Synthetic:false
Is Default:false
Is Bridge:false
Exception Types:
Generic Exception Types:
Is VarArgs:false

"#,
    );
}

#[test]
fn should_support_getting_declared_constructors() {
    assert_success(
        "samples.reflection.trivial.declaredconstructors.DeclaredConstructorsExample",
        r#"Basic Example Constructors:
--------------------------------
Constructor: public samples.reflection.trivial.declaredconstructors.BasicExample()
  Parameter types: []
  Modifier: public
  Throws: []
  Declaring class: class samples.reflection.trivial.declaredconstructors.BasicExample
  Name: samples.reflection.trivial.declaredconstructors.BasicExample
  Parameter count: 0
--------------------------------
Constructor: protected samples.reflection.trivial.declaredconstructors.BasicExample(int)
  Parameter types: [int]
  Modifier: protected
  Throws: []
  Declaring class: class samples.reflection.trivial.declaredconstructors.BasicExample
  Name: samples.reflection.trivial.declaredconstructors.BasicExample
  Parameter count: 1
--------------------------------
Constructor: private samples.reflection.trivial.declaredconstructors.BasicExample(java.lang.String)
  Parameter types: [class java.lang.String]
  Modifier: private
  Throws: []
  Declaring class: class samples.reflection.trivial.declaredconstructors.BasicExample
  Name: samples.reflection.trivial.declaredconstructors.BasicExample
  Parameter count: 1
Edge Case Example Constructors:
--------------------------------
Constructor: samples.reflection.trivial.declaredconstructors.EdgeCaseExample()
  Parameter types: []
  Modifier: 
  Throws: []
  Declaring class: class samples.reflection.trivial.declaredconstructors.EdgeCaseExample
  Name: samples.reflection.trivial.declaredconstructors.EdgeCaseExample
  Parameter count: 0
--------------------------------
Constructor: public samples.reflection.trivial.declaredconstructors.EdgeCaseExample(java.lang.String) throws java.lang.IllegalArgumentException
  Parameter types: [class java.lang.String]
  Modifier: public
  Throws: [class java.lang.IllegalArgumentException]
  Declaring class: class samples.reflection.trivial.declaredconstructors.EdgeCaseExample
  Name: samples.reflection.trivial.declaredconstructors.EdgeCaseExample
  Parameter count: 1
Handle Constructor Exceptions:
No such constructor found: samples.reflection.trivial.declaredconstructors.EdgeCaseExample.<init>(java.lang.Float)
Access Private Constructor:
Constructor: private samples.reflection.trivial.declaredconstructors.BasicExample(java.lang.String)
  Parameter types: [class java.lang.String]
  Modifier: private
  Throws: []
  Declaring class: class samples.reflection.trivial.declaredconstructors.BasicExample
  Name: samples.reflection.trivial.declaredconstructors.BasicExample
  Parameter count: 1
Private instance created: BasicExample{}
Anonymous Class Constructors:
--------------------------------
Constructor: samples.reflection.trivial.declaredconstructors.DeclaredConstructorsExample$1()
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
    assert!(user_dir.ends_with("java_classes_for_tests"));

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

    let other_property = json["other.property"]
        .as_str()
        .expect("other.property is not a string");
    assert_eq!(other_property, "null");

    let java_home = json["java.home"]
        .as_str()
        .expect("java.home is not a string");
    assert_eq!(
        java_home,
        env::var("JAVA_HOME").expect("JAVA_HOME is not set")
    );

    let sun_boot_library_path = json["sun.boot.library.path"]
        .as_str()
        .expect("sun.boot.library.path is not a string");

    let expected_sun_boot_library_path = if cfg!(windows) {
        format!(
            "{}\\bin",
            env::var("JAVA_HOME").expect("JAVA_HOME is not set")
        )
    } else {
        format!(
            "{}/lib",
            env::var("JAVA_HOME").expect("JAVA_HOME is not set")
        )
    };
    assert_eq!(sun_boot_library_path, expected_sun_boot_library_path);

    let tmp_dir = json["java.io.tmpdir"]
        .as_str()
        .expect("java.io.tmpdir is not a string");
    assert_eq!(
        tmp_dir,
        env::temp_dir().to_str().expect("tmp_dir is not UTF-8")
    );
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
------- arrayElementVarHandle (int[] array elements) -------
(SampleClass)int - null: 200
Modified array: [100, 200, 300, 40, 50]
------- bindTo (String.length) -------
()int - MethodHandle()int: 26
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
// It's not so nice but a bad test is better than no test
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
A lambda 1 (class samples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample$$Lambda/0x0000): class samples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample
A lambda 2 (class samples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample$$Lambda/0x0000): class samples.reflection.trivial.classgetnesthostexample.ClassGetNestHostExample
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
class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample and class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample$$Lambda/0x0000 are nest mates: true
class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample and class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample$$Lambda/0x0000 are nest mates: true
class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample and class java.lang.String are nest mates: false
class samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample and class [Lsamples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample; are nest mates: false
"#,
    );
}

#[test]
fn should_support_class_for_name_method() {
    assert_success(
        "samples.reflection.trivial.forname.ClassForNameExample",
        r#">>> First static block executed
Loaded class: samples.reflection.trivial.forname.First (initialized=true)
Loaded class: samples.reflection.trivial.forname.Second (initialized=false)
ClassNotFoundException: samples.reflection.trivial.forname.NonExisting - java.lang.ClassNotFoundException: samples.reflection.trivial.forname.NonExisting
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
fn should_return_random_number() {
    assert_success("samples.javautil.random.RandomExample", "666\n");
}

#[test]
fn should_work_with_records() {
    assert_success(
        "samples.javacore.recordexample.RecordExample",
        r#"one: Rcd[first=10, second=20]
two: Rcd[first=100, second=200]
three: Rcd[first=10, second=20]
hashOne: 330
hashTwo: 3300
hashThree: 330
Rcd[first=10, second=20] equals Rcd[first=100, second=200]: false
Rcd[first=10, second=20] equals Rcd[first=10, second=20]: true
"#,
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
        r#"VM execution failed: Execution Error: Reason: Execution Error: Exceeded max stack size; Current Frame: StackFrame { method_name: "returnInt:()I", pc: 5, ex_pc: None, locals: [], operand_stack: Stack { max_size: 2, data: [10, 20] }, bytecode_ref: [18, 7, 18, 8, 18, 9, 172], current_class_name: "samples/invalidprograms/operandstackoverflow/OperandStackOverflowExample", line_numbers: {}, exception_table: ExceptionTable { table: [] } }
"#,
    );
}

#[test]
fn should_support_file_dispatcher_for_various_os() {
    let (file_path, _guard) = tmp_file("file_dispatcher_example.txt");
    assert_file_with_args(
        "samples.nio.filedispatcherexample.FileDispatcherExample",
        &[&file_path],
        &file_path,
        "Hello from FileChannel!",
        "Read: Hello from FileChannel!\n",
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
Caught as Throwable: java.lang.Error: This is an error. cause=null stackTrace=[samples.javacore.exceptionexample.FewTriesInOneMethod.thrower(ExceptionExample.java:72), samples.javacore.exceptionexample.FewTriesInOneMethod.runImpl(ExceptionExample.java:54), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:32), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:21)] suppressed=[]
Inside another try block
Caught as Throwable second time: java.lang.IndexOutOfBoundsException: This is an index out of bounds exception. cause=null stackTrace=[samples.javacore.exceptionexample.FewTriesInOneMethod.runImpl(ExceptionExample.java:65), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:32), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:21)] suppressed=[]

Running case: TryWithResources
Inside try-with-resources block
Doing something with the resource
Custom resource is about to be closed
Custom resource is closed
Caught try-with-resources exception: java.lang.RuntimeException: An error occurred while using the resource. cause=null stackTrace=[samples.javacore.exceptionexample.CustomResource.doSomethingAndThrow(ExceptionExample.java:247), samples.javacore.exceptionexample.TryWithResources.tryWithResourcesWithNoCatch(ExceptionExample.java:95), samples.javacore.exceptionexample.TryWithResources.runImpl(ExceptionExample.java:87), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:32), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:21)] suppressed=[]

Running case: TryWithResources
Inside try-with-resources block
Doing something with the resource
Custom resource is about to be closed
Caught try-with-resources exception: java.lang.RuntimeException: An error occurred while using the resource. cause=null stackTrace=[samples.javacore.exceptionexample.CustomResource.doSomethingAndThrow(ExceptionExample.java:247), samples.javacore.exceptionexample.TryWithResources.tryWithResourcesWithNoCatch(ExceptionExample.java:95), samples.javacore.exceptionexample.TryWithResources.runImpl(ExceptionExample.java:87), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:32), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:21)] suppressed=[java.lang.IllegalStateException: An error occurred while closing the resource]

Running case: TryWithResourcesMimic
Inside try-with-resources block
Doing something with the resource
Custom resource is about to be closed
Custom resource is closed
Caught try-with-resources exception: java.lang.RuntimeException: An error occurred while using the resource. cause=null stackTrace=[samples.javacore.exceptionexample.CustomResource.doSomethingAndThrow(ExceptionExample.java:247), samples.javacore.exceptionexample.TryWithResourcesMimic.tryWithResourcesMimic(ExceptionExample.java:121), samples.javacore.exceptionexample.TryWithResourcesMimic.runImpl(ExceptionExample.java:111), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:32), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:21)] suppressed=[]

Running case: TryWithResourcesMimic
Inside try-with-resources block
Doing something with the resource
Custom resource is about to be closed
Caught try-with-resources exception: java.lang.RuntimeException: An error occurred while using the resource. cause=null stackTrace=[samples.javacore.exceptionexample.CustomResource.doSomethingAndThrow(ExceptionExample.java:247), samples.javacore.exceptionexample.TryWithResourcesMimic.tryWithResourcesMimic(ExceptionExample.java:121), samples.javacore.exceptionexample.TryWithResourcesMimic.runImpl(ExceptionExample.java:111), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:32), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:21)] suppressed=[java.lang.IllegalStateException: An error occurred while closing the resource]

Running case: ReThrowWithCause
Inside try block
Caught as Throwable: java.lang.RuntimeException: This is a runtime exception. cause=null stackTrace=[samples.javacore.exceptionexample.ReThrowWithCause.runImpl(ExceptionExample.java:142), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:32), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:21)] suppressed=[]
Caught as IllegalStateException: java.lang.IllegalStateException: java.lang.RuntimeException: This is a runtime exception. cause=java.lang.RuntimeException: This is a runtime exception stackTrace=[samples.javacore.exceptionexample.ReThrowWithCause.runImpl(ExceptionExample.java:145), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:32), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:21)] suppressed=[]

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
Caught as Throwable: java.lang.RuntimeException: Exception in try. cause=null stackTrace=[samples.javacore.exceptionexample.FinallyIllustration.withUncaughtException(ExceptionExample.java:208), samples.javacore.exceptionexample.FinallyIllustration.runImpl(ExceptionExample.java:167), samples.javacore.exceptionexample.Case.run(ExceptionExample.java:32), samples.javacore.exceptionexample.ExceptionExample.main(ExceptionExample.java:21)] suppressed=[]
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

#[test]
fn should_print_info_about_unhandled_exception() {
    utils::assert_failure_with_stderr(
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
	at java.lang.String.checkBoundsBeginEnd(String.java:4937)
	at java.lang.String.substring(String.java:2899)
	at samples.javacore.unhandledexception.UnhandledExceptionExample.fun3(UnhandledExceptionExample.java:18)
	at samples.javacore.unhandledexception.UnhandledExceptionExample.fun2(UnhandledExceptionExample.java:13)
	at samples.javacore.unhandledexception.UnhandledExceptionExample.fun1(UnhandledExceptionExample.java:9)
	at samples.javacore.unhandledexception.UnhandledExceptionExample.main(UnhandledExceptionExample.java:5)

"#,
    );
}

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
    -V, --version     Show version information
"#;
    utils::assert_with_all_args(
        &["--help"],
        "",
        &[],
        expected_stdout,
        "",
        ExecutionResult::Success,
        0,
    );
}

#[test]
fn should_print_version_message() {
    let expected_stdout = "rusty-jvm 0.5.0\n";
    utils::assert_with_all_args(
        &["--version"],
        "",
        &[],
        expected_stdout,
        "",
        ExecutionResult::Success,
        0,
    );
}

#[test]
fn should_support_getting_declared_fields() {
    assert_success(
        "samples.reflection.trivial.declaredfields.DeclaredFieldsExample",
        r#"Class: samples.reflection.trivial.declaredfields.Examinee
All declared fields:
Information about field: publicField
------------------------------------------------
String representation: public int samples.reflection.trivial.declaredfields.Examinee.publicField
Class: class samples.reflection.trivial.declaredfields.Examinee
Modifiers: public
Type: int
Is Synthetic: false

Information about field: publicStaticField
------------------------------------------------
String representation: public static java.lang.Object samples.reflection.trivial.declaredfields.Examinee.publicStaticField
Class: class samples.reflection.trivial.declaredfields.Examinee
Modifiers: public static
Type: class java.lang.Object
Is Synthetic: false

Information about field: protectedField
------------------------------------------------
String representation: protected java.lang.String samples.reflection.trivial.declaredfields.Examinee.protectedField
Class: class samples.reflection.trivial.declaredfields.Examinee
Modifiers: protected
Type: class java.lang.String
Is Synthetic: false

Information about field: privateField
------------------------------------------------
String representation: private double samples.reflection.trivial.declaredfields.Examinee.privateField
Class: class samples.reflection.trivial.declaredfields.Examinee
Modifiers: private
Type: double
Is Synthetic: false

Information about field: packagePrivateFinalField
------------------------------------------------
String representation: final int[] samples.reflection.trivial.declaredfields.Examinee.packagePrivateFinalField
Class: class samples.reflection.trivial.declaredfields.Examinee
Modifiers: final
Type: class [I
Is Synthetic: false

Information about field: staticFinalField
------------------------------------------------
String representation: static final java.lang.String samples.reflection.trivial.declaredfields.Examinee.staticFinalField
Class: class samples.reflection.trivial.declaredfields.Examinee
Modifiers: static final
Type: class java.lang.String
Is Synthetic: false

Information about field: staticNonFinalField
------------------------------------------------
String representation: static java.util.Map samples.reflection.trivial.declaredfields.Examinee.staticNonFinalField
Class: class samples.reflection.trivial.declaredfields.Examinee
Modifiers: static
Type: interface java.util.Map
Is Synthetic: false

Information about field: transientField
------------------------------------------------
String representation: transient java.lang.String samples.reflection.trivial.declaredfields.Examinee.transientField
Class: class samples.reflection.trivial.declaredfields.Examinee
Modifiers: transient
Type: class java.lang.String
Is Synthetic: false

Public only fields:
Information about field: publicField
------------------------------------------------
String representation: public int samples.reflection.trivial.declaredfields.Examinee.publicField
Class: class samples.reflection.trivial.declaredfields.Examinee
Modifiers: public
Type: int
Is Synthetic: false

Information about field: publicStaticField
------------------------------------------------
String representation: public static java.lang.Object samples.reflection.trivial.declaredfields.Examinee.publicStaticField
Class: class samples.reflection.trivial.declaredfields.Examinee
Modifiers: public static
Type: class java.lang.Object
Is Synthetic: false

Class: samples.reflection.trivial.declaredfields.Examinee$MyInner
All declared fields:
Information about field: i
------------------------------------------------
String representation: int samples.reflection.trivial.declaredfields.Examinee$MyInner.i
Class: class samples.reflection.trivial.declaredfields.Examinee$MyInner
Modifiers: 
Type: int
Is Synthetic: false

Information about field: this$0
------------------------------------------------
String representation: final samples.reflection.trivial.declaredfields.Examinee samples.reflection.trivial.declaredfields.Examinee$MyInner.this$0
Class: class samples.reflection.trivial.declaredfields.Examinee$MyInner
Modifiers: final
Type: class samples.reflection.trivial.declaredfields.Examinee
Is Synthetic: true

Public only fields:
"#,
    );
}

#[test]
fn should_support_string_concat_helper() {
    assert_success("java.lang.StringConcatHelperExample", "abc\n");
}

#[test]
fn should_support_string_concat_factory() {
    assert_success(
        "samples.reflection.stringconcat.StringConcatFactoryExample",
        "abc\n",
    );
}

#[test]
fn should_support_bootstrap_method_invoker() {
    assert_success(
        "java.lang.invoke.BootstrapMethodInvokerExample",
        r#"--- Simulating `invokedynamic` for concat('a','b','c') ---
--- Calling BootstrapMethodInvoker.invoke directly ---
CallSite's Dynamic Invoker: MethodHandle(String,String,String)String
--- Executing the final linked MethodHandle ---
Result of invokeExact("a", "b", "c"): abc

--- Simulating `invokedynamic` for custom bootstrap method ---
[BSM]: customBootstrap called!
  > invokedName: myCustomAdd
  > invokedType: (int,int)int
  > Linking to target: MethodHandle(int,int)int
Executing add(10, 20)
Custom BSM Result: 30

--- Simulating `invokedynamic` for lambda expression ---
Executing lambda body: format('User', 123)
Lambda Result: User-123
"#,
    );
}

#[test]
fn should_support_composite_pattern() {
    assert_success(
        "samples.patterns.composite.CompositePattern",
        r#"Assassin(100) says: Target acquired.
Assassin(110) says: Target acquired.
Soldier(2) says: For honor!
Army attack power is 222
"#,
    );
}

#[test]
fn should_support_lambdas() {
    assert_success(
        "samples.javacore.invokedynamic.lambda.LambdaExample",
        r#"Length: 13
invoke dynamic
Hello from method reference!
Result: 15
Result: 5
Result: 50
Result: 2
Joined numbers: 1, 2, 3, 4
1 2 3.0 true 4 5.0 test 10
Captured: 1, 2, 3.0, true, 4, 5.0, test, 10
startsWith 'dyn'? true
BigInteger add: 123
Sorted: apple
Sorted: banana
Sorted: cherry
10 * 7 = 70
Triple 6: 18
"#,
    );
}

#[test]
fn should_support_regex() {
    assert_success(
        "samples.regex.regexexample.RegexExample",
        r#"Full match: support@example.com
Username: support
Domain: example.com
---
Full match: sales@example.org.
Username: sales
Domain: example.org.
---
Censored text: Contact us at [email hidden] or [email hidden]
User: Alice
  Email: alice@example.com
  Username: alice
  Domain: example.com
  > Corporate email detected!

User: Bob
  Email: bob@EXAMPLE.org
  Username: bob
  Domain: EXAMPLE.org

User: Carol
  Email: carol@sub.example.co.uk
  Username: carol
  Domain: sub.example.co.uk

"#,
    );
}

#[test]
fn should_support_java_streams() {
    assert_success(
        "samples.javacore.streams.streamexamples.StreamExamples",
        r#"a
b
c
Lengths: [5, 3, 7]
Even numbers: [0, 2, 4, 6, 8]
Sum 1 to 5: 15
Joined: a-b-c
Grouped: {a=[apple, apricot], b=[banana, blueberry]}
Flattened: [a, b, c, d, e]
First 5 squares: [1, 4, 9, 16, 25]
Custom collected: APPLE BANANA PEAR
Product (parallel): 362880
"#,
    );
}

#[test]
fn should_rethrow_an_exception() {
    assert_success(
        "samples.javacore.rethrowloopexample.RethrowExample",
        "Caught in main: java.lang.AssertionError: boom\n",
    );
}

#[test]
fn should_support_class_get_declared_method() {
    assert_success(
        "samples.reflection.getdeclaredmethod.GetDeclaredMethodExample",
        r#"ofMethod found: public static samples.reflection.getdeclaredmethod.Person samples.reflection.getdeclaredmethod.Person.of(java.lang.String,int,java.lang.String[])
Person created: Person{name='Alice', age=25, hobbies=Reading, Hiking}
ageMethod found: private java.lang.String samples.reflection.getdeclaredmethod.Person.getAgeAsString()
Age as String: 25
formatNameMethod found: private java.lang.String samples.reflection.getdeclaredmethod.Person.formatName(java.lang.String)
Result: Dr. Alice
setAgeMethod found: public void samples.reflection.getdeclaredmethod.Person.setAge(int)
setHobbiesMethod found: public void samples.reflection.getdeclaredmethod.Person.setHobbies(java.lang.String[])
Updated Person: Person{name='Alice', age=30, hobbies=Swimming, Cycling}
"#,
    );
}

#[test]
fn should_support_class_get_declared_constructor() {
    assert_success(
        "samples.reflection.getdeclaredconstructorexample.GetDeclaredConstructorExample",
        r#"Person() found: private samples.reflection.getdeclaredconstructorexample.Person()
Created: John Doe (0) - Hobbies: []
Person(String name, int age) found: private samples.reflection.getdeclaredconstructorexample.Person(java.lang.String)
Created: Max (0) - Hobbies: []
Person(String name, int age) found: private samples.reflection.getdeclaredconstructorexample.Person(java.lang.String,int)
Created: Alice (30) - Hobbies: []
Person(String name, String[] hobbies) found: private samples.reflection.getdeclaredconstructorexample.Person(java.lang.String,java.lang.String[])
Created: Deborah (0) - Hobbies: [Cycling, Cooking]
Person(String name, int age, String[] hobbies) found: private samples.reflection.getdeclaredconstructorexample.Person(java.lang.String,int,java.lang.String[])
Created: Michael (42) - Hobbies: [Reading, Hiking]
"#,
    );
}

#[test]
fn should_support_compression_decompression() {
    assert_success(
        "samples.zlib.zlibexample.ZlibExample",
        r#"Original size: 35000 bytes
Original CRC32: 639401744
=== Wrapped DEFLATE ===
Level -1, Compressed size: 82, Compression ratio: 0.2342857142857143%, Decompressed CRC32: 639401744, Correct: true
Level 9, Compressed size: 82, Compression ratio: 0.2342857142857143%, Decompressed CRC32: 639401744, Correct: true
=== Raw DEFLATE (nowrap=true) ===
Level -1, Compressed size: 76, Compression ratio: 0.21714285714285714%, Decompressed CRC32: 639401744, Correct: true
Level 9, Compressed size: 76, Compression ratio: 0.21714285714285714%, Decompressed CRC32: 639401744, Correct: true
"#,
    );
}

#[test]
fn should_invoke_constructor_via_reflection() {
    assert_success(
        "samples.reflection.directconstructorhandleaccessor.DirectConstructorHandleAccessorExample",
        r#"Target{byteField=-128, booleanField=true, shortField=-32768, charField=ї, intField=2000000000, floatField=3.14, longField=-9223372036854775808, doubleField=3.141592653589793, stringField='seven', objectField=[1, 2, 3], mapField={8=eight}}
Target{byteField=127, booleanField=true, shortField=32767, charField=є, intField=2147483647, floatField=1.337, longField=9223372036854775807, doubleField=3.141592653589793, stringField='seventy', objectField=[10, 20, 30], mapField={80=eighty}}
"#);
}

#[test]
fn should_invoke_method_via_reflection() {
    assert_success(
        "samples.reflection.directmethodhandleaccessor.DirectMethodHandleAccessorExample",
        "TargetInstance - secretMethod invoked with: byte=-128, boolean=true, short=-32768, char=ї, int=2000000000, float=3.14, long=-9223372036854775808, double=3.141592653589793, string=seven, object=[1, 2, 3], map={8=eight}\n",
    );
}

#[test]
fn should_perform_lookup_in_interfaces_during_invokespecial_call() {
    assert_success(
        "samples.inheritance.defaultmethodviaparent.DefaultMethodViaParentExample",
        "42\n",
    );
}

#[test]
fn should_handle_null_pointer_exceptions() {
    assert_success(
        "samples.npe.allnpeexamples.AllNPEExamples",
        r#"Field access: Cannot read field "x" because "<VAR_NAME>" is null
Method call (invokevirtual): Cannot invoke "java/lang/String.length:()I" because "<VAR_NAME>" is null
Method call (invokeinterface): Cannot invoke "java/lang/Runnable.run:()V" because "<VAR_NAME>" is null
Method call on param: Cannot invoke "java/lang/String.length:()I" because "<VAR_NAME>" is null
Array length: Cannot read the array length because "<VAR_NAME>" is null
Array access: Cannot load from object array because "<VAR_NAME>" is null
Synchronization on null: Cannot enter synchronized block because "<VAR_NAME>" is null
Unboxing: Cannot invoke "java/lang/Integer.intValue:()I" because "<VAR_NAME>" is null
Throw null: Cannot throw exception because "<VAR_NAME>" is null
Var args: Cannot read the array length because "<VAR_NAME>" is null
Switch on null: Cannot invoke "java/lang/String.hashCode:()I" because "<VAR_NAME>" is null
Objects.requireNonNull: null
Objects.requireNonNull with message: Object must not be null
Method reference bound to null: null
Reflection field get: java.lang.NullPointerException: Cannot invoke "java/lang/Object.getClass:()Ljava/lang/Class;" because "<VAR_NAME>" is null
Reflection method invoke: java.lang.NullPointerException: Cannot invoke "java/lang/Object.getClass:()Ljava/lang/Class;" because "<VAR_NAME>" is null
"#,
    );
}

#[test]
fn should_support_file_input_stream() {
    use std::io::Write;
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    let expected_content = "I'm a sample file.";
    write!(file.as_file_mut(), "{}", expected_content).expect("Failed to write to temp file");

    assert_success_with_args(
        "samples.io.fileinputstreamexample.FileInputStreamExample",
        &[&file.path().display().to_string()],
        expected_content,
    );
}

#[test]
fn should_support_file_input_stream_exceptions() {
    let (file_path, tmp_dir) = tmp_file("test.txt");
    let dir_path = tmp_dir.as_ref().display().to_string();
    {
        let mut file = File::create(&file_path).expect("Failed to open temp file");
        file.write_all(b"Sample content")
            .expect("Failed to write to temp file");
    }

    let msg = if cfg!(windows) {
        "Access is denied. (os error 5)"
    } else {
        "Is a directory"
    };
    let expected_stdout = format!(
        r#"S
openFile: java.io.FileNotFoundException: {dir_path} ({msg})
readByte: java.io.IOException: Stream Closed
readBytes: java.io.IOException: Stream Closed
readAllBytes: java.io.IOException: Stream Closed
"#
    );
    assert_success_with_args(
        "samples.io.fileinputstreamthrowexample.FileInputStreamThrowExample",
        &[&file_path],
        &expected_stdout,
    )
}

#[test]
fn should_support_jep512() {
    assert_success(
        "CompactSourceFilesExample",
        r#"James: 5
Bill: 4
Guy: 3
Alex: 4
Dan: 3
Gavin: 5
"#,
    );
}

#[test]
fn should_support_jep512_args() {
    assert_success_with_args(
        "CompactSourceFilesArgsExample",
        &["Monica", "Eve", "Jessica"],
        r#"Monica: 6
Eve: 3
Jessica: 7
"#,
    );
}

#[test]
fn should_create_temp_file_and_parse_xml() {
    assert_success(
        "samples.io.xmlstringtofileexample.XmlStringToFileExample",
        r#"Root element: person
Name: John
Age: 30
"#,
    );
}

#[test]
fn should_return_info_about_modules() {
    assert_success(
        "samples.module.inspectmodule.InspectModule",
        r#"Class: class java.lang.String
  Module name: java.base
    ModuleDescriptor:
      name: java.base
      modifiers: []
      requires: []
      exports:
        com.sun.crypto.provider [jdk.crypto.cryptoki] []
        com.sun.security.ntlm [java.security.sasl] []
        java.io [] []
        java.lang [] []
        java.lang.annotation [] []
        java.lang.classfile [] []
        java.lang.classfile.attribute [] []
        java.lang.classfile.constantpool [] []
        java.lang.classfile.instruction [] []
        java.lang.constant [] []
        java.lang.foreign [] []
        java.lang.invoke [] []
        java.lang.module [] []
        java.lang.ref [] []
        java.lang.reflect [] []
        java.lang.runtime [] []
        java.math [] []
        java.net [] []
        java.net.spi [] []
        java.nio [] []
        java.nio.channels [] []
        java.nio.channels.spi [] []
        java.nio.charset [] []
        java.nio.charset.spi [] []
        java.nio.file [] []
        java.nio.file.attribute [] []
        java.nio.file.spi [] []
        java.security [] []
        java.security.cert [] []
        java.security.interfaces [] []
        java.security.spec [] []
        java.text [] []
        java.text.spi [] []
        java.time [] []
        java.time.chrono [] []
        java.time.format [] []
        java.time.temporal [] []
        java.time.zone [] []
        java.util [] []
        java.util.concurrent [] []
        java.util.concurrent.atomic [] []
        java.util.concurrent.locks [] []
        java.util.function [] []
        java.util.jar [] []
        java.util.random [] []
        java.util.regex [] []
        java.util.spi [] []
        java.util.stream [] []
        java.util.zip [] []
        javax.crypto [] []
        javax.crypto.interfaces [] []
        javax.crypto.spec [] []
        javax.net [] []
        javax.net.ssl [] []
        javax.security.auth [] []
        javax.security.auth.callback [] []
        javax.security.auth.login [] []
        javax.security.auth.spi [] []
        javax.security.auth.x500 [] []
        javax.security.cert [] []
        jdk.internal [jdk.incubator.vector] []
        jdk.internal.access [java.desktop, java.logging, java.management, java.rmi, jdk.charsets, jdk.crypto.cryptoki, jdk.jartool, jdk.jfr, jdk.jlink, jdk.management, jdk.net, jdk.sctp] []
        jdk.internal.classfile.components [jdk.jfr] []
        jdk.internal.event [jdk.jfr] []
        jdk.internal.foreign [jdk.incubator.vector] []
        jdk.internal.io [jdk.internal.le, jdk.jshell] []
        jdk.internal.javac [java.compiler, java.desktop, jdk.compiler, jdk.incubator.vector, jdk.jartool, jdk.jdeps, jdk.jfr, jdk.jlink, jdk.jshell] []
        jdk.internal.jimage [jdk.jlink] []
        jdk.internal.jimage.decompressor [jdk.jlink] []
        jdk.internal.jmod [jdk.compiler, jdk.jlink] []
        jdk.internal.loader [java.instrument, java.logging, java.naming] []
        jdk.internal.logger [java.logging] []
        jdk.internal.misc [java.desktop, java.logging, java.management, java.naming, java.net.http, java.rmi, java.security.jgss, jdk.attach, jdk.charsets, jdk.compiler, jdk.crypto.cryptoki, jdk.graal.compiler, jdk.incubator.vector, jdk.internal.vm.ci, jdk.jfr, jdk.jshell, jdk.nio.mapmode, jdk.unsupported] []
        jdk.internal.module [java.instrument, java.management.rmi, jdk.compiler, jdk.jartool, jdk.jfr, jdk.jlink, jdk.jpackage] []
        jdk.internal.org.xml.sax [jdk.jfr] []
        jdk.internal.org.xml.sax.helpers [jdk.jfr] []
        jdk.internal.perf [java.management, jdk.internal.jvmstat, jdk.management.agent] []
        jdk.internal.platform [jdk.jfr, jdk.management] []
        jdk.internal.ref [java.desktop, java.net.http, jdk.naming.dns] []
        jdk.internal.reflect [java.logging, java.sql, java.sql.rowset, jdk.dynalink, jdk.internal.vm.ci, jdk.unsupported] []
        jdk.internal.util [java.desktop, java.naming, java.net.http, java.prefs, java.rmi, java.security.jgss, java.smartcardio, jdk.charsets, jdk.httpserver, jdk.incubator.vector, jdk.internal.vm.ci, jdk.jlink, jdk.jpackage, jdk.net] []
        jdk.internal.util.xml [jdk.jfr] []
        jdk.internal.util.xml.impl [jdk.jfr] []
        jdk.internal.vm [java.management, jdk.internal.jvmstat, jdk.internal.vm.ci, jdk.jfr, jdk.management, jdk.management.agent] []
        jdk.internal.vm.annotation [java.instrument, jdk.incubator.vector, jdk.internal.vm.ci, jdk.jfr, jdk.unsupported] []
        jdk.internal.vm.vector [jdk.incubator.vector] []
        sun.invoke.util [jdk.compiler] []
        sun.net [java.net.http, jdk.naming.dns] []
        sun.net.dns [java.security.jgss, jdk.naming.dns] []
        sun.net.ext [jdk.net] []
        sun.net.util [java.net.http, jdk.jconsole, jdk.sctp] []
        sun.net.www [java.net.http, jdk.jartool] []
        sun.net.www.protocol.http [java.security.jgss] []
        sun.nio.ch [java.management, jdk.crypto.cryptoki, jdk.net, jdk.sctp] []
        sun.nio.fs [jdk.net] []
        sun.reflect.annotation [jdk.compiler] []
        sun.reflect.generics.reflectiveObjects [java.desktop] []
        sun.reflect.misc [java.desktop, java.management] []
        sun.security.internal.interfaces [jdk.crypto.cryptoki] []
        sun.security.jca [java.smartcardio, jdk.crypto.cryptoki, jdk.naming.dns] []
        sun.security.pkcs [jdk.jartool] []
        sun.security.provider [java.security.jgss, jdk.crypto.cryptoki, jdk.security.auth] []
        sun.security.provider.certpath [java.naming, jdk.jartool] []
        sun.security.timestamp [jdk.jartool] []
        sun.security.tools [jdk.jartool] []
        sun.security.validator [jdk.jartool] []
        sun.security.x509 [jdk.crypto.cryptoki, jdk.jartool] []
        sun.util.cldr [jdk.jlink] []
        sun.util.locale.provider [java.desktop, jdk.jlink, jdk.localedata] []
        sun.util.logging [java.desktop, java.logging, java.prefs] []
        sun.util.resources [jdk.localedata] []
      opens: []
      uses: [java.lang.System$LoggerFinder, java.net.ContentHandlerFactory, java.net.spi.InetAddressResolverProvider, java.net.spi.URLStreamHandlerProvider, java.nio.channels.spi.AsynchronousChannelProvider, java.nio.channels.spi.SelectorProvider, java.nio.charset.spi.CharsetProvider, java.nio.file.spi.FileSystemProvider, java.nio.file.spi.FileTypeDetector, java.security.Provider, java.text.spi.BreakIteratorProvider, java.text.spi.CollatorProvider, java.text.spi.DateFormatProvider, java.text.spi.DateFormatSymbolsProvider, java.text.spi.DecimalFormatSymbolsProvider, java.text.spi.NumberFormatProvider, java.time.chrono.AbstractChronology, java.time.chrono.Chronology, java.time.zone.ZoneRulesProvider, java.util.spi.CalendarDataProvider, java.util.spi.CalendarNameProvider, java.util.spi.CurrencyNameProvider, java.util.spi.LocaleNameProvider, java.util.spi.ResourceBundleControlProvider, java.util.spi.ResourceBundleProvider, java.util.spi.TimeZoneNameProvider, java.util.spi.ToolProvider, javax.security.auth.spi.LoginModule, jdk.internal.io.JdkConsoleProvider, jdk.internal.logger.DefaultLoggerFinder, sun.text.spi.JavaTimeDateTimePatternProvider, sun.util.locale.provider.LocaleDataMetaInfo, sun.util.resources.LocaleData$CommonResourceBundleProvider, sun.util.resources.LocaleData$SupplementaryResourceBundleProvider, sun.util.spi.CalendarProvider]
      provides: [java.nio.file.spi.FileSystemProvider with [jdk.internal.jrtfs.JrtFileSystemProvider]]

Class: interface org.xml.sax.XMLReader
  Module name: java.xml
    ModuleDescriptor:
      name: java.xml
      modifiers: []
      requires: [mandated java.base]
      exports:
        com.sun.org.apache.xml.internal.dtm [java.xml.crypto] []
        com.sun.org.apache.xml.internal.utils [java.xml.crypto] []
        com.sun.org.apache.xpath.internal [java.xml.crypto] []
        com.sun.org.apache.xpath.internal.compiler [java.xml.crypto] []
        com.sun.org.apache.xpath.internal.functions [java.xml.crypto] []
        com.sun.org.apache.xpath.internal.objects [java.xml.crypto] []
        com.sun.org.apache.xpath.internal.res [java.xml.crypto] []
        javax.xml [] []
        javax.xml.catalog [] []
        javax.xml.datatype [] []
        javax.xml.namespace [] []
        javax.xml.parsers [] []
        javax.xml.stream [] []
        javax.xml.stream.events [] []
        javax.xml.stream.util [] []
        javax.xml.transform [] []
        javax.xml.transform.dom [] []
        javax.xml.transform.sax [] []
        javax.xml.transform.stax [] []
        javax.xml.transform.stream [] []
        javax.xml.validation [] []
        javax.xml.xpath [] []
        org.w3c.dom [] []
        org.w3c.dom.bootstrap [] []
        org.w3c.dom.events [] []
        org.w3c.dom.ls [] []
        org.w3c.dom.ranges [] []
        org.w3c.dom.traversal [] []
        org.w3c.dom.views [] []
        org.xml.sax [] []
        org.xml.sax.ext [] []
        org.xml.sax.helpers [] []
      opens: []
      uses: [javax.xml.datatype.DatatypeFactory, javax.xml.parsers.DocumentBuilderFactory, javax.xml.parsers.SAXParserFactory, javax.xml.stream.XMLEventFactory, javax.xml.stream.XMLInputFactory, javax.xml.stream.XMLOutputFactory, javax.xml.transform.TransformerFactory, javax.xml.validation.SchemaFactory, javax.xml.xpath.XPathFactory, org.xml.sax.XMLReader]
      provides: []

Class: class samples.module.inspectmodule.InspectModule
  Module name: unnamed module
No ModuleDescriptor

"#,
    );
}

#[test]
fn should_support_locales() {
    assert_success(
        "samples.locale.localemonthsexample.LocaleMonthsExample",
        "[січня, лютого, березня, квітня, травня, червня, липня, серпня, вересня, жовтня, листопада, грудня, ]\n",
    );
}

#[test]
fn should_support_random_access_files() {
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let expected_out = r#"=== RandomAccessFile example ===
intValue = 0xDEADBEEF
longValue = 0xFEEDFACE01020304
doubleValue = 3.141592653589793
stringValue = HelloMMap

=== Memory-mapped file example ===
intValue = 0xDEADBEEF
longValue = 0xFEEDFACE01020304
doubleValue = 3.141592653589793
stringValue = HelloMMap

=== Verify after mmap modification ===
intValue = 0xCAFEBABE
longValue = 0xC0DEBA5EDEADBEEF
doubleValue = 2.718281828459045
stringValue = Water boils at 100 °C

=== Hex Dump ===
0000: 00 00 CA FE BA BE 00 00 C0 DE BA 5E DE AD BE EF  ...........^....
0010: 00 00 40 05 BF 0A 8B 14 57 69 00 00 00 00 00 16  ..@.....Wi......
0020: 57 61 74 65 72 20 62 6F 69 6C 73 20 61 74 20 31  Water boils at 1
0030: 30 30 20 C2 B0 43 00 00 00 00 00 00 00 00 00 00  00 ..C..........
"#;
    assert_success_with_args(
        "samples.io.randomaccessfilevsmmapexample.RandomAccessFileVsMMapExample",
        &[&file.path().display().to_string()],
        expected_out,
    );
}

#[test]
fn should_support_arrays_to_string() {
    assert_success(
        "samples.arrays.tostringexample.ArrayToStringExample",
        r#"[-128, -1, 0, 1, 127]
[true, false, true, true, false]
[-17000, -2000, 0, 2000, 17000]
[A, r, r, a, y, Ї, ⅒]
[-2000000000, -1, 0, 1, 2000000000]
[-9000000000000000000, -1, 0, 1, 9000000000000000000]
[3.14, 2.71, 1.41, 1.73]
[3.141592653589793, 2.71, 1.41, 1.73]
[Hello, Array, to, String, Example]
"#,
    );
}

#[test]
fn should_support_locales_advanced() {
    assert_success(
        "samples.locale.showlocale.ShowLocale",
        r#"--- Locale: ja_JP ---
Language:           'ja'
Country:            'JP'
Variant:            ''
Script:             ''
Display Language:   'Japanese'
Display Country:    'Japan'
Display Name:       'Japanese (Japan)'
Display Variant:    ''
Display Script:     ''
ISO3 Language:      'jpn'
ISO3 Country:       'JPN'
Number Format:   1,234,567.89
Currency Format: ￥1,234,568
Date Format:     1991年8月24日土曜日

--- Locale: uk_UA ---
Language:           'uk'
Country:            'UA'
Variant:            ''
Script:             ''
Display Language:   'Ukrainian'
Display Country:    'Ukraine'
Display Name:       'Ukrainian (Ukraine)'
Display Variant:    ''
Display Script:     ''
ISO3 Language:      'ukr'
ISO3 Country:       'UKR'
Number Format:   1 234 567,89
Currency Format: 1 234 567,89 ₴
Date Format:     субота, 24 серпня 1991 р.

--- Locale: es_MX ---
Language:           'es'
Country:            'MX'
Variant:            ''
Script:             ''
Display Language:   'Spanish'
Display Country:    'Mexico'
Display Name:       'Spanish (Mexico)'
Display Variant:    ''
Display Script:     ''
ISO3 Language:      'spa'
ISO3 Country:       'MEX'
Number Format:   1,234,567.89
Currency Format: $1,234,567.89
Date Format:     sábado, 24 de agosto de 1991

"#,
    );
}

#[test]
fn should_print_error_and_exit_if_no_main_method() {
    assert_failure(
        "samples.nomainmethod.NoMainMethod",
        r#"Error: Main method not found in class samples.nomainmethod.NoMainMethod, please define the main method as:
   public static void main(String[] args)
or a JavaFX application class must extend javafx.application.Application
"#,
    );
}

#[test]
fn should_delete_file_on_exit() {
    let temp_dir_path = env::temp_dir();
    let file_name = format!("delete_on_exit_{}.txt", rand::rng().random::<u64>());
    let file_path = Path::new(&temp_dir_path).join(file_name);
    let expected_content = "tru-la-la-la!";
    {
        let mut file =
            File::create_new(&file_path).expect("Failed to create file for DeleteOnExitDemo test");
        write!(file, "{expected_content}")
            .expect("Failed to write to file for DeleteOnExitDemo test");
    }

    assert_success_with_args(
        "samples.io.file.deleteonexitdemo.DeleteOnExitDemo",
        &[file_path.to_str().unwrap()],
        &format!("{expected_content}"),
    );

    assert!(
        !file_path.exists(),
        "File {} was not deleted on exit",
        file_path.display()
    );
}

#[rstest]
#[case::success(0, ExecutionResult::Success, 0)]
#[case::failure(1, ExecutionResult::Failure, 1)]
#[case::failure(2, ExecutionResult::Failure, 2)]
#[case::failure(3, ExecutionResult::Failure, 3)]
#[case::failure(1000, ExecutionResult::Failure, 232)]
#[case::failure(-2, ExecutionResult::Failure, 254)]
fn should_exit_with_given_code(
    #[case] given_exit_code: i32,
    #[case] result: ExecutionResult,
    #[case] expected_exit_code: i32,
) {
    utils::assert_with_all_args(
        &[],
        "samples.shutdown.exitwithcodedemo.ExitWithCodeDemo",
        &[&given_exit_code.to_string()],
        &format!("Exiting with code: {given_exit_code}"),
        "",
        result,
        expected_exit_code,
    );
}
