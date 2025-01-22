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
    assert_success("samples.reflection.trivial.ArrayClass",
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

"#);
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
    assert_success("samples.javacore.cloneable.trivial.CloneExample",
    r#"cloneable and anotherCloneable have different references but the same content
cloneable is not affected by changes in anotherCloneable
intArray and anotherIntArray have different references but the same content
intArray is not affected by changes in anotherIntArray
objArray and anotherObjArray have different references but the same content
objArray is not affected by changes in anotherObjArray
intMatrix and anotherIntMatrix have different references but the same content
intMatrix is affected by changes in anotherIntMatrix
"#);
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
    assert_success("samples.javacore.strings.trivial.ToStringAndBack",
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
"#);
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
fn should_do_operations_with_doubles() {
    assert_success(
        "samples.arithmetics.operations.doubles.DoubleOperations",
        r#"1.7E308
1.7E308
0.0
8.547008547008547E277
1.338773289334918E30
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
        r#"20
16.0
31.0
10001000000
Updated wide test
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

"#,
    );
}

#[test]
fn should_perform_integral_calculations_with_overflow() {
    assert_success("samples.arithmetics.overflow.ArithmeticIntegralOverflow",
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

"#);
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

#[test]
fn should_check_if_class_is_enum() {
    assert_success(
        "samples.reflection.trivial.isenumexample.IsEnumExample",
        r#"Is TimeUnit enum: true
Is TimeUnit.MINUTES enum: true
Is String enum: false
Is TimeUnit[].class enum: false
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
