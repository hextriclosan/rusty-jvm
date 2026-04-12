use jni::sys::{
    jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jmethodID, jobject, jshort,
    jstring, jvalue, JNIEnv,
};
use std::ptr::{null, null_mut};

/// Native implementation of VirtualDispatchDemo.CallViaDeclaringClass.
///
/// Calls `GetMethodID` with the supplied `declaring_class` (which may be an interface,
/// abstract class, or parent class) and then invokes `CallObjectMethodA` on the concrete
/// `instance`.  This exercises virtual dispatch in `Call<type>MethodA`.
#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_VirtualDispatchDemo_CallViaDeclaringClass(
    env: *mut JNIEnv,
    _this: jobject,
    instance: jobject,
    declaring_class: jclass,
    method_name_ref: jstring,
    signature_ref: jstring,
) -> jobject {
    unsafe {
        let method_name = ((*(*env)).v24.GetStringUTFChars)(env, method_name_ref, null_mut());
        let signature = ((*(*env)).v24.GetStringUTFChars)(env, signature_ref, null_mut());
        let method_id = ((*(*env)).v24.GetMethodID)(env, declaring_class, method_name, signature);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, signature_ref, signature);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, method_name_ref, method_name);

        let args: Vec<jvalue> = vec![];
        ((*(*env)).v24.CallObjectMethodA)(env, instance, method_id, args.as_ptr())
    }
}

macro_rules! process_method_impl {
    ($name:ident, $jni_ty:ty, $call_fn:ident) => {
        #[no_mangle]
        pub extern "system" fn $name(
            env: *mut JNIEnv,
            this: jobject,
            method_name_ref: jstring,
            signature_ref: jstring,
            z: jboolean,
            b: jbyte,
            c: jchar,
            s: jshort,
            i: jint,
            j: jlong,
            f: jfloat,
            d: jdouble,
            l: jobject,
        ) -> $jni_ty {
            unsafe {
                process_method_call::<$jni_ty>(
                    env,
                    this,
                    method_name_ref,
                    signature_ref,
                    z,
                    b,
                    c,
                    s,
                    i,
                    j,
                    f,
                    d,
                    l,
                    (*(*env)).v24.$call_fn,
                )
            }
        }
    };
}
process_method_impl!(
    Java_samples_javacore_loadlibrary_example_InstanceMethodsDemo_InstanceObjectMethodDemo,
    jobject,
    CallObjectMethodA
);
process_method_impl!(
    Java_samples_javacore_loadlibrary_example_InstanceMethodsDemo_InstanceBooleanMethodDemo,
    jboolean,
    CallBooleanMethodA
);
process_method_impl!(
    Java_samples_javacore_loadlibrary_example_InstanceMethodsDemo_InstanceByteMethodDemo,
    jbyte,
    CallByteMethodA
);
process_method_impl!(
    Java_samples_javacore_loadlibrary_example_InstanceMethodsDemo_InstanceCharMethodDemo,
    jchar,
    CallCharMethodA
);
process_method_impl!(
    Java_samples_javacore_loadlibrary_example_InstanceMethodsDemo_InstanceShortMethodDemo,
    jshort,
    CallShortMethodA
);
process_method_impl!(
    Java_samples_javacore_loadlibrary_example_InstanceMethodsDemo_InstanceIntMethodDemo,
    jint,
    CallIntMethodA
);
process_method_impl!(
    Java_samples_javacore_loadlibrary_example_InstanceMethodsDemo_InstanceLongMethodDemo,
    jlong,
    CallLongMethodA
);
process_method_impl!(
    Java_samples_javacore_loadlibrary_example_InstanceMethodsDemo_InstanceFloatMethodDemo,
    jfloat,
    CallFloatMethodA
);
process_method_impl!(
    Java_samples_javacore_loadlibrary_example_InstanceMethodsDemo_InstanceDoubleMethodDemo,
    jdouble,
    CallDoubleMethodA
);
process_method_impl!(
    Java_samples_javacore_loadlibrary_example_InstanceMethodsDemo_InstanceVoidMethodDemo,
    (),
    CallVoidMethodA
);
unsafe fn process_method_call<T>(
    env: *mut JNIEnv,
    this: jobject,
    method_name_ref: jstring,
    signature_ref: jstring,
    z: jboolean,
    b: jbyte,
    c: jchar,
    s: jshort,
    i: jint,
    j: jlong,
    f: jfloat,
    d: jdouble,
    l: jobject,
    call_fn: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        method_id: jmethodID,
        args: *const jvalue,
    ) -> T,
) -> T {
    let method_name = ((*(*env)).v24.GetStringUTFChars)(env, method_name_ref, null_mut());
    let signature = ((*(*env)).v24.GetStringUTFChars)(env, signature_ref, null_mut());
    let class = ((*(*env)).v24.GetObjectClass)(env, this);
    let method_id = ((*(*env)).v24.GetMethodID)(env, class, method_name, signature);
    ((*(*env)).v24.ReleaseStringUTFChars)(env, signature_ref, signature);
    ((*(*env)).v24.ReleaseStringUTFChars)(env, method_name_ref, method_name);

    let args: Vec<jvalue> = vec![
        jvalue { z },
        jvalue { b },
        jvalue { c },
        jvalue { s },
        jvalue { i },
        jvalue { j },
        jvalue { f },
        jvalue { d },
        jvalue { l },
    ];
    call_fn(env, this, method_id, args.as_ptr())
}

