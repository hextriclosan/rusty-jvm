use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{clazz_ref, klass};
use crate::vm::jni::jni_value::JNIValue;
use crate::vm::jni::utils::{get_method_id_impl, transform_args_to_vec};
use jni_sys::{
    jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jmethodID, jobject, jshort,
    jvalue, JNIEnv,
};
use std::ffi::c_char;

pub(super) extern "system" fn get_method_id(
    _env: *mut JNIEnv,
    clazz: jclass,
    name: *const c_char,
    sig: *const c_char,
) -> jmethodID {
    get_method_id_impl(clazz, name, sig)
}

macro_rules! call_method_a_impl {
    ($name:ident, $jni_ty:ty) => {
        pub(super) extern "system" fn $name(
            env: *mut JNIEnv,
            this: jobject,
            method_id: jmethodID,
            args: *const jvalue,
        ) -> $jni_ty {
            call_method_a::<$jni_ty>(env, this, method_id, args)
        }
    };
}
call_method_a_impl!(call_object_method_a, jobject);
call_method_a_impl!(call_boolean_method_a, jboolean);
call_method_a_impl!(call_byte_method_a, jbyte);
call_method_a_impl!(call_char_method_a, jchar);
call_method_a_impl!(call_short_method_a, jshort);
call_method_a_impl!(call_int_method_a, jint);
call_method_a_impl!(call_long_method_a, jlong);
call_method_a_impl!(call_float_method_a, jfloat);
call_method_a_impl!(call_double_method_a, jdouble);
call_method_a_impl!(call_void_method_a, ());
fn call_method_a<T: JNIValue>(
    _env: *mut JNIEnv,
    this: jobject,
    method_id: jmethodID,
    args: *const jvalue,
) -> T {
    let raw = invoke_method(this as i32, method_id as i64, args);
    JNIValue::from_vec(&raw)
}

fn invoke_method(this: i32, method_index: i64, args: *const jvalue) -> Vec<i32> {
    let instance_name = HEAP
        .get_instance_name(this)
        .expect("Failed to get instance name from reference");
    let clazz_ref =
        clazz_ref(&instance_name).expect("Failed to get class reference from instance name");
    let klass = klass(clazz_ref).expect("Failed to get class from reference");
    let method = klass
        .get_method_by_index(method_index)
        .expect("Failed to get method from class by index");
    let implementation_klass_name = method.class_name();
    let args_values = transform_args_to_vec(&method, args);

    Executor::invoke_non_static_method(
        implementation_klass_name,
        method.name_signature(),
        this,
        &args_values,
    )
    .expect("Failed to invoke non-static method")
}
