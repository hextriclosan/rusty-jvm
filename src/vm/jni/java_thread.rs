use crate::vm::error::ErrorKind::JniExceptionAlreadyPending;
use crate::vm::error::{Error, Result};
use crate::vm::jni::jni_env::jni_native_interface;
use crate::vm::safepoint::{self, Safepoint};
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_frame::{StackFrame, StackFrames};
use jni_sys::{JNIEnv, JNINativeInterface_};
use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

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
    /// Heap reference to this OS thread's `java.lang.Thread`, or `None` before it is attached.
    ///
    /// This is what `Thread.currentThread()` returns, making thread identity a per-OS-thread fact
    /// rather than a single VM-wide global — the precondition for spawning real threads.
    current_thread: Cell<Option<i32>>,
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
    /// This thread's cooperative safepoint, polled by the interpreter loop. Shared (by `Arc`) with
    /// the global registry once the thread is attached, so another thread can drive it to a
    /// safepoint to snapshot its stack. See [`crate::vm::safepoint`].
    safepoint: Arc<Safepoint>,
    /// References held alive across a VM operation that has no frame to put them in.
    ///
    /// Almost every reference the VM touches lives in some frame's locals or operand stack, which
    /// is what a root scan walks. A few do not: values popped off the caller before a callee's
    /// frame exists, or allocated by VM-side code that then calls into Java. While that code runs
    /// they exist only in Rust locals, where no scan can see them — and the code in question can
    /// run class initializers or re-enter the interpreter, so a collection really can happen in the
    /// middle of it.
    ///
    /// Scopes nest and unwind in LIFO order via [`TempRootsGuard`], so this is a stack, not a set.
    temp_roots: RefCell<Vec<Slot>>,
}

thread_local! {
    static JAVA_THREAD: JavaThread = JavaThread {
        env: jni_native_interface(),
        exception_pending: Cell::new(None),
        current_thread: Cell::new(None),
        stack_frames: RefCell::new(Vec::new()),
        safepoint: Safepoint::new(),
        temp_roots: RefCell::new(Vec::new()),
    };
}

/// RAII guard returned by [`JavaThread::hold_temp_roots`]; drops its scope's references on drop,
/// restoring the stack to the depth it had on entry.
///
/// Deliberately `!Send`. The guard names a depth in *the creating thread's* `temp_roots`, but `Drop`
/// resolves `JAVA_THREAD` on whichever thread runs it. Were the guard to cross a thread boundary it
/// would truncate a stack it never pushed to — discarding live roots on the destination thread while
/// the origin's scope leaked, and neither failure would be visible where it was caused.
///
/// [`StackFramesGuard`] has the same requirement and gets it for free, since a raw pointer is
/// already `!Send`; a bare `usize` is not, hence the marker.
pub(crate) struct TempRootsGuard {
    restore_to: usize,
    /// Depth of this scope's top, kept so [`Self::add_root`] can check it is still the innermost.
    scope_end: usize,
    _not_send: PhantomData<Rc<()>>,
}

impl TempRootsGuard {
    /// Adds `slot` to this scope, for references acquired one at a time with Java calls in between.
    ///
    /// Only the innermost live scope may grow: a slot pushed above a nested scope is discarded when
    /// *that* scope unwinds, while this guard goes on assuming it is rooted.
    pub(crate) fn add_root(&mut self, slot: Slot) {
        JAVA_THREAD.with(|t| {
            let mut temp_roots = t.temp_roots.borrow_mut();
            debug_assert_eq!(temp_roots.len(), self.scope_end, "not the innermost scope");
            temp_roots.push(slot);
            self.scope_end = temp_roots.len();
        });
    }
}

impl Drop for TempRootsGuard {
    fn drop(&mut self) {
        JAVA_THREAD.with(|t| t.temp_roots.borrow_mut().truncate(self.restore_to));
    }
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

    /// Returns the `java.lang.Thread` heap reference for the calling OS thread, or `None` if it has
    /// not been attached yet (i.e. `Thread.currentThread()` has no answer for this thread).
    pub(crate) fn current_thread() -> Option<i32> {
        JAVA_THREAD.with(|t| t.current_thread.get())
    }

    /// Binds `thread_ref` as the `java.lang.Thread` of the calling OS thread. Must be set before any
    /// code on this thread observes `Thread.currentThread()` — for the main thread this is done at
    /// VM bootstrap, before `Thread.<init>` (which itself calls `currentThread()`) runs.
    pub(crate) fn set_current_thread(thread_ref: i32) {
        JAVA_THREAD.with(|t| {
            t.current_thread.set(Some(thread_ref));
            // Publish this thread's safepoint under its identity so it can be targeted for a stack dump.
            safepoint::register(thread_ref, Arc::clone(&t.safepoint));
        });
    }

