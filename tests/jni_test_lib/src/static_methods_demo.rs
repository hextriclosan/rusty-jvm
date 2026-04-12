use jni::sys::{
    jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jmethodID, jobject, jshort,
    jstring, jvalue, JNIEnv,
};

macro_rules! process_static_method_impl {
    ($name:ident, $jni_ty:ty, $call_fn:ident) => {
        #[no_mangle]
        pub extern "system" fn $name(
            env: *mut JNIEnv,
            class: jclass,
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
                process_static_method_call::<$jni_ty>(
                    env,
                    class,
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
process_static_method_impl!(
    Java_samples_javacore_loadlibrary_example_StaticMethodsDemo_StaticObjectMethodDemo,
    jobject,
    CallStaticObjectMethodA
);
process_static_method_impl!(
    Java_samples_javacore_loadlibrary_example_StaticMethodsDemo_StaticBooleanMethodDemo,
    jboolean,
    CallStaticBooleanMethodA
);
process_static_method_impl!(
    Java_samples_javacore_loadlibrary_example_StaticMethodsDemo_StaticByteMethodDemo,
    jbyte,
    CallStaticByteMethodA
);
process_static_method_impl!(
    Java_samples_javacore_loadlibrary_example_StaticMethodsDemo_StaticCharMethodDemo,
    jchar,
    CallStaticCharMethodA
);
process_static_method_impl!(
    Java_samples_javacore_loadlibrary_example_StaticMethodsDemo_StaticShortMethodDemo,
    jshort,
    CallStaticShortMethodA
);
process_static_method_impl!(
    Java_samples_javacore_loadlibrary_example_StaticMethodsDemo_StaticIntMethodDemo,
    jint,
    CallStaticIntMethodA
);
process_static_method_impl!(
    Java_samples_javacore_loadlibrary_example_StaticMethodsDemo_StaticLongMethodDemo,
    jlong,
    CallStaticLongMethodA
);
process_static_method_impl!(
    Java_samples_javacore_loadlibrary_example_StaticMethodsDemo_StaticFloatMethodDemo,
    jfloat,
    CallStaticFloatMethodA
);
process_static_method_impl!(
    Java_samples_javacore_loadlibrary_example_StaticMethodsDemo_StaticDoubleMethodDemo,
    jdouble,
    CallStaticDoubleMethodA
);
process_static_method_impl!(
    Java_samples_javacore_loadlibrary_example_StaticMethodsDemo_StaticVoidMethodDemo,
    (),
    CallStaticVoidMethodA
);
unsafe fn process_static_method_call<T>(
    env: *mut JNIEnv,
    class: jclass,
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
    call_fn: unsafe extern "system" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> T,
) -> T {
    let method_name =
        ((*(*env)).v24.GetStringUTFChars)(env, method_name_ref, std::ptr::null_mut());
    let signature = ((*(*env)).v24.GetStringUTFChars)(env, signature_ref, std::ptr::null_mut());
    let method_id = ((*(*env)).v24.GetStaticMethodID)(env, class, method_name, signature);
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
    call_fn(env, class, method_id, args.as_ptr())
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_StaticMethodsDemo_BaseStaticMethodDemo(
    env: *mut JNIEnv,
    _class: jclass,
    target: jclass,
    method_name_ref: jstring,
    signature_ref: jstring,
) {
    unsafe {
        let method_name =
            ((*(*env)).v24.GetStringUTFChars)(env, method_name_ref, std::ptr::null_mut());
        let signature =
            ((*(*env)).v24.GetStringUTFChars)(env, signature_ref, std::ptr::null_mut());
        let method_id = ((*(*env)).v24.GetStaticMethodID)(env, target, method_name, signature);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, signature_ref, signature);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, method_name_ref, method_name);

        ((*(*env)).v24.CallStaticVoidMethodA)(env, target, method_id, std::ptr::null());
    }
}

/// Calls a static zero-arg method via `CallStaticObjectMethodV` with a null va_list.
///
/// This exercises the V-variant dispatch path end-to-end: the JNI vtable
/// entry for `CallStaticObjectMethodV` is our Rust implementation which reads
/// parameter types, and since the method takes no arguments the va_list is
/// never dereferenced (null is safe).
#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_StaticMethodsDemo_StaticObjectMethodVDemo(
    env: *mut JNIEnv,
    class: jclass,
    method_name_ref: jstring,
    sig_ref: jstring,
) -> jobject {
    unsafe {
        let method_name =
            ((*(*env)).v24.GetStringUTFChars)(env, method_name_ref, std::ptr::null_mut());
        let sig = ((*(*env)).v24.GetStringUTFChars)(env, sig_ref, std::ptr::null_mut());
        let method_id = ((*(*env)).v24.GetStaticMethodID)(env, class, method_name, sig);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, sig_ref, sig);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, method_name_ref, method_name);
        // Null va_list is safe here because the target method takes no arguments.
        ((*(*env)).v24.CallStaticObjectMethodV)(env, class, method_id, std::ptr::null_mut())
    }
}

/// Calls a static method via the variadic `CallStaticObjectMethod` entry point.
///
/// The variadic entry point routes through a C shim that calls `va_start` and
/// delegates to `CallStaticObjectMethodV`, exercising the full
/// non-A / non-V variant path.
#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_StaticMethodsDemo_StaticObjectMethodNonVDemo(
    env: *mut JNIEnv,
    class: jclass,
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
        let method_name =
            ((*(*env)).v24.GetStringUTFChars)(env, method_name_ref, std::ptr::null_mut());
        let sig = ((*(*env)).v24.GetStringUTFChars)(env, sig_ref, std::ptr::null_mut());
        let method_id = ((*(*env)).v24.GetStaticMethodID)(env, class, method_name, sig);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, sig_ref, sig);
        ((*(*env)).v24.ReleaseStringUTFChars)(env, method_name_ref, method_name);
        // Rust applies C default-argument promotions when calling a variadic
        // extern "C" function, so smaller integer types become i32/u32 and
        // f32 becomes f64 – exactly as the x86-64 System V ABI requires.
        ((*(*env)).v24.CallStaticObjectMethod)(
            env, class, method_id,
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
