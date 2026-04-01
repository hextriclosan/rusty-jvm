use jni::sys::{jclass, jint, jobject, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ArrayOperationsDemo_GetArrayLength(
    env: *mut JNIEnv,
    _class: jclass,
    input: jobject,
) -> jint {
    unsafe { ((*(*env)).v24.GetArrayLength)(env, input) }
}
