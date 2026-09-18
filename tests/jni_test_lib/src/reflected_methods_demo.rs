use jni::sys::{jboolean, jclass, jobject, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_reflection_JniReflectedMethodsDemo_roundTrip(
    env: *mut JNIEnv,
    _class: jclass,
    reflected: jobject,
    declaring_class: jclass,
    is_static: jboolean,
) -> jobject {
    unsafe {
        let method_id = ((*(*env)).v24.FromReflectedMethod)(env, reflected);
        ((*(*env)).v24.ToReflectedMethod)(env, declaring_class, method_id, is_static)
    }
}
