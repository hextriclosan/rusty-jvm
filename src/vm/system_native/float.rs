use crate::vm::error::Result;

/// `java.lang.Float.floatToRawIntBits(F)I`
pub(crate) fn float_to_raw_int_bits(value: f32) -> Result<i32> {
    Ok(value.to_bits() as i32)
}
