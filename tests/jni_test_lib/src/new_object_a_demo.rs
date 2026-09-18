use jni::sys::{jclass, jint, jobject, jstring, jvalue, JNIEnv};
use std::ffi::c_char;

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_objectops_JniNewObjectADemo_create(
    env: *mut JNIEnv,
    _class: jclass,
    target: jclass,
    number: jint,
    text: jstring,
) -> jobject {
    let arguments = [jvalue { i: number }, jvalue { l: text }];
    unsafe {
        let constructor = ((*(*env)).v24.GetMethodID)(
            env,
            target,
            b"<init>\0".as_ptr().cast::<c_char>(),
            b"(ILjava/lang/String;)V\0".as_ptr().cast::<c_char>(),
        );
        ((*(*env)).v24.NewObjectA)(env, target, constructor, arguments.as_ptr())
    }
}
