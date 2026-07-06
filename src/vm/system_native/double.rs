use crate::vm::error::Result;

/// `java.lang.Double.doubleToRawLongBits(D)J`
pub(crate) fn double_to_raw_long_bits(value: f64) -> Result<i64> {
    Ok(value.to_bits() as i64)
}

/// `java.lang.Double.longBitsToDouble(J)D`
pub(crate) fn long_bits_to_double(value: i64) -> Result<f64> {
    Ok(f64::from_bits(value as u64))
}
