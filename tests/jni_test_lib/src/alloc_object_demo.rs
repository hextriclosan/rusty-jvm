use jni::sys::{jclass, jobject, JNIEnv};

/// Returns the pending exception (cleared) when `AllocObject` correctly returned null,
/// otherwise the allocated object.
#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_objectops_JniAllocObjectDemo_allocate(
    env: *mut JNIEnv,
    _class: jclass,
    target: jclass,
) -> jobject {
    unsafe {
        let obj = ((*(*env)).v24.AllocObject)(env, target);
        let exception = ((*(*env)).v24.ExceptionOccurred)(env);
        if !exception.is_null() {
            ((*(*env)).v24.ExceptionClear)(env);
            if obj.is_null() {
                return exception;
            }
        }
        obj
    }
}
