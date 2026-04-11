use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::klass;
use crate::vm::jni::jni_value::JNIValue;
use crate::vm::jni::utils::{get_method_id_impl, transform_args_to_vec};
use crate::vm::method_area::method_area::with_method_area;
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

fn invoke_method(this: i32, method_id: i64, args: *const jvalue) -> Vec<i32> {
    // Decode the declaring class reference and method index from the encoded jmethodID.
    // High 32 bits: declaring class object reference
    // Low 32 bits: method index within that class's methods map
    let declaring_class_ref = (method_id >> 32) as i32;
    let method_index = method_id & 0xFFFF_FFFF;

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
