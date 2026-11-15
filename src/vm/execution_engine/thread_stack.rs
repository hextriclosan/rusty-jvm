//! The size of a thread's Java stack, as `-Xss` sets it, and the frames that fill it (JVMS
//! §2.5.2). Frames live on the heap here, not on the OS thread's stack, so the size is a budget:
//! each frame is charged what HotSpot's interpreter would lay out for it, a fixed overhead plus a
//! word per local and per operand stack slot, and a call whose frame would take the budget past
//! its end raises `StackOverflowError`, so that runaway recursion ends the way it does on HotSpot
//! instead of in the VM eating memory, and at a depth that responds to `-Xss` the same way.

use crate::vm::error::{Error, Result};
use crate::vm::jni::java_thread::{JavaThread, StackOverflowHandling};
use std::sync::OnceLock;

/// HotSpot's default `ThreadStackSize` on 64-bit Linux.
pub(crate) const DEFAULT_STACK_SIZE: usize = 1024 * 1024;

/// HotSpot's interpreted frame on a 64-bit platform, less its locals and operand stack: the
/// return address and saved frame pointer, the method, the bytecode pointer, the constant pool
/// cache, the pointers to the locals and to the sender's stack pointer, the monitor block, and
/// the like.
const FRAME_OVERHEAD_WORDS: usize = 10;
const WORD: usize = 8;

static STACK_SIZE: OnceLock<usize> = OnceLock::new();

/// Sets the stack size from the last `-Xss` among `jvm_options`, or to the default without one.
/// The value is spelled as for HotSpot: a number of bytes, or with a `k`, `m` or `g` suffix in
/// either case. A size too small for the VM's own startup fails that startup with a
/// `StackOverflowError`, as it does on HotSpot.
pub(crate) fn init_stack_size(jvm_options: &[String]) -> Result<()> {
    let mut size = DEFAULT_STACK_SIZE;
    for option in jvm_options {
        if let Some(value) = option.strip_prefix("-Xss") {
            size = parse_size(value).ok_or_else(|| {
                Error::new_execution(&format!("Invalid thread stack size: {option}"))
            })?;
        }
    }
    STACK_SIZE
        .set(size)
        .map_err(|_| Error::new_execution("Thread stack size already set"))
}

/// The size of every thread's Java stack.
pub(crate) fn stack_size() -> usize {
    STACK_SIZE.get().copied().unwrap_or(DEFAULT_STACK_SIZE)
}

/// What a frame with `locals` locals and `operand_stack` operand stack slots takes of the stack.
pub(crate) fn frame_bytes(locals: usize, operand_stack: usize) -> usize {
    (FRAME_OVERHEAD_WORDS + locals + operand_stack) * WORD
}

/// Whether a frame of `frame_bytes` would take the current thread's stack past its size, so that
/// the call has to raise `StackOverflowError` instead of pushing it. The guard it returns then
/// suspends the limit for as long as it lives: raising the error takes frames of its own, which
/// must not overflow in turn. `None` as well while the limit is already suspended.
pub(crate) fn exhausted(frame_bytes: usize) -> Option<StackOverflowHandling> {
    if JavaThread::stack_used() + frame_bytes <= stack_size() {
        return None;
    }
    JavaThread::begin_stack_overflow_handling()
}

/// `-Xss`'s value in bytes: `512k`, `1m`, `1048576`, ... `None` for anything else, zero included.
fn parse_size(value: &str) -> Option<usize> {
    let (digits, multiplier) = match value.chars().last()? {
        'k' | 'K' => (&value[..value.len() - 1], 1024),
        'm' | 'M' => (&value[..value.len() - 1], 1024 * 1024),
        'g' | 'G' => (&value[..value.len() - 1], 1024 * 1024 * 1024),
        _ => (value, 1),
    };
    let number: usize = digits.parse().ok()?;
    number.checked_mul(multiplier).filter(|&size| size > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_sizes_as_hotspot_spells_them() {
        assert_eq!(parse_size("1048576"), Some(1024 * 1024));
        assert_eq!(parse_size("512k"), Some(512 * 1024));
        assert_eq!(parse_size("512K"), Some(512 * 1024));
        assert_eq!(parse_size("2m"), Some(2 * 1024 * 1024));
        assert_eq!(parse_size("1G"), Some(1024 * 1024 * 1024));
    }

    #[test]
    fn should_reject_what_is_not_a_size() {
        assert_eq!(parse_size(""), None);
        assert_eq!(parse_size("0"), None);
        assert_eq!(parse_size("abc"), None);
        assert_eq!(parse_size("1x"), None);
        assert_eq!(parse_size("-1m"), None);
        assert_eq!(parse_size("99999999999999999999g"), None);
    }

    #[test]
    fn should_charge_a_frame_for_its_slots_and_overhead() {
        assert_eq!(frame_bytes(0, 0), FRAME_OVERHEAD_WORDS * WORD);
        assert_eq!(frame_bytes(3, 2), (FRAME_OVERHEAD_WORDS + 5) * WORD);
    }
}
