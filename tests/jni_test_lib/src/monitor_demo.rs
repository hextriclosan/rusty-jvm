use jni::sys::{jboolean, jclass, jint, jobject, jvalue, JNIEnv};
use std::ffi::CString;

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
pub extern "system" fn Java_samples_javacore_loadlibrary_monitor_JniMonitorDemo_exit(
    env: *mut JNIEnv,
    _class: jclass,
    object: jobject,
) -> jint {
    unsafe { ((*(*env)).v24.MonitorExit)(env, object) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_monitor_JniMonitorDemo_enter(
    env: *mut JNIEnv,
    _class: jclass,
    object: jobject,
) -> jint {
    unsafe { ((*(*env)).v24.MonitorEnter)(env, object) }
}

/// `Thread.holdsLock(object)`, asked through JNI from inside a native, by way of the fixture's
/// `holdsLockOf` wrapper: `holdsLock` is itself native, which a JNI call cannot target here.
unsafe fn holds_lock(env: *mut JNIEnv, object: jobject) -> jboolean {
    let demo_class_name =
        CString::new("samples/javacore/loadlibrary/monitor/JniMonitorDemo").unwrap();
    let demo_class = ((*(*env)).v24.FindClass)(env, demo_class_name.as_ptr());
    let method_name = CString::new("holdsLockOf").unwrap();
    let signature = CString::new("(Ljava/lang/Object;)Z").unwrap();
    let method_id = ((*(*env)).v24.GetStaticMethodID)(
        env,
        demo_class,
        method_name.as_ptr(),
        signature.as_ptr(),
    );
    let args = [jvalue { l: object }];
    ((*(*env)).v24.CallStaticBooleanMethodA)(env, demo_class, method_id, args.as_ptr())
}

/// `synchronized native boolean heldInNative()`: the VM entered the receiver's monitor before
/// calling the native, so the native finds it held.
#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_monitor_JniMonitorDemo_heldInNative(
    env: *mut JNIEnv,
    this: jobject,
) -> jboolean {
    unsafe { holds_lock(env, this) }
}

/// `static synchronized native boolean heldInStaticNative()`: the class's monitor, likewise.
#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_monitor_JniMonitorDemo_heldInStaticNative(
    env: *mut JNIEnv,
    class: jclass,
) -> jboolean {
    unsafe { holds_lock(env, class) }
}

/// `synchronized native void throwInNative()`: the native leaves through a pending exception,
/// which the VM must throw only after giving the monitor up.
#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_monitor_JniMonitorDemo_throwInNative(
    env: *mut JNIEnv,
    _this: jobject,
) {
    unsafe {
        let class_name = CString::new("java/lang/IllegalStateException").unwrap();
        let clazz = ((*(*env)).v24.FindClass)(env, class_name.as_ptr());
        let message = CString::new("thrown in native").unwrap();
        ((*(*env)).v24.ThrowNew)(env, clazz, message.as_ptr());
    }
}
