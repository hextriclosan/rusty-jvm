//! Thread-local measurement of real Rust allocator calls, not a mock heap.
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

struct ObservedSystem;

#[global_allocator]
static ALLOCATOR: ObservedSystem = ObservedSystem;

thread_local! {
    static OBSERVATION: Cell<Option<(usize, usize)>> = const { Cell::new(None) };
    static BYTES: Cell<Option<usize>> = const { Cell::new(None) };
}

fn admitted(size: usize) -> bool {
    BYTES
        .try_with(|bytes| {
            if let Some(total) = bytes.get() {
                bytes.set(Some(total.saturating_add(size)));
            }
        })
        .ok();
    OBSERVATION
        .try_with(|state| {
            if let Some((count, limit)) = state.get() {
                state.set(Some((count.saturating_add(1), limit)));
                size <= limit
            } else {
                true
            }
        })
        .unwrap_or(true)
}

/// Actual allocator request bytes. For collector preflight all four allocations
/// coexist until sweep finishes, so their sum is its scratch high-water storage.
/// With other allocations this is an upper bound, not a retained-memory counter.
pub(crate) fn measure_bytes<T>(operation: impl FnOnce() -> T) -> (T, usize) {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            BYTES.with(|bytes| bytes.set(None));
        }
    }
    BYTES.with(|bytes| assert!(bytes.replace(Some(0)).is_none()));
    let reset = Reset;
    let result = operation();
    let bytes = BYTES.with(|bytes| bytes.get().unwrap());
    drop(reset);
    (result, bytes)
}

// SAFETY: all ownership, alignment and layout contracts are forwarded unchanged to
// System. Instrumentation uses only nonallocating thread-local Cells and cannot unwind.
unsafe impl GlobalAlloc for ObservedSystem {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if admitted(layout.size()) {
            // SAFETY: GlobalAlloc's caller supplied a valid allocation layout.
            unsafe { System.alloc(layout) }
        } else {
            std::ptr::null_mut()
        }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if admitted(layout.size()) {
            // SAFETY: the caller's layout is passed unchanged to the same allocator.
            unsafe { System.alloc_zeroed(layout) }
        } else {
            std::ptr::null_mut()
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if admitted(size) {
            // SAFETY: ptr/layout came from this System-backed allocator; the caller
            // guarantees the new size satisfies GlobalAlloc::realloc's contract.
            unsafe { System.realloc(ptr, layout, size) }
        } else {
            std::ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: the caller returns a live System allocation with its original layout.
        unsafe { System.dealloc(ptr, layout) }
    }
}

/// Count alloc/alloc_zeroed/realloc. Reject requests above limit so a broken
/// oversized-dimension test cannot actually allocate gigabytes.
pub(crate) fn measure<T>(limit: usize, operation: impl FnOnce() -> T) -> (T, usize) {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            OBSERVATION.with(|state| state.set(None));
        }
    }
    OBSERVATION.with(|state| {
        assert!(
            state.get().is_none(),
            "allocation observations must not nest"
        );
        state.set(Some((0, limit)));
    });
    let reset = Reset;
    let result = operation();
    let count = OBSERVATION.with(|state| state.get().unwrap().0);
    drop(reset);
    (result, count)
}

/// Minimal real JDK bootstrap, with no Java constructor or native callback.
pub(crate) fn bootstrap_oom() -> i32 {
    use crate::vm::method_area::{loaded_classes::CLASSES, method_area::MethodArea};
    let java_home = std::env::var_os("JAVA_HOME").expect("focused tests require JAVA_HOME");
    crate::vm::JAVA_HOME.set(java_home.into()).unwrap();
    MethodArea::init().unwrap();
    CLASSES.pre_construct().unwrap();
    CLASSES.post_construct().unwrap();
    crate::vm::jni::handles::initialize().unwrap();
    crate::vm::jni::handles::emergency_oom_ref().unwrap()
}
