use crate::vm::error::ErrorKind::JniExceptionAlreadyPending;
use crate::vm::error::{Error, Result};
use crate::vm::jni::jni_env::jni_native_interface;
use crate::vm::stack::stack_frame::{StackFrame, StackFrames};
use jni_sys::{JNIEnv, JNINativeInterface_};
use std::cell::{Cell, RefCell};

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
    /// Chain of live [`StackFrames`] *segments* for the calling thread, ordered oldest → newest.
    ///
    /// Each `Engine::execute` invocation owns one `StackFrames` and registers a pointer to it here
    /// via [`JavaThread::register_stack_frames`] for its whole duration. Because a native method
    /// can re-enter the interpreter (`native → Executor → Engine::execute`), a single logical
    /// thread stack is physically split into several segments interleaved on the real call stack;
    /// this chain lets stack-walking natives (e.g. `Reflection.getCallerClass`) traverse *all* of
    /// them, newest first — the reified analogue of HotSpot's `last_Java_frame` anchor chain.
    ///
    /// Only the newest (top) segment is being mutated by the running interpreter; every older
    /// segment is suspended in a native call and therefore safe to read.
    stack_frames: RefCell<Vec<*mut StackFrames>>,
}

thread_local! {
    static JAVA_THREAD: JavaThread = JavaThread {
        env: jni_native_interface(),
        exception_pending: Cell::new(None),
        stack_frames: RefCell::new(Vec::new()),
    };
}

/// RAII guard returned by [`JavaThread::register_stack_frames`]; pops its segment off the
/// thread-local chain on drop.
///
/// Guards are held as locals by nested `Engine::execute` calls, so they drop in strict LIFO
/// order (matching push order) and are exception-safe on unwind.
pub(crate) struct StackFramesGuard {
    segment: *mut StackFrames,
}

impl Drop for StackFramesGuard {
    fn drop(&mut self) {
        JAVA_THREAD.with(|t| {
            let popped = t.stack_frames.borrow_mut().pop();
            debug_assert_eq!(
                popped,
                Some(self.segment),
                "StackFrames chain popped out of LIFO order"
            );
        });
    }
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

    pub(crate) fn get_pending_exception() -> Option<i32> {
        JAVA_THREAD.with(|t| t.exception_pending.get())
    }

    /// Stores `throwable_ref` as the pending exception for the calling thread.
    ///
    /// Returns `Err(JniAlreadyPendingException(existing))` without modifying state
    /// when another exception is already pending; the first exception is preserved,
    /// per JNI semantics. Used by VM-internal paths that surface a Rust `Err` as a
    /// Java throwable (`GetFieldID`, `NewObject`, `CallXxxMethod`, ...). The dedicated
    /// `Throw` / `ThrowNew` JNI entry points bypass this and overwrite directly,
    /// which the spec explicitly permits.
    pub(crate) fn try_set_pending_exception(throwable_ref: i32) -> Result<()> {
        JAVA_THREAD.with(|t| {
            if let Some(pending_ref) = t.exception_pending.get() {
                Err(Error::new(JniExceptionAlreadyPending(pending_ref)))
            } else {
                t.exception_pending.set(Some(throwable_ref));
                Ok(())
            }
        })
    }

    /// Used by JNI `Throw`/`ThrowNew`: always installs `throwable_ref`,
    /// replacing any previously-pending exception. This is spec-compliant.
    pub(super) fn force_set_pending_exception(throwable_ref: i32) {
        JAVA_THREAD.with(|t| t.exception_pending.set(Some(throwable_ref)));
    }

