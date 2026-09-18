use jni::sys::{jbyteArray, jclass, jobject, jstring, JNIEnv, JNI_ABORT};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_classops_JniDefineClassDemo_define(
    env: *mut JNIEnv,
    _class: jclass,
    name: jstring,
    loader: jobject,
    bytecode: jbyteArray,
) -> jclass {
    unsafe {
        let name_chars = if name.is_null() {
            std::ptr::null()
        } else {
            ((*(*env)).v24.GetStringUTFChars)(env, name, std::ptr::null_mut())
        };
        let bytes = ((*(*env)).v24.GetByteArrayElements)(env, bytecode, std::ptr::null_mut());
        let length = ((*(*env)).v24.GetArrayLength)(env, bytecode);
        let defined = ((*(*env)).v24.DefineClass)(env, name_chars, loader, bytes, length);
        ((*(*env)).v24.ReleaseByteArrayElements)(env, bytecode, bytes, JNI_ABORT);
        if !name.is_null() {
            ((*(*env)).v24.ReleaseStringUTFChars)(env, name, name_chars);
        }
        defined
    }
}
