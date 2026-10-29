use jni::sys::{jboolean, jclass, jint, jobject, jstring, jvalue, JNIEnv};
use std::ptr::null_mut;

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_reflection_JniReflectedMethodsDemo_roundTrip(
    env: *mut JNIEnv,
    _class: jclass,
    reflected: jobject,
    declaring_class: jclass,
    is_static: jboolean,
) -> jobject {
    unsafe {
        let method_id = ((*(*env)).v24.FromReflectedMethod)(env, reflected);
        ((*(*env)).v24.ToReflectedMethod)(env, declaring_class, method_id, is_static)
    }
}

/// Checks that `FromReflectedMethod` yields the same ID as a `GetMethodID`/`GetStaticMethodID` lookup.
#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_reflection_JniReflectedMethodsDemo_sameIdAsLookup(
    env: *mut JNIEnv,
    _class: jclass,
    reflected: jobject,
    lookup_class: jclass,
    method_name_ref: jstring,
    signature_ref: jstring,
    is_static: jboolean,
) -> jboolean {
    unsafe {
        let method_name = ((*(*env)).v24.GetStringUTFChars)(env, method_name_ref, null_mut());
        let signature = ((*(*env)).v24.GetStringUTFChars)(env, signature_ref, null_mut());
        let looked_up = if is_static {
            ((*(*env)).v24.GetStaticMethodID)(env, lookup_class, method_name, signature)
        } else {
            ((*(*env)).v24.GetMethodID)(env, lookup_class, method_name, signature)
        };
        ((*(*env)).v24.ReleaseStringUTFChars)(env, signature_ref, signature);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, method_name_ref, method_name);

        let from_reflected = ((*(*env)).v24.FromReflectedMethod)(env, reflected);
        !from_reflected.is_null() && from_reflected == looked_up
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_reflection_JniReflectedMethodsDemo_invokeInstance(
    env: *mut JNIEnv,
    _class: jclass,
    reflected: jobject,
    target: jobject,
) -> jint {
    unsafe {
        let method_id = ((*(*env)).v24.FromReflectedMethod)(env, reflected);
        let args: Vec<jvalue> = vec![];
        ((*(*env)).v24.CallIntMethodA)(env, target, method_id, args.as_ptr())
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_reflection_JniReflectedMethodsDemo_invokeStatic(
    env: *mut JNIEnv,
    _class: jclass,
    reflected: jobject,
    target_class: jclass,
) -> jint {
    unsafe {
        let method_id = ((*(*env)).v24.FromReflectedMethod)(env, reflected);
        let args: Vec<jvalue> = vec![];
        ((*(*env)).v24.CallStaticIntMethodA)(env, target_class, method_id, args.as_ptr())
    }
}
