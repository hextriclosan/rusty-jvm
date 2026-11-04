use jni::sys::{jclass, jint, jintArray, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_gc_GcBudgetDemo_oversizedJniArray(
    env: *mut JNIEnv,
    _class: jclass,
    length: jint,
) -> jintArray {
    unsafe { ((*(*env)).v24.NewIntArray)(env, length) }
}
