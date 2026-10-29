use crate::vm::error::Result;
use crate::vm::system_native::system::epoch_nanos;
use std::time::SystemTime;

const NANOS_PER_SECOND: i128 = 1_000_000_000;

/// Furthest the wall clock may be from the caller's offset, in seconds, for
/// `getNanoTimeAdjustment` to report the adjustment instead of -1, as in HotSpot.
const MAX_DIFF_SECONDS: i128 = 1 << 32;

/// `jdk.internal.misc.VM.initialize()V`
pub(crate) fn initialize() -> Result<()> {
    // todo: implement me
    Ok(())
}

/// `jdk.internal.misc.VM.getNanoTimeAdjustment(J)J`
///
/// The wall clock in nanoseconds relative to `offset_in_seconds` since the epoch, or -1 if the
/// two are too far apart for the result to fit. `java.time.Clock.systemUTC()` (and so
/// `Instant.now()`) reads the time through it.
pub(crate) fn get_nano_time_adjustment(offset_in_seconds: i64) -> Result<i64> {
    Ok(nano_time_adjustment(
        epoch_nanos(SystemTime::now()),
        offset_in_seconds,
    ))
}

fn nano_time_adjustment(epoch_nanos: i128, offset_in_seconds: i64) -> i64 {
    let seconds = epoch_nanos.div_euclid(NANOS_PER_SECOND);
    let nanos = epoch_nanos.rem_euclid(NANOS_PER_SECOND);
    let diff = seconds - offset_in_seconds as i128;
    if diff >= MAX_DIFF_SECONDS || diff <= -MAX_DIFF_SECONDS {
        return -1;
    }
    (diff * NANOS_PER_SECOND + nanos) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: i128 = 1_792_403_275 * NANOS_PER_SECOND + 558_000_123;

    #[test]
    fn nano_time_adjustment_is_relative_to_the_offset() {
        assert_eq!(nano_time_adjustment(NOW, 1_792_403_275), 558_000_123);
        assert_eq!(
            nano_time_adjustment(NOW, 1_792_403_265),
            10 * 1_000_000_000 + 558_000_123
        );
        assert_eq!(
            nano_time_adjustment(NOW, 1_792_403_285),
            -10 * 1_000_000_000 + 558_000_123
        );
    }

    #[test]
    fn nano_time_adjustment_handles_times_before_the_epoch() {
        assert_eq!(nano_time_adjustment(-1_500_000_000, 0), -1_500_000_000);
        assert_eq!(nano_time_adjustment(-1_500_000_000, -2), 500_000_000);
    }

    #[test]
    fn nano_time_adjustment_is_minus_one_when_the_offset_is_too_far() {
        let seconds = (NOW / NANOS_PER_SECOND) as i64;
        let max = MAX_DIFF_SECONDS as i64;
        assert_eq!(nano_time_adjustment(NOW, seconds - max), -1);
        assert_eq!(nano_time_adjustment(NOW, seconds + max), -1);
        assert_ne!(nano_time_adjustment(NOW, seconds - max + 1), -1);
        assert_ne!(nano_time_adjustment(NOW, seconds + max - 1), -1);
    }
}
