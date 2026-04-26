use crate::from_mutf8_ptr;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::helper::klass;
use crate::vm::invoke_uncaught_exception_handler;
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::stack::stack_value::StackValueKind;
use jni_sys::{jboolean, jclass, jint, jthrowable, JNIEnv, JNI_FALSE, JNI_OK, JNI_TRUE};
use std::ffi::{c_char, CStr};
use std::ptr::null_mut;

pub(super) extern "system" fn throw(_env: *mut JNIEnv, throwable: jthrowable) -> jint {
    // `throwable` is a JNI opaque pointer whose numeric value is a heap reference (i32).
    // Truncating to i32 is safe because all heap refs in this JVM fit in 32 bits.
    JavaThread::replace_pending_exception(throwable as i32);
    JNI_OK
}

pub(super) extern "system" fn throw_new(
    _env: *mut JNIEnv,
    clazz: jclass,
    msg_mutf8: *const c_char,
) -> jint {
    // clear any pending exception, otherwise we'll get an UncaughtException when we invoke the constructor
    JavaThread::take_pending_exception();

    let klass = klass(clazz as i32).expect("Failed to get class from reference for exception");

    let throwable_res = if msg_mutf8.is_null() {
        Executor::invoke_default_constructor_jc(&klass)
    } else {
        let msg = from_mutf8_ptr!(msg_mutf8).expect("Failed to convert message from CESU-8");
        let msg_ref = StringPoolHelper::get_string(&msg)
            .expect("Failed to get string reference from string pool");
        Executor::invoke_args_constructor(
            klass.this_class_name(),
            "<init>:(Ljava/lang/String;)V",
            &[StackValueKind::from(msg_ref)],
            None,
        )
    };

    let throwable_ref = throwable_res.expect("Failed to construct exception instance");
    JavaThread::replace_pending_exception(throwable_ref);
    JNI_OK
}

pub(super) extern "system" fn exception_occurred(_env: *mut JNIEnv) -> jthrowable {
    match JavaThread::get_pending_exception() {
        // Heap references are i32 values; widen to usize before casting to the opaque pointer
        // type required by JNI. The value is never dereferenced as a real pointer.
        Some(r) => r as usize as jthrowable,
        None => null_mut(),
    }
}

pub(super) extern "system" fn exception_describe(_env: *mut JNIEnv) {
    if let Some(exception_ref) = JavaThread::take_pending_exception() {
        invoke_uncaught_exception_handler(exception_ref)
            .expect("Failed to invoke uncaught exception handler");
    }
}

pub(super) extern "system" fn exception_clear(_env: *mut JNIEnv) {
    JavaThread::take_pending_exception();
}

pub(super) extern "system" fn fatal_error(_env: *mut JNIEnv, msg: *const c_char) -> ! {
    if !msg.is_null() {
        let msg_str = unsafe { CStr::from_ptr(msg) }.to_string_lossy();
        eprintln!("FATAL ERROR in native method: {msg_str}");
    } else {
        eprintln!("FATAL ERROR in native method");
    }
    std::process::abort()
}

pub(super) extern "system" fn exception_check(_env: *mut JNIEnv) -> jboolean {
    let pending = JavaThread::get_pending_exception();
    if pending.is_some() {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}