    /// Keeps `slots` reachable for a root scan until the returned guard drops.
    ///
    /// For VM operations that hold references with no frame to put them in; see the `temp_roots`
    /// field. Guards must be dropped in LIFO order, which holding them as locals gives naturally.
    pub(crate) fn hold_temp_roots(slots: Vec<Slot>) -> TempRootsGuard {
        JAVA_THREAD.with(|t| {
            let mut temp_roots = t.temp_roots.borrow_mut();
            let restore_to = temp_roots.len();
            temp_roots.extend(slots);
            TempRootsGuard {
                restore_to,
                scope_end: temp_roots.len(),
                _not_send: PhantomData,
            }
        })
    }

    /// Every reference currently held in this thread's temporary root scopes.
    ///
    /// Read by the calling thread about itself, the same way a thread collects its own stack at a
    /// safepoint rather than having another thread reach into it.
    #[allow(dead_code)] // consumed by the root scan added in a later step
    pub(crate) fn temp_root_refs() -> Vec<i32> {
        JAVA_THREAD.with(|t| {
            t.temp_roots
                .borrow()
                .iter()
                .filter(|slot| slot.is_ref())
                .map(|slot| slot.value())
                .collect()
        })
    }

    /// Returns a clone of the calling thread's safepoint. `Engine::execute` grabs it once and polls
    /// it every iteration (a cheap flag check).
    pub(crate) fn safepoint() -> Arc<Safepoint> {
        JAVA_THREAD.with(|t| Arc::clone(&t.safepoint))
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

#[cfg(test)]
mod temp_roots_tests {
    use super::*;

    #[test]
    fn should_hold_references_for_the_life_of_the_scope() {
        assert!(JavaThread::temp_root_refs().is_empty());

        let held = JavaThread::hold_temp_roots(vec![Slot::Ref(5), Slot::Value(6), Slot::Ref(7)]);
        // The untagged 6 is an ordinary value that merely looks like a reference.
        assert_eq!(JavaThread::temp_root_refs(), vec![5, 7]);

        drop(held);
        assert!(JavaThread::temp_root_refs().is_empty());
    }

    /// The guard must not cross threads: `Drop` truncates whichever thread's `temp_roots` it runs
    /// on, so a guard dropped elsewhere would discard that thread's live roots and leak its own.
    /// The `PhantomData<Rc<()>>` in the struct is what prevents it; this fails if that is removed.
    #[test]
    fn should_keep_the_temp_roots_guard_thread_bound() {
        struct Probe<T>(PhantomData<T>);

        trait MaybeSend {
            const IS_SEND: bool = false;
        }
        impl<T> MaybeSend for Probe<T> {}

        // An inherent const wins over the trait's, but only when its `Send` bound is satisfied.
        impl<T: Send> Probe<T> {
            const IS_SEND: bool = true;
        }

        const { assert!(!<Probe<TempRootsGuard>>::IS_SEND) };
        const { assert!(<Probe<usize>>::IS_SEND, "probe does not discriminate") };
    }

    /// A scope grows as references appear, and drops everything it accumulated.
    #[test]
    fn should_hold_references_added_after_the_scope_opened() {
        let mut held = JavaThread::hold_temp_roots(vec![Slot::Ref(5)]);
        held.add_root(Slot::Ref(6));
        // A nested scope opened and closed in between leaves the outer one appendable.
        drop(JavaThread::hold_temp_roots(vec![Slot::Ref(7)]));
        held.add_root(Slot::Ref(8));
        assert_eq!(JavaThread::temp_root_refs(), vec![5, 6, 8]);

        drop(held);
        assert!(JavaThread::temp_root_refs().is_empty());
    }

    /// Scopes nest: a polymorphic call can re-enter Java and reach another one.
    #[test]
    fn should_unwind_nested_scopes_in_order() {
        let outer = JavaThread::hold_temp_roots(vec![Slot::Ref(1)]);
        let inner = JavaThread::hold_temp_roots(vec![Slot::Ref(2)]);
        assert_eq!(JavaThread::temp_root_refs(), vec![1, 2]);

        drop(inner);
        assert_eq!(JavaThread::temp_root_refs(), vec![1]);

        drop(outer);
        assert!(JavaThread::temp_root_refs().is_empty());
    }
}
