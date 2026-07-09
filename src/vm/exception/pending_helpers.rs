//! `set_pending_*` exception helpers for the FFI/dispatcher native path.
//!
//! Unlike [`crate::vm::exception::helpers`], which construct an exception **and**
//! immediately unwind the Rust-side [`StackFrames`](crate::vm::stack::stack_frame::StackFrames),
//! these helpers only *construct* the throwable and record it as the current
//! thread's **pending** exception (the JNI convention). The actual stack unwinding
//! is performed later by the invoker, which owns the `StackFrames` and checks
//! [`JavaThread::take_pending_exception`] after every native call.
//!
//! This lets built-in natives implemented as `extern "system"` functions (which
//! cannot receive `&mut StackFrames` across the libffi boundary) raise precise
//! Java exceptions without threading the call stack into the FFI layer.

use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::stack::stack_value::StackValueKind;
use tracing::warn;

/// Sets a pending `NullPointerException` (no message), mirroring
/// `throw_null_pointer_exception`.
pub(crate) fn set_pending_null_pointer_exception() -> Result<()> {
    set_pending_exception("java/lang/NullPointerException", "<init>:()V", &[])
}

/// Sets a pending `ArrayStoreException` with the given message.
pub(crate) fn set_pending_array_store_exception(message: &str) -> Result<()> {
    set_pending_exception_with_message("java/lang/ArrayStoreException", message)
}

/// Sets a pending `ArrayIndexOutOfBoundsException` with the given message.
pub(crate) fn set_pending_array_index_out_of_bounds_exception(message: &str) -> Result<()> {
    set_pending_exception_with_message("java/lang/ArrayIndexOutOfBoundsException", message)
}

/// Sets a pending `ClassNotFoundException` with the given message.
pub(crate) fn set_pending_class_not_found_exception(message: &str) -> Result<()> {
    set_pending_exception_with_message("java/lang/ClassNotFoundException", message)
}

/// Sets a pending `FileNotFoundException` with the given file path and message.
pub(crate) fn set_pending_file_not_found_exception(path_ref: i32, message: &str) -> Result<()> {
    let reason_ref = StringPoolHelper::get_string(message)?;
    let args = vec![path_ref.into(), StackValueKind::from(reason_ref)];
    set_pending_exception(
        "java/io/FileNotFoundException",
        "<init>:(Ljava/lang/String;Ljava/lang/String;)V",
        &args,
    )
}

/// Sets a pending `IOException` with the given message.
pub(crate) fn set_pending_io_exception(message: &str) -> Result<()> {
    set_pending_exception_with_message("java/io/IOException", message)
}

fn set_pending_exception_with_message(class_name: &str, message: &str) -> Result<()> {
    let message_ref = StringPoolHelper::get_string(message)?;
    set_pending_exception(
        class_name,
        "<init>:(Ljava/lang/String;)V",
        &[StackValueKind::from(message_ref)],
    )
}

/// Constructs `class_name` via `constructor_signature`/`args` and records it as the
/// current thread's pending exception.
///
/// First-wins semantics: if an exception is already pending it is preserved and
/// this one is dropped (like OpenJDK), and the throwable is *not* constructed —
/// so we never run a Java constructor while an exception is already pending.
fn set_pending_exception(
    class_name: &str,
    constructor_signature: &str,
    args: &[StackValueKind],
) -> Result<()> {
    if let Some(pending_ref) = JavaThread::get_pending_exception() {
        warn!("Dropping {class_name}: an exception (ref={pending_ref}) is already pending");
        return Ok(());
    }

    let throwable_ref =
        Executor::invoke_args_constructor(class_name, constructor_signature, args, None)?;

    if let Err(e) = JavaThread::try_set_pending_exception(throwable_ref) {
        // Constructing the throwable ran Java code, which may itself have raised a
        // pending exception in the meantime. Honor first-wins and drop this one.
        warn!("Dropping {class_name}: {e}");
    }
    Ok(())
}
