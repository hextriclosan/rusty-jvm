use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::klass;
use crate::vm::jni::jni_value::JNIValue;
use crate::vm::jni::utils::{
    decode_method_id, get_method_id_impl, transform_args_from_va_list, transform_args_to_vec,
};
use crate::vm::method_area::method_area::with_method_area;
use jni_sys::{
    jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jmethodID, jobject, jshort,
    jvalue, va_list, JNIEnv,
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
    let raw = invoke_method(this as i32, method_id as usize, args);
    JNIValue::from_vec(&raw)
}

macro_rules! call_method_v_impl {
    ($name:ident, $jni_ty:ty) => {
        pub(super) unsafe extern "system" fn $name(
            env: *mut JNIEnv,
            this: jobject,
            method_id: jmethodID,
            args: va_list,
        ) -> $jni_ty {
            call_method_v::<$jni_ty>(env, this, method_id, args)
        }
    };
}
call_method_v_impl!(call_object_method_v, jobject);
call_method_v_impl!(call_boolean_method_v, jboolean);
call_method_v_impl!(call_byte_method_v, jbyte);
call_method_v_impl!(call_char_method_v, jchar);
call_method_v_impl!(call_short_method_v, jshort);
call_method_v_impl!(call_int_method_v, jint);
call_method_v_impl!(call_long_method_v, jlong);
call_method_v_impl!(call_float_method_v, jfloat);
call_method_v_impl!(call_double_method_v, jdouble);
call_method_v_impl!(call_void_method_v, ());
unsafe fn call_method_v<T: JNIValue>(
    _env: *mut JNIEnv,
    this: jobject,
    method_id: jmethodID,
    va: va_list,
) -> T {
    let (declaring_class_ref, method_index) = decode_method_id(method_id as usize);
    let declaring_klass =
        klass(declaring_class_ref).expect("Failed to get declaring class from jmethodID");
    let method = declaring_klass
        .get_method_by_index(method_index)
        .expect("Failed to get method from declaring class by index");
    let args_vec = transform_args_from_va_list(&method, va);
    let raw = invoke_method(this as i32, method_id as usize, args_vec.as_ptr());
    JNIValue::from_vec(&raw)
}

