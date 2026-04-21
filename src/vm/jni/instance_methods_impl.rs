use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::klass;
use crate::vm::jni::jni_invoke::jni_invoke;
use crate::vm::jni::jni_value::JNIValue;
use crate::vm::jni::utils::{decode_method_id, get_method_id_impl, transform_args_to_vec};
use crate::vm::method_area::lookup;
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
    let (declaring_class_ref, method_index) = decode_method_id(method_id as usize);

    let declaring_klass =
        klass(declaring_class_ref).expect("Failed to get declaring class from jmethodID");
    let method = declaring_klass
        .get_method_by_index(method_index)
        .expect("Failed to get method from declaring class by index");

    let name_signature = method.name_signature().to_owned();
    let args_values = transform_args_to_vec(&method, args);

    // Perform virtual dispatch: find the concrete implementation in the actual instance's
    // class hierarchy (handles interface methods, abstract methods, and overridden methods).
    let this_ref = this as i32;
    let instance_name = HEAP
        .get_instance_name(this_ref)
        .expect("Failed to get instance name from reference");
    let implementation = lookup::lookup_method(&instance_name, &name_signature)
        .unwrap_or_else(|e| {
            panic!("Failed to find implementation of {name_signature} for {instance_name}: {e}")
        })
        .unwrap_or_else(|| {
            panic!("Failed to find implementation of {name_signature} for {instance_name}")
        });

    let implementation_klass_name = implementation.class_name().to_owned();

    let context = format!("{implementation_klass_name}.{name_signature}");
    let result = Executor::invoke_non_static_method(
        &implementation_klass_name,
        &name_signature,
        this_ref,
        &args_values,
    );
    jni_invoke(result, &context)
}

macro_rules! call_non_virtual_method_a_impl {
    ($name:ident, $jni_ty:ty) => {
        pub(super) extern "system" fn $name(
            env: *mut JNIEnv,
            this: jobject,
            target: jclass,
            method_id: jmethodID,
            args: *const jvalue,
        ) -> $jni_ty {
            call_non_virtual_method_a::<$jni_ty>(env, this, target, method_id, args)
        }
    };
}
call_non_virtual_method_a_impl!(call_non_virtual_object_method_a, jobject);
call_non_virtual_method_a_impl!(call_non_virtual_boolean_method_a, jboolean);
call_non_virtual_method_a_impl!(call_non_virtual_byte_method_a, jbyte);
call_non_virtual_method_a_impl!(call_non_virtual_char_method_a, jchar);
call_non_virtual_method_a_impl!(call_non_virtual_short_method_a, jshort);
call_non_virtual_method_a_impl!(call_non_virtual_int_method_a, jint);
call_non_virtual_method_a_impl!(call_non_virtual_long_method_a, jlong);
call_non_virtual_method_a_impl!(call_non_virtual_float_method_a, jfloat);
call_non_virtual_method_a_impl!(call_non_virtual_double_method_a, jdouble);
call_non_virtual_method_a_impl!(call_non_virtual_void_method_a, ());
fn call_non_virtual_method_a<T: JNIValue>(
    _env: *mut JNIEnv,
    this: jobject,
    target: jclass,
    method_id: jmethodID,
    args: *const jvalue,
) -> T {
    let (declaring_class_ref, method_index) = decode_method_id(method_id as usize);

    let target_klass_name = klass(target as i32)
        .expect("Failed to get target class")
        .this_class_name()
        .to_owned();

    let declaring_klass =
        klass(declaring_class_ref).expect("Failed to get declaring class from jmethodID");
    let method = declaring_klass
        .get_method_by_index(method_index)
        .expect("Failed to get method from declaring class by index");
    let name_signature = method.name_signature().to_owned();
    let args_values = transform_args_to_vec(&method, args);

    let context = format!("{target_klass_name}.{name_signature}");
    let result = Executor::invoke_non_static_method(
        &target_klass_name,
        &name_signature,
        this as i32,
        &args_values,
    );
    jni_invoke(result, &context)
}
