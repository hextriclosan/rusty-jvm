use crate::vm::execution_engine::executor::Executor;
use crate::vm::helper::klass;
use crate::vm::jni::jni_value::JNIValue;
use crate::vm::jni::utils::{
    decode_method_id, get_method_id_impl, transform_args_from_va_list, transform_args_to_vec,
};
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::java_method::JavaMethod;
use jni_sys::{
    jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jmethodID, jobject, jshort,
    jvalue, va_list, JNIEnv,
};
use std::ffi::c_char;
use std::sync::Arc;

pub(super) extern "system" fn get_static_method_id(
    _env: *mut JNIEnv,
    clazz: jclass,
    name: *const c_char,
    sig: *const c_char,
) -> jmethodID {
    get_method_id_impl(clazz, name, sig)
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
    let (declaring_class_ref, method_index) = decode_method_id(method_id as usize);

    let klass = klass(declaring_class_ref).expect("Failed to get class from reference");
    let method = klass
        .get_method_by_index(method_index)
        .expect("Failed to get method by ID for static void method call");

    let raw = invoke_static_method(&klass, &method, args);
    JNIValue::from_vec(&raw)
}

macro_rules! call_static_method_v_impl {
    ($name:ident, $jni_ty:ty) => {
        pub(super) unsafe extern "system" fn $name(
            env: *mut JNIEnv,
            clazz: jclass,
            method_id: jmethodID,
            args: va_list,
        ) -> $jni_ty {
            call_static_method_v::<$jni_ty>(env, clazz, method_id, args)
        }
    };
}
call_static_method_v_impl!(call_static_object_method_v, jobject);
call_static_method_v_impl!(call_static_boolean_method_v, jboolean);
call_static_method_v_impl!(call_static_byte_method_v, jbyte);
call_static_method_v_impl!(call_static_char_method_v, jchar);
call_static_method_v_impl!(call_static_short_method_v, jshort);
call_static_method_v_impl!(call_static_int_method_v, jint);
call_static_method_v_impl!(call_static_long_method_v, jlong);
call_static_method_v_impl!(call_static_float_method_v, jfloat);
call_static_method_v_impl!(call_static_double_method_v, jdouble);
call_static_method_v_impl!(call_static_void_method_v, ());
unsafe fn call_static_method_v<T: JNIValue>(
    _env: *mut JNIEnv,
    _cls: jclass,
    method_id: jmethodID,
    va: va_list,
) -> T {
    let (declaring_class_ref, method_index) = decode_method_id(method_id as usize);
    let klass = klass(declaring_class_ref).expect("Failed to get class from reference");
    let method = klass
        .get_method_by_index(method_index)
        .expect("Failed to get method by ID for static method V call");
    let args_vec = transform_args_from_va_list(&method, va);
    let raw = invoke_static_method(&klass, &method, args_vec.as_ptr());
    JNIValue::from_vec(&raw)
}

// Exported symbols used by the C shims for CallStatic<type>Method.
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_static_object_method_v(
    env: *mut JNIEnv,
    cls: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jobject {
    call_static_method_v::<jobject>(env, cls, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_static_boolean_method_v(
    env: *mut JNIEnv,
    cls: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jboolean {
    call_static_method_v::<jboolean>(env, cls, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_static_byte_method_v(
    env: *mut JNIEnv,
    cls: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jbyte {
    call_static_method_v::<jbyte>(env, cls, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_static_char_method_v(
    env: *mut JNIEnv,
    cls: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jchar {
    call_static_method_v::<jchar>(env, cls, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_static_short_method_v(
    env: *mut JNIEnv,
    cls: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jshort {
    call_static_method_v::<jshort>(env, cls, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_static_int_method_v(
    env: *mut JNIEnv,
    cls: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jint {
    call_static_method_v::<jint>(env, cls, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_static_long_method_v(
    env: *mut JNIEnv,
    cls: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jlong {
    call_static_method_v::<jlong>(env, cls, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_static_float_method_v(
    env: *mut JNIEnv,
    cls: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jfloat {
    call_static_method_v::<jfloat>(env, cls, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_static_double_method_v(
    env: *mut JNIEnv,
    cls: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jdouble {
    call_static_method_v::<jdouble>(env, cls, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_static_void_method_v(
    env: *mut JNIEnv,
    cls: jclass,
    method_id: jmethodID,
    va: va_list,
) {
    call_static_method_v::<()>(env, cls, method_id, va)
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
