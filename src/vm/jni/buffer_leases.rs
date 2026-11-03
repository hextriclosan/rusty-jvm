//! Copied native buffers own their allocation and a root until release or native return.
use crate::vm::heap::heap::HEAP;
use crate::vm::jni::handles::{HandleError, HandleResult};
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::jni::utils::set_pending_internal_error;
use jni_sys::{jint, JNI_ABORT, JNI_COMMIT};
use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

pub(crate) struct BufferLease {
    pub(super) frame: usize,
    pub(super) target: i32,
    pointer: NonNull<u8>,
    layout: Layout,
    len: usize,
}

impl Drop for BufferLease {
    fn drop(&mut self) {
        // SAFETY: this lease uniquely owns the allocation made with exactly this layout.
        unsafe { dealloc(self.pointer.as_ptr(), self.layout) };
    }
}

/// Alignment is explicit even for critical arrays (whose element type is erased).
pub(super) fn copy(target: i32, bytes: &[u8], alignment: usize) -> *mut u8 {
    let result = (|| -> HandleResult<*mut u8> {
        let frame = JavaThread::with_jni_frames(|frames| frames.native_frame())?;
        let layout = Layout::from_size_align(bytes.len().max(1), alignment)
            .map_err(|_| HandleError::OutOfMemory)?;
        JavaThread::with_jni_leases(|leases| {
            leases
                .try_reserve(1)
                .map_err(|_| HandleError::OutOfMemory)?;
            // SAFETY: layout has nonzero size and valid alignment. The returned memory is
            // initialized before exposure to native code; a null allocation reports OOM.
            let pointer =
                NonNull::new(unsafe { alloc(layout) }).ok_or(HandleError::OutOfMemory)?;
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), pointer.as_ptr(), bytes.len())
            };
            leases.push(BufferLease {
                frame,
                target,
                pointer,
                layout,
                len: bytes.len(),
            });
            Ok(pointer.as_ptr())
        })
    })();
    match result {
        Ok(pointer) => pointer,
        Err(error) => {
            error.set_pending();
            std::ptr::null_mut()
        }
    }
}

pub(super) fn release(pointer: *const u8, target: i32, array_mode: Option<jint>) {
    if pointer.is_null() {
        return;
    }
    let result = JavaThread::with_jni_leases(|leases| {
        let Some(index) = leases.iter().position(|lease| {
            lease.pointer.as_ptr().cast_const() == pointer && lease.target == target
        }) else {
            return Err("Invalid JNI buffer lease");
        };
        if let Some(mode) = array_mode {
            if !matches!(mode, 0 | JNI_COMMIT | JNI_ABORT) {
                return Err("Invalid JNI array release mode");
            }
            if mode != JNI_ABORT {
                let lease = &leases[index];
                // SAFETY: the lease owns len initialized bytes; the JNI caller has finished
                // modifying them before Release. No Rust slice survives across native calls.
                let bytes = unsafe { std::slice::from_raw_parts(pointer, lease.len) };
                let Ok(mut array) = HEAP.get_entire_raw_data_mut(lease.target) else {
                    return Err("JNI array lease target is missing");
                };
                array.copy_from_slice(bytes);
            }
            if mode == JNI_COMMIT {
                return Ok(());
            }
        }
        leases.swap_remove(index);
        Ok(())
    });
    if let Err(message) = result {
        set_pending_internal_error(message);
    }
}
