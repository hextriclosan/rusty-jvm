use jni::sys::{jclass, jobjectArray, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_arrayops_JniObjectArrayNullClassDemo_create(
    env: *mut JNIEnv,
    _class: jclass,
    component: jclass,
) -> jobjectArray {
    unsafe { ((*(*env)).v24.NewObjectArray)(env, 1, component, std::ptr::null_mut()) }
}