/// Calls a zero-arg instance method via `CallObjectMethodV` with a null va_list.
///
/// Tests the V-variant dispatch path: since the method takes no arguments,
/// the va_list is never dereferenced and null is safe.
#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_InstanceMethodsDemo_InstanceObjectMethodVDemo(
    env: *mut JNIEnv,
    this: jobject,
    method_name_ref: jstring,
    sig_ref: jstring,
) -> jobject {
    unsafe {
        let method_name = ((*(*env)).v24.GetStringUTFChars)(env, method_name_ref, null_mut());
        let sig = ((*(*env)).v24.GetStringUTFChars)(env, sig_ref, null_mut());
        let class = ((*(*env)).v24.GetObjectClass)(env, this);
        let method_id = ((*(*env)).v24.GetMethodID)(env, class, method_name, sig);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, sig_ref, sig);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, method_name_ref, method_name);
        // Null va_list is safe because the target method takes no arguments.
        ((*(*env)).v24.CallObjectMethodV)(env, this, method_id, null_mut())
    }
}

/// Calls an instance method via the variadic `CallObjectMethod` entry point.
///
/// Routes through the C shim → `CallObjectMethodV` → method dispatch,
/// exercising the full non-A / non-V variant path.
#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_InstanceMethodsDemo_InstanceObjectMethodNonVDemo(
    env: *mut JNIEnv,
    this: jobject,
    method_name_ref: jstring,
    sig_ref: jstring,
    z: jboolean,
    b: jbyte,
    c: jchar,
    s: jshort,
    i: jint,
    j: jlong,
    f: jfloat,
    d: jdouble,
    l: jobject,
) -> jobject {
    unsafe {
        let method_name = ((*(*env)).v24.GetStringUTFChars)(env, method_name_ref, null_mut());
        let sig = ((*(*env)).v24.GetStringUTFChars)(env, sig_ref, null_mut());
        let class = ((*(*env)).v24.GetObjectClass)(env, this);
        let method_id = ((*(*env)).v24.GetMethodID)(env, class, method_name, sig);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, sig_ref, sig);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, method_name_ref, method_name);
        // Rust applies C default-argument promotions for variadic extern "C"
        // calls: smaller integer types → i32/u32, f32 → f64.
        ((*(*env)).v24.CallObjectMethod)(
            env, this, method_id,
            z as i32,  // jboolean → i32
            b as i32,  // jbyte    → i32
            c as u32,  // jchar    → u32
            s as i32,  // jshort   → i32
            i,         // jint     (i32)
            j,         // jlong    (i64)
            f as f64,  // jfloat   → f64
            d,         // jdouble  (f64)
            l,         // jobject
        )
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_NonVirtualDispatchDemo_CallNonVirtualViaDeclaringClass(
    env: *mut JNIEnv,
    _this: jobject,
    instance: jobject,
    target: jclass,
    method_name_ref: jstring,
    signature_ref: jstring,
) -> jobject {
    unsafe {
        let method_name = ((*(*env)).v24.GetStringUTFChars)(env, method_name_ref, null_mut());
        let signature = ((*(*env)).v24.GetStringUTFChars)(env, signature_ref, null_mut());
        let method_id = ((*(*env)).v24.GetMethodID)(env, target, method_name, signature);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, signature_ref, signature);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, method_name_ref, method_name);

        ((*(*env)).v24.CallNonvirtualObjectMethodA)(env, instance, target, method_id, null())
    }
}