    /// Registers `stack_frames` as the newest segment of the calling thread's stack chain for
    /// the lifetime of the returned guard, so stack-walking natives can traverse it via
    /// [`JavaThread::with_frames`] without receiving it as a parameter.
    ///
    /// Call this once per `Engine::execute` invocation (each owns one `StackFrames`). The guard
    /// pops the segment on drop, keeping the chain correct across native re-entries and unwinding.
    pub(crate) fn register_stack_frames(stack_frames: &mut StackFrames) -> StackFramesGuard {
        let segment = stack_frames as *mut StackFrames;
        JAVA_THREAD.with(|t| t.stack_frames.borrow_mut().push(segment));
        StackFramesGuard { segment }
    }

    /// Invokes `f` with a mutable reference to the **newest** (top) `StackFrames` segment of the
    /// calling thread — the one owned by the interpreter loop currently paused in this native call.
    ///
    /// This is the mutable, top-only counterpart of [`JavaThread::with_frames`]: polymorphic
    /// intrinsics (`MethodHandle.invoke*`) reach the caller's operand stack through it without
    /// receiving `&mut StackFrames` as a parameter, which keeps native dispatch free of interpreter
    /// stack plumbing.
    ///
    /// Returns an execution error when no segment is registered (i.e. outside any interpreter
    /// invocation), which should never happen for a native dispatched through the interpreter.
    ///
    /// The borrow handed to `f` **must not** be held across code that re-enters native dispatch on
    /// the same segment (keep `f` a minimal, non-nesting leaf), otherwise two `&mut` to the same
    /// `StackFrames` would alias.
    pub(crate) fn with_top_frames_mut<R>(f: impl FnOnce(&mut StackFrames) -> R) -> Result<R> {
        let segment = JAVA_THREAD.with(|t| {
            // Copy the pointer out and drop the chain borrow before calling `f`, so `f` may consult
            // the chain again (e.g. via `with_frames`) without a `RefCell` double-borrow.
            t.stack_frames.borrow().last().copied().ok_or_else(|| {
                Error::new_execution("no stack frames registered for the current thread")
            })
        })?;
        // SAFETY: `segment` is the newest pointer installed by `register_stack_frames` from a live
        // `StackFrames` owned by the `Engine::execute` frame currently on the call stack, removed by
        // that frame's guard before the `StackFrames` is dropped. The interpreter that owns it is
        // paused in this native call, so no other `&mut` to it is active for the duration of `f`.
        Ok(f(unsafe { &mut *segment }))
    }

    /// Invokes `f` with an iterator over **all** live Java frames of the calling thread, newest
    /// first, walking across every registered `StackFrames` segment (i.e. across interpreter
    /// re-entries through native code).
    ///
    /// Returns an execution error when no segment is registered (i.e. outside any interpreter
    /// invocation), which should never happen for a native dispatched through the interpreter.
    pub(crate) fn with_frames<R>(
        f: impl FnOnce(&mut dyn Iterator<Item = &StackFrame>) -> R,
    ) -> Result<R> {
        JAVA_THREAD.with(|t| {
            let segments = t.stack_frames.borrow();
            if segments.is_empty() {
                return Err(Error::new_execution(
                    "no stack frames registered for the current thread",
                ));
            }
            // SAFETY: every pointer was installed by `register_stack_frames` from a live
            // `StackFrames` owned by an `Engine::execute` frame currently on the call stack, and
            // is removed by that frame's guard before the `StackFrames` is dropped. All segments
            // except the newest are suspended in a native call, so shared reads are sound; the
            // newest is only read here while the interpreter is paused in the calling native.
            let mut iter = segments
                .iter()
                .rev()
                .flat_map(|&segment| unsafe { (*segment).iter().rev() });
            Ok(f(&mut iter))
        })
    }
}

#[cfg(test)]
#[test]
fn try_set_preserves_first() {
    let _ = JavaThread::take_pending_exception();
    assert!(JavaThread::try_set_pending_exception(11).is_ok());
    let err = JavaThread::try_set_pending_exception(22).unwrap_err();
    assert!(matches!(err.kind(), JniExceptionAlreadyPending(11)));
    assert_eq!(JavaThread::take_pending_exception(), Some(11));
}
