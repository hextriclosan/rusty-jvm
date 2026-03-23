use crate::vm::error::Result;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{i32toi64, i64_to_vec};
use crate::vm::perf_data;
use crate::vm::system_native::string::get_utf8_string_by_ref;

// --- Perf units constants (mirroring jdk.internal.perf.Perf) ---
const U_STRING: u8 = 5;

pub(crate) fn perf_register_natives_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![])
}

/// `createLong(String name, int variability, int units, long value) : ByteBuffer`
///
/// args layout (instance method):
///   [0] this
///   [1] name ref  (java.lang.String)
///   [2] variability (int)
///   [3] units (int)
///   [4] value high 32 bits
///   [5] value low  32 bits
///
/// Returns 0 (null ByteBuffer) — we don't need the writable handle because
/// the counter value is written once during creation and never updated.
pub(crate) fn perf_create_long_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let name_ref = args[1];
    let variability = args[2] as u8;
    let units = args[3] as u8;
    let value = i32toi64(args[4], args[5]);

    if let Ok(name) = get_utf8_string_by_ref(name_ref) {
        perf_data::create_long(&name, variability, units, value);
    } else {
        tracing::warn!("perf_create_long: failed to read name string ref={name_ref}");
    }

    // Return null (0) for the ByteBuffer — callers that need a writable
    // handle will fail gracefully with a NullPointerException, which is
    // acceptable since rusty-jvm does not need in-place counter updates.
    Ok(vec![0])
}

/// `createByteArray(String name, int variability, int units, byte[] value, int maxLength) : ByteBuffer`
///
/// args layout (instance method):
///   [0] this
///   [1] name ref        (java.lang.String)
///   [2] variability     (int)
///   [3] units           (int)
///   [4] value array ref (byte[])
///   [5] max_length      (int)
///
/// Returns 0 (null ByteBuffer).
pub(crate) fn perf_create_byte_array_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let name_ref = args[1];
    let variability = args[2] as u8;
    let units = args[3] as u8;
    let value_array_ref = args[4];
    let max_len = args[5] as usize;

    if let Ok(name) = get_utf8_string_by_ref(name_ref) {
        let bytes: Vec<u8> = if value_array_ref != 0 {
            HEAP.get_entire_raw_data(value_array_ref)
                .map(|data| data.clone())
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        perf_data::create_byte_array(&name, variability, units, &bytes, max_len);
    } else {
        tracing::warn!("perf_create_byte_array: failed to read name string ref={name_ref}");
    }

    Ok(vec![0])
}

/// `createString(String name, int variability, int units, String value) : ByteBuffer`
///
/// Non-native wrapper that calls createByteArray internally; we provide it as
/// a convenience so that classes calling createString directly also work.
///
/// args layout (instance method):
///   [0] this
///   [1] name ref        (java.lang.String)
///   [2] variability     (int)
///   [3] units           (int — always U_STRING)
///   [4] value ref       (java.lang.String)
///
/// Returns 0 (null ByteBuffer).
pub(crate) fn perf_create_string_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let name_ref = args[1];
    let variability = args[2] as u8;
    let value_ref = args[4];

    if let (Ok(name), Ok(value)) = (
        get_utf8_string_by_ref(name_ref),
        get_utf8_string_by_ref(value_ref),
    ) {
        let bytes = value.as_bytes();
        let max_len = bytes.len().max(32) + 1; // +1 for null terminator
        perf_data::create_byte_array(&name, variability, U_STRING, bytes, max_len);
    }

    Ok(vec![0])
}

/// `highResCounter() : long` — returns the current nanosecond timestamp.
pub(crate) fn perf_high_res_counter_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    let now = std::time::SystemTime::now();
    let nanos = now
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0);
    Ok(i64_to_vec(nanos))
}

/// `highResFrequency() : long` — nanoseconds per tick (always 1 ns/tick = 1 GHz).
pub(crate) fn perf_high_res_frequency_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(1_000_000_000i64))
}

/// `attach0(int pid) : ByteBuffer` — attaching to another JVM's perf data is
/// not supported; return null so that callers receive an IOException.
pub(crate) fn perf_attach0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![0])
}

/// `detach(ByteBuffer bb) : void` — no-op (we never hand out real ByteBuffers).
pub(crate) fn perf_detach_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![])
}
