use jni::sys::{jclass, jobject, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_ObjectOperationsDemo_GetObjectClass(
    env: *mut JNIEnv,
    _class: jclass,
    input: jobject,
) -> jclass {
    unsafe { ((*(*env)).v24.GetObjectClass)(env, input) }
}
