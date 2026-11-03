use crate::vm::error::{Error, Result};
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::jni::jni_value::JNIValue;
use crate::vm::stack::rooted_slots::RootedSlots;
use tracing::{trace, warn};

/// Convert a rooted, typed Java result into the JNI ABI representation.
/// On error the throwable is stored as a pending exception on
/// the current thread and `T::default()` is returned.
pub(super) fn jni_invoke<T: JNIValue>(result: Result<RootedSlots>, context: &str) -> T {
    match result {
        Ok(value) => T::from_vec(&value.raw()),
        Err(e) => handle_error::<T>(e, context),
    }
}

/// Constructor spelling of [`jni_invoke`]; the returned reference stays rooted
/// throughout conversion to its JNI representation.
pub(super) fn jni_invoke_scalar<T: JNIValue>(result: Result<RootedSlots>, context: &str) -> T {
    jni_invoke(result, context)
}

fn handle_error<T: JNIValue>(e: Error, context: &str) -> T {
    trace!("JNI call threw exception in {context} ({e})");
    match e.throwable_ref() {
        Some(throwable_ref) => {
            match JavaThread::try_set_pending_exception(throwable_ref) {
                Ok(()) => (),
                // First-wins: an exception is already pending, so preserve it
                // and drop this one, like OpenJDK does. Never panic — this runs at the JNI/FFI
                // boundary where unwinding would abort the VM.
                Err(e) => warn!("Dropping exception in {context}: {e}"),
            }

            T::default()
        }
        None => panic!("Non-exception VM error escaped JNI boundary in {context}: {e}"),
    }
}
