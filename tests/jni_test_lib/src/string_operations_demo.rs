use jni::sys::{jclass, jint, jstring, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_StringOperationsDemo_getStringLength__Ljava_lang_String_2(
    env: *mut JNIEnv,
    _class: jclass,
    input: jstring,
) -> jint {
    unsafe { ((*(*env)).v24.GetStringLength)(env, input) }
}
