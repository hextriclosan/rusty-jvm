use crate::vm::execution_engine::executor::Executor;
use crate::vm::helper::klass;
use crate::vm::jni::jni_value::JNIValue;
use crate::vm::jni::utils::{get_method_id_impl, transform_args_to_vec};
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::java_method::JavaMethod;
use jni_sys::{
    jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jmethodID, jobject, jshort,
    jvalue, JNIEnv,
};
use std::ffi::c_char;
use std::sync::Arc;

pub(super) extern "system" fn get_static_method_id(
    _env: *mut JNIEnv,
    clazz: jclass,
    name: *const c_char,
    sig: *const c_char,
) -> jmethodID {
    get_method_id_impl(clazz, name, sig, true)
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
fn call_static_method_a<T: JNIValue>(
    _env: *mut JNIEnv,
    _cls: jclass,
    method_id: jmethodID,
    args: *const jvalue,
) -> T {
    // Decode the declaring class reference and method index from the encoded jmethodID.
    // High 32 bits: declaring class object reference
    // Low 32 bits:  method index within that class's methods map
    let declaring_class_ref = (method_id as i64 >> 32) as i32;
    let method_index = (method_id as i64) & 0xFFFF_FFFF;
    let klass = klass(declaring_class_ref).expect("Failed to get declaring class from jmethodID");
    let method = klass
        .get_method_by_index(method_index)
        .expect("Failed to get method by ID for static method call");

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
        .unwrap_or_else(|e| {
            panic!(
                "Failed to invoke static method {}.{} ({e})",
                klass.this_class_name(),
                method.name_signature()
            )
        })
}
