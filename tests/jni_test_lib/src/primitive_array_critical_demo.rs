use jni::sys::{jboolean, jclass, jint, jintArray, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniPrimitiveArrayCriticalDemo_modifyCritical(
    env: *mut JNIEnv,
    _class: jclass,
    array: jintArray,
) -> jboolean {
    unsafe {
        let mut is_copy = false;
        let elements = ((*(*env)).v24.GetPrimitiveArrayCritical)(env, array, &mut is_copy);
        let len = ((*(*env)).v24.GetArrayLength)(env, array) as usize;
        let values = std::slice::from_raw_parts_mut(elements.cast::<jint>(), len);
        values[0] = 40;
        values[len - 1] = 60;
        ((*(*env)).v24.ReleasePrimitiveArrayCritical)(env, array, elements, 0);
        is_copy
    }
}
