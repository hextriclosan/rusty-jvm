use crate::vm::execution_engine::executor::Executor;
use crate::vm::helper::klass;
use crate::vm::jni::jni_invoke::jni_invoke;
use crate::vm::jni::jni_value::JNIValue;
use crate::vm::jni::utils::{decode_method_id, get_method_id_impl, transform_args_to_vec};
use jni_sys::{
    jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jmethodID, jobject, jshort,
    jvalue, JNIEnv,
};
use std::ffi::c_char;

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
        .expect("Failed to get method by ID for static method call");

    let args_values = transform_args_to_vec(&method, args);
    let context = format!("{}.{}", klass.this_class_name(), method.name_signature());
    let result = Executor::invoke_static_method_jc(&klass, method.name_signature(), &args_values);
    jni_invoke(result, &context)
}