// Exported symbols used by the C shims in call_method_shims.c so that the
// variadic Call<type>Method entry points can delegate to the V variants.
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_object_method_v(
    env: *mut JNIEnv,
    this: jobject,
    method_id: jmethodID,
    va: va_list,
) -> jobject {
    call_method_v::<jobject>(env, this, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_boolean_method_v(
    env: *mut JNIEnv,
    this: jobject,
    method_id: jmethodID,
    va: va_list,
) -> jboolean {
    call_method_v::<jboolean>(env, this, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_byte_method_v(
    env: *mut JNIEnv,
    this: jobject,
    method_id: jmethodID,
    va: va_list,
) -> jbyte {
    call_method_v::<jbyte>(env, this, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_char_method_v(
    env: *mut JNIEnv,
    this: jobject,
    method_id: jmethodID,
    va: va_list,
) -> jchar {
    call_method_v::<jchar>(env, this, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_short_method_v(
    env: *mut JNIEnv,
    this: jobject,
    method_id: jmethodID,
    va: va_list,
) -> jshort {
    call_method_v::<jshort>(env, this, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_int_method_v(
    env: *mut JNIEnv,
    this: jobject,
    method_id: jmethodID,
    va: va_list,
) -> jint {
    call_method_v::<jint>(env, this, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_long_method_v(
    env: *mut JNIEnv,
    this: jobject,
    method_id: jmethodID,
    va: va_list,
) -> jlong {
    call_method_v::<jlong>(env, this, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_float_method_v(
    env: *mut JNIEnv,
    this: jobject,
    method_id: jmethodID,
    va: va_list,
) -> jfloat {
    call_method_v::<jfloat>(env, this, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_double_method_v(
    env: *mut JNIEnv,
    this: jobject,
    method_id: jmethodID,
    va: va_list,
) -> jdouble {
    call_method_v::<jdouble>(env, this, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_void_method_v(
    env: *mut JNIEnv,
    this: jobject,
    method_id: jmethodID,
    va: va_list,
) {
    call_method_v::<()>(env, this, method_id, va)
}

fn invoke_method(this: i32, method_id: usize, args: *const jvalue) -> Vec<i32> {
    let (declaring_class_ref, method_index) = decode_method_id(method_id);

    let declaring_klass =
        klass(declaring_class_ref).expect("Failed to get declaring class from jmethodID");
    let method = declaring_klass
        .get_method_by_index(method_index)
        .expect("Failed to get method from declaring class by index");

    let name_signature = method.name_signature().to_owned();
    let args_values = transform_args_to_vec(&method, args);

    // Perform virtual dispatch: find the concrete implementation in the actual instance's
    // class hierarchy (handles interface methods, abstract methods, and overridden methods).
    let instance_name = HEAP
        .get_instance_name(this)
        .expect("Failed to get instance name from reference");
    let implementation = with_method_area(|method_area| {
        method_area
            .lookup_for_implementation(&instance_name, &name_signature)
            .or_else(|| {
                method_area.lookup_for_implementation_interface(&instance_name, &name_signature)
            })
    })
    .unwrap_or_else(|| {
        panic!("Failed to find implementation of {name_signature} for {instance_name}")
    });

    let implementation_klass_name = implementation.class_name().to_owned();

    Executor::invoke_non_static_method(
        &implementation_klass_name,
        &name_signature,
        this,
        &args_values,
    )
    .expect("Failed to invoke non-static method")
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
    let raw = invoke_non_virtual_method(this as i32, target as i32, method_id as usize, args);
    JNIValue::from_vec(&raw)
}

macro_rules! call_non_virtual_method_v_impl {
    ($name:ident, $jni_ty:ty) => {
        pub(super) unsafe extern "system" fn $name(
            env: *mut JNIEnv,
            this: jobject,
            target: jclass,
            method_id: jmethodID,
            args: va_list,
        ) -> $jni_ty {
            call_non_virtual_method_v::<$jni_ty>(env, this, target, method_id, args)
        }
    };
}
call_non_virtual_method_v_impl!(call_non_virtual_object_method_v, jobject);
call_non_virtual_method_v_impl!(call_non_virtual_boolean_method_v, jboolean);
call_non_virtual_method_v_impl!(call_non_virtual_byte_method_v, jbyte);
call_non_virtual_method_v_impl!(call_non_virtual_char_method_v, jchar);
call_non_virtual_method_v_impl!(call_non_virtual_short_method_v, jshort);
call_non_virtual_method_v_impl!(call_non_virtual_int_method_v, jint);
call_non_virtual_method_v_impl!(call_non_virtual_long_method_v, jlong);
call_non_virtual_method_v_impl!(call_non_virtual_float_method_v, jfloat);
call_non_virtual_method_v_impl!(call_non_virtual_double_method_v, jdouble);
call_non_virtual_method_v_impl!(call_non_virtual_void_method_v, ());
unsafe fn call_non_virtual_method_v<T: JNIValue>(
    _env: *mut JNIEnv,
    this: jobject,
    target: jclass,
    method_id: jmethodID,
    va: va_list,
) -> T {
    let (declaring_class_ref, method_index) = decode_method_id(method_id as usize);
    let declaring_klass =
        klass(declaring_class_ref).expect("Failed to get declaring class from jmethodID");
    let method = declaring_klass
        .get_method_by_index(method_index)
        .expect("Failed to get method from declaring class by index");
    let args_vec = transform_args_from_va_list(&method, va);
    let raw =
        invoke_non_virtual_method(this as i32, target as i32, method_id as usize, args_vec.as_ptr());
    JNIValue::from_vec(&raw)
}

// Exported symbols used by the C shims for CallNonvirtual<type>Method.
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_non_virtual_object_method_v(
    env: *mut JNIEnv,
    this: jobject,
    target: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jobject {
    call_non_virtual_method_v::<jobject>(env, this, target, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_non_virtual_boolean_method_v(
    env: *mut JNIEnv,
    this: jobject,
    target: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jboolean {
    call_non_virtual_method_v::<jboolean>(env, this, target, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_non_virtual_byte_method_v(
    env: *mut JNIEnv,
    this: jobject,
    target: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jbyte {
    call_non_virtual_method_v::<jbyte>(env, this, target, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_non_virtual_char_method_v(
    env: *mut JNIEnv,
    this: jobject,
    target: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jchar {
    call_non_virtual_method_v::<jchar>(env, this, target, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_non_virtual_short_method_v(
    env: *mut JNIEnv,
    this: jobject,
    target: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jshort {
    call_non_virtual_method_v::<jshort>(env, this, target, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_non_virtual_int_method_v(
    env: *mut JNIEnv,
    this: jobject,
    target: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jint {
    call_non_virtual_method_v::<jint>(env, this, target, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_non_virtual_long_method_v(
    env: *mut JNIEnv,
    this: jobject,
    target: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jlong {
    call_non_virtual_method_v::<jlong>(env, this, target, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_non_virtual_float_method_v(
    env: *mut JNIEnv,
    this: jobject,
    target: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jfloat {
    call_non_virtual_method_v::<jfloat>(env, this, target, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_non_virtual_double_method_v(
    env: *mut JNIEnv,
    this: jobject,
    target: jclass,
    method_id: jmethodID,
    va: va_list,
) -> jdouble {
    call_non_virtual_method_v::<jdouble>(env, this, target, method_id, va)
}
#[no_mangle]
pub unsafe extern "C" fn rusty_jvm_call_non_virtual_void_method_v(
    env: *mut JNIEnv,
    this: jobject,
    target: jclass,
    method_id: jmethodID,
    va: va_list,
) {
    call_non_virtual_method_v::<()>(env, this, target, method_id, va)
}

fn invoke_non_virtual_method(
    this: i32,
    target: i32,
    method_id: usize,
    args: *const jvalue,
) -> Vec<i32> {
    let (declaring_class_ref, method_index) = decode_method_id(method_id);

    let target_klass_name = klass(target)
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

    Executor::invoke_non_static_method(&target_klass_name, &name_signature, this, &args_values)
        .expect("Failed to invoke non-static method") // todo: throw java.lang.AbstractMethodError
}
