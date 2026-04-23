use jni::sys::{jclass, jobject, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_JniNoSuchIdDemo_lookupNonexistentFieldId(
    env: *mut JNIEnv,
    obj: jobject,
) {
    unsafe {
        let class = ((*(*env)).v24.GetObjectClass)(env, obj);
        ((*(*env)).v24.GetFieldID)(
            env,
            class,
            b"nonexistentField\0".as_ptr() as *const i8,
            b"I\0".as_ptr() as *const i8,
        );
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_JniNoSuchIdDemo_lookupNonexistentStaticFieldId(
    env: *mut JNIEnv,
    class: jclass,
) {
    unsafe {
        ((*(*env)).v24.GetStaticFieldID)(
            env,
            class,
            b"nonexistentStaticField\0".as_ptr() as *const i8,
            b"I\0".as_ptr() as *const i8,
        );
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_JniNoSuchIdDemo_lookupNonexistentMethodId(
    env: *mut JNIEnv,
    obj: jobject,
) {
    unsafe {
        let class = ((*(*env)).v24.GetObjectClass)(env, obj);
        ((*(*env)).v24.GetMethodID)(
            env,
            class,
            b"nonexistentMethod\0".as_ptr() as *const i8,
            b"()V\0".as_ptr() as *const i8,
        );
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_example_JniNoSuchIdDemo_lookupNonexistentStaticMethodId(
    env: *mut JNIEnv,
    class: jclass,
) {
    unsafe {
        ((*(*env)).v24.GetStaticMethodID)(
            env,
            class,
            b"nonexistentStaticMethod\0".as_ptr() as *const i8,
            b"()V\0".as_ptr() as *const i8,
        );
    }
}
