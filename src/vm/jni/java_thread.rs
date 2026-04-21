use crate::vm::jni::jni_env::jni_native_interface;
use jni_sys::{JNIEnv, JNINativeInterface_};
use std::cell::Cell;

/// Represents a JVM thread for JNI purposes.
///
/// The `env` field **must** be first because the JNI specification requires that `*mut JNIEnv`
/// points at the function-table pointer, which is the first word of the per-thread state.
/// Native code can therefore cast a `*mut JNIEnv` back to `*mut JavaThread` to reach
/// thread-local data.
#[repr(C)]
pub(crate) struct JavaThread {
    /// Pointer to the JNI function table.  The address of this field is what `*mut JNIEnv`
    /// ultimately points *at*; see [`crate::vm::jni::jni_env::get_jni_env`].
    env: *const JNINativeInterface_,
    /// Heap reference to the currently-pending exception, or `None` if no exception is pending.
    exception_pending: Cell<Option<i32>>,
}

thread_local! {
    static JAVA_THREAD: JavaThread = JavaThread {
        env: jni_native_interface(),
        exception_pending: Cell::new(None),
    };
}

impl JavaThread {
    /// Returns a `*mut JNIEnv` pointing at the `env` field of the current thread's
    /// [`JavaThread`].  This is what every JNI call receives as its first argument.
    pub(super) fn get_env_ptr() -> *mut JNIEnv {
        // Thread-local statics have a stable address for the lifetime of the thread, so the
        // pointer we return remains valid for as long as any JNI code on this thread can use it.
        JAVA_THREAD.with(|t| (&raw const t.env) as *mut JNIEnv)
    }

    /// Takes the currently-pending exception (if any) for the calling thread, leaving `None` in its place.
    pub(crate) fn take_pending_exception() -> Option<i32> {
        JAVA_THREAD.with(|t| t.exception_pending.take())
    }

    /// Stores `throwable_ref` as the pending exception for the calling thread.
    ///
    /// Returns `true` if it was stored, or `false` if another exception was already pending
    /// (in which case the existing pending exception is preserved, matching JNI semantics).
    pub(crate) fn set_pending_exception(throwable_ref: i32) -> bool {
        JAVA_THREAD.with(|t| {
            if t.exception_pending.get().is_none() {
                t.exception_pending.set(Some(throwable_ref));
                true
            } else {
                false
            }
        })
    }
}
