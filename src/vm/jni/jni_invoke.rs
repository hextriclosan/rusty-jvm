use crate::vm::error::{Error, Result};
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::jni::jni_value::JNIValue;
use tracing::trace;

/// Convert a `Result<Vec<i32>>` from a Java method invocation into a typed JNI
/// return value.  On error the throwable is stored as a pending exception on
/// the current thread and `T::default()` is returned.
pub(super) fn jni_invoke<T: JNIValue>(result: Result<Vec<i32>>, context: &str) -> T {
    match result {
        Ok(raw) => T::from_vec(&raw),
        Err(e) => handle_error::<T>(e, context),
    }
}

/// Like [`jni_invoke`] but for results that return a single scalar (e.g. a heap
/// reference from a constructor) instead of a `Vec<i32>`.
pub(super) fn jni_invoke_scalar<T: JNIValue>(result: Result<i32>, context: &str) -> T {
    jni_invoke(result.map(|v| vec![v]), context)
}

fn handle_error<T: JNIValue>(e: Error, context: &str) -> T {
    trace!("JNI call threw exception in {context} ({e})");
    match e.throwable_ref() {
        Some(throwable_ref) => {
            match JavaThread::try_set_pending_exception(throwable_ref) {
                Ok(()) => (),
                Err(e) => {
                    panic!("Failed to set pending exception in {context}: {e}");
                }
            }

            T::default()
        }
        None => panic!("Non-exception VM error escaped JNI boundary in {context}: {e}"),
    }
}
