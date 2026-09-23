use jni::sys::{jarray, jclass, jint, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniArrayLengthNullDemo_length(
    env: *mut JNIEnv,
    _class: jclass,
    array: jarray,
) -> jint {
    unsafe { ((*(*env)).v24.GetArrayLength)(env, array) }
}
