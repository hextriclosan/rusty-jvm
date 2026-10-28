use jni::sys::{
    jboolean, jclass, jint, jintArray, jlong, jlongArray, JNIEnv, JNI_ABORT, JNI_COMMIT,
};
use std::slice;

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniPrimitiveArrayCriticalDemo_modifyCritical(
    env: *mut JNIEnv,
    _class: jclass,
    array: jintArray,
) -> jboolean {
    unsafe {
        // No other JNI calls are allowed inside the critical region
        let len = ((*(*env)).v24.GetArrayLength)(env, array) as usize;
        let mut is_copy = false;
        let elements = ((*(*env)).v24.GetPrimitiveArrayCritical)(env, array, &mut is_copy);
        let values = slice::from_raw_parts_mut(elements.cast::<jint>(), len);
        values[0] = 40;
        values[len - 1] = 60;
        ((*(*env)).v24.ReleasePrimitiveArrayCritical)(env, array, elements, 0);
        is_copy
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniPrimitiveArrayCriticalDemo_commitThenAbortCritical(
    env: *mut JNIEnv,
    _class: jclass,
    array: jlongArray,
) {
    unsafe {
        let len = ((*(*env)).v24.GetArrayLength)(env, array) as usize;
        let elements = ((*(*env)).v24.GetPrimitiveArrayCritical)(env, array, std::ptr::null_mut());
        let values = slice::from_raw_parts_mut(elements.cast::<jlong>(), len);
        values[0] = 100;
        // JNI_COMMIT copies back but keeps the buffer alive
        ((*(*env)).v24.ReleasePrimitiveArrayCritical)(env, array, elements, JNI_COMMIT);
        values[1] = 200;
        // JNI_ABORT frees the buffer without copying back
        ((*(*env)).v24.ReleasePrimitiveArrayCritical)(env, array, elements, JNI_ABORT);
    }
}
