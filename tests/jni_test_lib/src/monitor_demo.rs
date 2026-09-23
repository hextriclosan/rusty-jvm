use jni::sys::{jclass, jint, jobject, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_monitor_JniMonitorDemo_roundTrip(
    env: *mut JNIEnv,
    _class: jclass,
    object: jobject,
) -> jint {
    unsafe {
        let first_enter = ((*(*env)).v24.MonitorEnter)(env, object);
        let second_enter = ((*(*env)).v24.MonitorEnter)(env, object);
        let first_exit = ((*(*env)).v24.MonitorExit)(env, object);
        let second_exit = ((*(*env)).v24.MonitorExit)(env, object);
        first_enter | second_enter | first_exit | second_exit
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_monitor_JniMonitorDemo_exitWithoutOwnership(
    env: *mut JNIEnv,
    _class: jclass,
    object: jobject,
) -> jint {
    unsafe { ((*(*env)).v24.MonitorExit)(env, object) }
}
