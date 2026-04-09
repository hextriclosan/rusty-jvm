use crate::from_mutf8_ptr;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::helper::klass;
use crate::vm::jni::jni_value::JNIValue;
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::stack::stack_value::StackValueKind;
use jdescriptor::TypeDescriptor;
use jni_sys::{
    jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jmethodID, jobject, jshort,
    jvalue, JNIEnv,
};
use std::ffi::c_char;
use std::ptr::null_mut;
use std::sync::Arc;

pub(super) extern "system" fn get_static_method_id(
    _env: *mut JNIEnv,
    clazz: jclass,
    name: *const c_char,
    sig: *const c_char,
) -> jmethodID {
    let name_str = from_mutf8_ptr!(name).expect("Failed to convert method name from CESU-8");
    let sig_str = from_mutf8_ptr!(sig).expect("Failed to convert method signature from CESU-8");
    let klass = klass(clazz as i32).expect("Failed to get class from reference");
    StaticInit::initialize_java_class(&klass)
        .expect("Failed to initialize class before getting static method ID"); // todo: throw ExceptionInInitializerError here
    let full_signature = format!("{}:{}", name_str, sig_str);

    klass
        .get_method_full(&full_signature)
        .and_then(|(method_id, _method)| Some(method_id as jmethodID))
        .unwrap_or(null_mut()) // todo: throw NoSuchMethodError here
}

macro_rules! get_static_method_a_impl {
    ($name:ident, $jni_ty:ty) => {
        pub(super) extern "system" fn $name(
            env: *mut JNIEnv,
            clazz: jclass,
            method_id: jmethodID,
            args: *const jvalue,
        ) -> $jni_ty {
            call_static_method_a::<$jni_ty>(env, clazz, method_id, args)
        }
    };
}
get_static_method_a_impl!(call_static_object_method_a, jobject);
get_static_method_a_impl!(call_static_boolean_method_a, jboolean);
get_static_method_a_impl!(call_static_byte_method_a, jbyte);
get_static_method_a_impl!(call_static_char_method_a, jchar);
get_static_method_a_impl!(call_static_short_method_a, jshort);
get_static_method_a_impl!(call_static_int_method_a, jint);
get_static_method_a_impl!(call_static_long_method_a, jlong);
get_static_method_a_impl!(call_static_float_method_a, jfloat);
get_static_method_a_impl!(call_static_double_method_a, jdouble);
get_static_method_a_impl!(call_static_void_method_a, ());
pub(super) extern "system" fn call_static_method_a<T: JNIValue>(
    _env: *mut JNIEnv,
    cls: jclass,
    method_id: jmethodID,
    args: *const jvalue,
) -> T {
    let method_index = method_id as i64;
    let klass = klass(cls as i32).expect("Failed to get class from reference");
    let method = klass
        .get_method_by_index(method_index)
        .expect("Failed to get method by ID for static void method call");

    let raw = invoke_static_method(&klass, &method, args);
    JNIValue::from_vec(&raw)
}

fn invoke_static_method(
    klass: &Arc<JavaClass>,
    method: &Arc<JavaMethod>,
    args: *const jvalue,
) -> Vec<i32> {
    let args_values = transform_args_to_vec(&method, args);
    Executor::invoke_static_method_jc(&klass, method.name_signature(), &args_values)
        .expect("Failed to invoke static void method")
}

fn transform_args_to_vec(method: &Arc<JavaMethod>, args: *const jvalue) -> Vec<StackValueKind> {
    let md = method.get_method_descriptor();
    let pt = md.parameter_types();
    let args_count = pt.len();

    let args = unsafe { std::slice::from_raw_parts(args, args_count) };
    args.iter()
        .zip(pt.iter())
        .map(|(arg, pt)| resolve_stack_kind_value(*arg, pt))
        .collect::<Vec<_>>()
}

fn resolve_stack_kind_value(value: jvalue, type_descriptor: &TypeDescriptor) -> StackValueKind {
    match type_descriptor {
        TypeDescriptor::Boolean => (if unsafe { value.z } { 1 } else { 0 }).into(),
        TypeDescriptor::Byte => (unsafe { value.b } as i32).into(),
        TypeDescriptor::Char => (unsafe { value.c } as i32).into(),
        TypeDescriptor::Short => (unsafe { value.s } as i32).into(),
        TypeDescriptor::Integer => (unsafe { value.i } as i32).into(),
        TypeDescriptor::Long => (unsafe { value.j } as i64).into(),
        TypeDescriptor::Float => (unsafe { value.f } as f32).into(),
        TypeDescriptor::Double => (unsafe { value.d } as f64).into(),
        TypeDescriptor::Object(_) | TypeDescriptor::Array(_, _) => {
            (unsafe { value.l } as i32).into()
        }
        TypeDescriptor::Void => panic!("Void type cannot be used as argument"),
    }
}
