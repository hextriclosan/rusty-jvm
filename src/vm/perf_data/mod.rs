mod perf_file;

use crate::Arguments;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use perf_file::PerfFile;

/// Global perf file instance.  Protected by a `Mutex` so that native
/// method calls from multiple Java threads can safely append new entries.
/// The `Option` wrapper lets us explicitly drop (and therefore delete) the
/// file at JVM shutdown while keeping the static alive.
static PERF_FILE: Lazy<Mutex<Option<PerfFile>>> = Lazy::new(|| Mutex::new(None));

/// RAII guard returned by `create_perf_file`.  Dropping this guard removes
/// the `PerfFile` from the global, which triggers `PerfFile::drop` —
/// flushing and unmapping the mmap, then deleting the file.
pub(crate) struct PerfFileGuard;

impl Drop for PerfFileGuard {
    fn drop(&mut self) {
        drop(PERF_FILE.lock().take());
    }
}

/// Creates the hsperfdata perf file, stores it in the global, and returns a
/// `PerfFileGuard` that will delete the file when dropped.
pub(crate) fn create_perf_file(arguments: &Arguments) -> Option<PerfFileGuard> {
    match perf_file::try_create_perf_file(arguments) {
        Ok(pf) => {
            *PERF_FILE.lock() = Some(pf);
            Some(PerfFileGuard)
        }
        Err(e) => {
            tracing::warn!("Failed to create perf file: {e}");
            None
        }
    }
}

/// Appends a new long counter to the live perf file.  No-op if the file was
/// not successfully created or has already been closed.
pub(crate) fn create_long(name: &str, variability: u8, units: u8, value: i64) {
    if let Some(pf) = PERF_FILE.lock().as_mut() {
        pf.create_long(name, variability, units, value);
    }
}

/// Appends a new byte-array counter to the live perf file.  No-op if the
/// file was not successfully created or has already been closed.
pub(crate) fn create_byte_array(
    name: &str,
    variability: u8,
    units: u8,
    value: &[u8],
    max_len: usize,
) {
    if let Some(pf) = PERF_FILE.lock().as_mut() {
        pf.create_byte_array(name, variability, units, value, max_len);
    }
}
