use jni::sys::{jclass, jobject, jobjectArray, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniObjectArrayStoreDemo_set(
    env: *mut JNIEnv,
    _class: jclass,
    array: jobjectArray,
    value: jobject,
) {
    unsafe { ((*(*env)).v24.SetObjectArrayElement)(env, array, 0, value) }
}