macro_rules! process_non_virtual_method_impl {
    ($name:ident, $jni_ty:ty, $call_fn:ident) => {
        #[no_mangle]
        pub extern "system" fn $name(
            env: *mut JNIEnv,
            _this: jobject,
            instance: jobject,
            target_class: jclass,
            method_name_ref: jstring,
            signature_ref: jstring,
            z: jboolean,
            b: jbyte,
            c: jchar,
            s: jshort,
            i: jint,
            j: jlong,
            f: jfloat,
            d: jdouble,
            l: jobject,
        ) -> $jni_ty {
            unsafe {
                process_non_virtual_method_call::<$jni_ty>(
                    env,
                    instance,
                    target_class,
                    method_name_ref,
                    signature_ref,
                    z,
                    b,
                    c,
                    s,
                    i,
                    j,
                    f,
                    d,
                    l,
                    (*(*env)).v24.$call_fn,
                )
            }
        }
    };
}
process_non_virtual_method_impl!(
    Java_samples_javacore_loadlibrary_example_NonVirtualDispatchDemo_NonVirtualObjectMethodDemo,
    jobject,
    CallNonvirtualObjectMethodA
);
process_non_virtual_method_impl!(
    Java_samples_javacore_loadlibrary_example_NonVirtualDispatchDemo_NonVirtualBooleanMethodDemo,
    jboolean,
    CallNonvirtualBooleanMethodA
);
process_non_virtual_method_impl!(
    Java_samples_javacore_loadlibrary_example_NonVirtualDispatchDemo_NonVirtualByteMethodDemo,
    jbyte,
    CallNonvirtualByteMethodA
);
process_non_virtual_method_impl!(
    Java_samples_javacore_loadlibrary_example_NonVirtualDispatchDemo_NonVirtualCharMethodDemo,
    jchar,
    CallNonvirtualCharMethodA
);
process_non_virtual_method_impl!(
    Java_samples_javacore_loadlibrary_example_NonVirtualDispatchDemo_NonVirtualShortMethodDemo,
    jshort,
    CallNonvirtualShortMethodA
);
process_non_virtual_method_impl!(
    Java_samples_javacore_loadlibrary_example_NonVirtualDispatchDemo_NonVirtualIntMethodDemo,
    jint,
    CallNonvirtualIntMethodA
);
process_non_virtual_method_impl!(
    Java_samples_javacore_loadlibrary_example_NonVirtualDispatchDemo_NonVirtualLongMethodDemo,
    jlong,
    CallNonvirtualLongMethodA
);
process_non_virtual_method_impl!(
    Java_samples_javacore_loadlibrary_example_NonVirtualDispatchDemo_NonVirtualFloatMethodDemo,
    jfloat,
    CallNonvirtualFloatMethodA
);
process_non_virtual_method_impl!(
    Java_samples_javacore_loadlibrary_example_NonVirtualDispatchDemo_NonVirtualDoubleMethodDemo,
    jdouble,
    CallNonvirtualDoubleMethodA
);
process_non_virtual_method_impl!(
    Java_samples_javacore_loadlibrary_example_NonVirtualDispatchDemo_NonVirtualVoidMethodDemo,
    (),
    CallNonvirtualVoidMethodA
);
unsafe fn process_non_virtual_method_call<T>(
    env: *mut JNIEnv,
    instance: jobject,
    target_class: jclass,
    method_name_ref: jstring,
    signature_ref: jstring,
    z: jboolean,
    b: jbyte,
    c: jchar,
    s: jshort,
    i: jint,
    j: jlong,
    f: jfloat,
    d: jdouble,
    l: jobject,
    call_fn: unsafe extern "system" fn(
        env: *mut JNIEnv,
        obj: jobject,
        clazz: jclass,
        method_id: jmethodID,
        args: *const jvalue,
    ) -> T,
) -> T {
    let method_name = ((*(*env)).v24.GetStringUTFChars)(env, method_name_ref, null_mut());
    let signature = ((*(*env)).v24.GetStringUTFChars)(env, signature_ref, null_mut());
    let method_id = ((*(*env)).v24.GetMethodID)(env, target_class, method_name, signature);
    ((*(*env)).v24.ReleaseStringUTFChars)(env, signature_ref, signature);
    ((*(*env)).v24.ReleaseStringUTFChars)(env, method_name_ref, method_name);

    let args: Vec<jvalue> = vec![
        jvalue { z },
        jvalue { b },
        jvalue { c },
        jvalue { s },
        jvalue { i },
        jvalue { j },
        jvalue { f },
        jvalue { d },
        jvalue { l },
    ];
    call_fn(env, instance, target_class, method_id, args.as_ptr())
}
