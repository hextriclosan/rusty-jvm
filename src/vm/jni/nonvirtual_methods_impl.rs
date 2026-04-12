use crate::vm::execution_engine::executor::Executor;
use crate::vm::helper::klass;
use crate::vm::jni::jni_value::JNIValue;
use crate::vm::jni::utils::{decode_method_id, transform_args_to_vec};
use jni_sys::{
    jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jmethodID, jobject, jshort,
    jvalue, va_list, JNIEnv,
};

macro_rules! call_nonvirtual_method_a_impl {
    ($name:ident, $jni_ty:ty) => {
        pub(super) extern "system" fn $name(
            env: *mut JNIEnv,
            this: jobject,
            clazz: jclass,
            method_id: jmethodID,
            args: *const jvalue,
        ) -> $jni_ty {
            call_nonvirtual_method_a::<$jni_ty>(env, this, clazz, method_id, args)
        }
    };
}
call_nonvirtual_method_a_impl!(call_nonvirtual_object_method_a, jobject);
call_nonvirtual_method_a_impl!(call_nonvirtual_boolean_method_a, jboolean);
call_nonvirtual_method_a_impl!(call_nonvirtual_byte_method_a, jbyte);
call_nonvirtual_method_a_impl!(call_nonvirtual_char_method_a, jchar);
call_nonvirtual_method_a_impl!(call_nonvirtual_short_method_a, jshort);
call_nonvirtual_method_a_impl!(call_nonvirtual_int_method_a, jint);
call_nonvirtual_method_a_impl!(call_nonvirtual_long_method_a, jlong);
call_nonvirtual_method_a_impl!(call_nonvirtual_float_method_a, jfloat);
call_nonvirtual_method_a_impl!(call_nonvirtual_double_method_a, jdouble);
call_nonvirtual_method_a_impl!(call_nonvirtual_void_method_a, ());

fn call_nonvirtual_method_a<T: JNIValue>(
    _env: *mut JNIEnv,
    this: jobject,
    _clazz: jclass,
    method_id: jmethodID,
    args: *const jvalue,
) -> T {
    let raw = invoke_nonvirtual_method(this as i32, method_id as usize, args);
    JNIValue::from_vec(&raw)
}

macro_rules! call_nonvirtual_method_v_impl {
    ($name:ident, $jni_ty:ty) => {
        pub(super) extern "system" fn $name(
            env: *mut JNIEnv,
            this: jobject,
            clazz: jclass,
            method_id: jmethodID,
            args: va_list,
        ) -> $jni_ty {
            call_nonvirtual_method_v::<$jni_ty>(env, this, clazz, method_id, args)
        }
    };
}
call_nonvirtual_method_v_impl!(call_nonvirtual_object_method_v, jobject);
call_nonvirtual_method_v_impl!(call_nonvirtual_boolean_method_v, jboolean);
call_nonvirtual_method_v_impl!(call_nonvirtual_byte_method_v, jbyte);
call_nonvirtual_method_v_impl!(call_nonvirtual_char_method_v, jchar);
call_nonvirtual_method_v_impl!(call_nonvirtual_short_method_v, jshort);
call_nonvirtual_method_v_impl!(call_nonvirtual_int_method_v, jint);
call_nonvirtual_method_v_impl!(call_nonvirtual_long_method_v, jlong);
call_nonvirtual_method_v_impl!(call_nonvirtual_float_method_v, jfloat);
call_nonvirtual_method_v_impl!(call_nonvirtual_double_method_v, jdouble);
call_nonvirtual_method_v_impl!(call_nonvirtual_void_method_v, ());

fn call_nonvirtual_method_v<T: JNIValue>(
    _env: *mut JNIEnv,
    this: jobject,
    _clazz: jclass,
    method_id: jmethodID,
    args: va_list,
) -> T {
    // va_list is defined as *mut c_void in jni_sys; treat it as a pointer to a
    // packed jvalue array, which is the convention used by most JNI callers that
    // forward variadic arguments via va_list.
    let args_as_jvalues = args as *const jvalue;
    let raw = invoke_nonvirtual_method(this as i32, method_id as usize, args_as_jvalues);
    JNIValue::from_vec(&raw)
}

fn invoke_nonvirtual_method(this: i32, method_id: usize, args: *const jvalue) -> Vec<i32> {
    let (declaring_class_ref, method_index) = decode_method_id(method_id);

    let declaring_klass =
        klass(declaring_class_ref).expect("Failed to get declaring class from jmethodID");
    let method = declaring_klass
        .get_method_by_index(method_index)
        .expect("Failed to get method from declaring class by index");

    let args_values = transform_args_to_vec(&method, args);

    // Non-virtual dispatch: invoke the method directly on the declaring class,
    // bypassing the vtable.  This is the correct behaviour for CallNonvirtualXxxMethodA/V.
    Executor::invoke_non_static_method_jc(&declaring_klass, method.name_signature(), this, &args_values)
        .unwrap_or_else(|e| {
            panic!(
                "Failed to invoke non-virtual method {}.{} ({e})",
                declaring_klass.this_class_name(),
                method.name_signature()
            )
        })
}
