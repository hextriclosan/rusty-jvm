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
