use jni::sys::{jclass, jint, jobject, jstring, jthrowable, jvalue, JNIEnv};
use std::ffi::{CStr, CString};
use std::ptr::null_mut;

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ExceptionDemo_Throw(
    env: *mut JNIEnv,
    _class: jclass,
    throwable: jthrowable,
) -> jint {
    unsafe { ((*(*env)).v24.Throw)(env, throwable) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ExceptionDemo_ThrowNew(
    env: *mut JNIEnv,
    _class: jclass,
    clazz: jclass,
    message: jstring,
) -> jint {
    unsafe {
        if message.is_null() {
            ((*(*env)).v24.ThrowNew)(env, clazz, std::ptr::null())
        } else {
            let raw = ((*(*env)).v24.GetStringUTFChars)(env, message, null_mut());
            let res = ((*(*env)).v24.ThrowNew)(env, clazz, raw);
            ((*(*env)).v24.ReleaseStringUTFChars)(env, message, raw);
            res
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ExceptionDemo_CheckOccurredClearDemo(
    env: *mut JNIEnv,
    class: jclass,
    method_name_ref: jstring,
    signature_ref: jstring,
) -> jstring {
    unsafe {
        // 1. Trigger a Java callback that throws.
        invoke_throwing_callback(env, class, method_name_ref, signature_ref);

        // 2. ExceptionCheck before clearing - should be JNI_TRUE.
        let pending_before = ((*(*env)).v24.ExceptionCheck)(env);

        // 3. ExceptionOccurred - should return the pending throwable.
        //let occurred: jthrowable = ((*(*env)).v24.ExceptionOccurred)(env); // todo use me after FindClass is implemented
        let occurred_text = ""; //if occurred.is_null() {
                                //     "null".to_string()
                                // } else {
                                //     throwable_to_string(env, occurred)
                                // };

        // 4. ExceptionClear - drops the pending exception.
        ((*(*env)).v24.ExceptionClear)(env);

        // 5. ExceptionCheck after clearing - should be JNI_FALSE.
        let pending_after = ((*(*env)).v24.ExceptionCheck)(env);

        let summary = format!(
            "checkBefore={pending_before}, occurred=[{occurred_text}], checkAfter={pending_after}"
        );

        new_string_utf(env, &summary)
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ExceptionDemo_DescribeAndClearDemo(
    env: *mut JNIEnv,
    class: jclass,
    method_name_ref: jstring,
    signature_ref: jstring,
) -> jstring {
    unsafe {
        // 1. Trigger a Java callback that throws.
        invoke_throwing_callback(env, class, method_name_ref, signature_ref);

        let pending_before = ((*(*env)).v24.ExceptionCheck)(env);

        // 2. ExceptionDescribe - writes class/message to stderr in the impl.
        ((*(*env)).v24.ExceptionDescribe)(env);

        let pending_after = ((*(*env)).v24.ExceptionCheck)(env);

        let summary =
            format!("described, checkBefore={pending_before}, checkAfter={pending_after}");

        new_string_utf(env, &summary)
    }
}

unsafe fn invoke_throwing_callback(
    env: *mut JNIEnv,
    target_class: jclass,
    method_name_ref: jstring,
    signature_ref: jstring,
) {
    let method_name = ((*(*env)).v24.GetStringUTFChars)(env, method_name_ref, null_mut());
    let signature = ((*(*env)).v24.GetStringUTFChars)(env, signature_ref, null_mut());
    let method_id = ((*(*env)).v24.GetStaticMethodID)(env, target_class, method_name, signature);
    ((*(*env)).v24.ReleaseStringUTFChars)(env, signature_ref, signature);
    ((*(*env)).v24.ReleaseStringUTFChars)(env, method_name_ref, method_name);

    ((*(*env)).v24.CallStaticVoidMethodA)(env, target_class, method_id, std::ptr::null());
}

unsafe fn throwable_to_string(env: *mut JNIEnv, throwable: jthrowable) -> String {
    const THROWABLE_CLASS: &[u8] = b"java/lang/Throwable\0";
    const TO_STRING_NAME: &[u8] = b"toString\0";
    const TO_STRING_SIG: &[u8] = b"()Ljava/lang/String;\0";

    ((*(*env)).v24.ExceptionClear)(env);

    let throwable_class = ((*(*env)).v24.FindClass)(env, THROWABLE_CLASS.as_ptr() as *const _);
    let to_string_id = ((*(*env)).v24.GetMethodID)(
        env,
        throwable_class,
        TO_STRING_NAME.as_ptr() as *const _,
        TO_STRING_SIG.as_ptr() as *const _,
    );
    let no_args: *const jvalue = std::ptr::null();
    let str_obj: jobject =
        ((*(*env)).v24.CallObjectMethodA)(env, throwable as jobject, to_string_id, no_args);

    let text = if str_obj.is_null() {
        "<null>".to_string()
    } else {
        let raw = ((*(*env)).v24.GetStringUTFChars)(env, str_obj as jstring, null_mut());
        let owned = CStr::from_ptr(raw).to_string_lossy().into_owned();
        ((*(*env)).v24.ReleaseStringUTFChars)(env, str_obj as jstring, raw);
        owned
    };

    // Restore the pending exception so the caller observes the same state.
    ((*(*env)).v24.Throw)(env, throwable);
    text
}

unsafe fn new_string_utf(env: *mut JNIEnv, s: &str) -> jstring {
    let c = CString::new(s).expect("string contained NUL byte");
    ((*(*env)).v24.NewStringUTF)(env, c.as_ptr())
}
