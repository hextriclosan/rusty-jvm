use jni::sys::{jboolean, jclass, jint, jobject, jvalue, JNIEnv, JNI_FALSE, JNI_TRUE};
use std::ptr::null_mut;

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcRootsDemo_nativeOuter(
    env: *mut JNIEnv,
    class: jclass,
    value: jobject,
    marker: jint,
) -> jboolean {
    // SAFETY: This is a live JNIEnv and the arguments belong to the native invocation frame.
    unsafe {
        let jni = &(*(*env)).v24;
        let method = (jni.GetStaticMethodID)(
            env,
            class,
            c"nativeCallback".as_ptr(),
            c"(Ljava/lang/Object;I)V".as_ptr(),
        );
        if method.is_null() {
            return JNI_FALSE;
        }
        let args = [jvalue { l: value }, jvalue { i: marker }];
        (jni.CallStaticVoidMethodA)(env, class, method, args.as_ptr());
        if !(jni.ExceptionCheck)(env) {
            (jni.IsSameObject)(env, value, value)
        } else {
            JNI_FALSE
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcRootsDemo_nativeInner(
    env: *mut JNIEnv,
    class: jclass,
    value: jobject,
    marker: jint,
) -> jboolean {
    // SAFETY: This is the innermost live JNI frame in the nested callback proof.
    unsafe {
        let jni = &(*(*env)).v24;
        let method =
            (jni.GetStaticMethodID)(env, class, c"collectFromNative".as_ptr(), c"()V".as_ptr());
        if method.is_null() {
            return JNI_FALSE;
        }
        (jni.CallStaticVoidMethodA)(env, class, method, null_mut());
        if (jni.ExceptionCheck)(env) {
            return JNI_FALSE;
        }
        let valid = (jni.IsSameObject)(env, value, value) && marker == 31;
        if valid {
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    }
}
