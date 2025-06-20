use crate::error::{JImageError, Result};

pub(crate) fn read_integer<T: ReadFromBytes>(bytes: &[u8], offset: usize) -> Result<T> {
    let from = offset;
    let to = from + 4;
    let arr: [u8; 4] = bytes[from..to]
        .try_into()
        .map_err(|_| JImageError::RawRead { from, to })?;
    Ok(T::read(arr))
}

pub(crate) fn read_integer_mut<T: ReadFromBytes>(bytes: &[u8], offset: &mut usize) -> Result<T> {
    let from = *offset;
    let to = from + 4;
    let arr: [u8; 4] = bytes[from..to]
        .try_into()
        .map_err(|_| JImageError::RawRead { from, to })?;
    *offset = to;
    Ok(T::read(arr))
}

pub(crate) trait ReadFromBytes: Sized {
    fn read(array: [u8; 4]) -> Self;
}

impl ReadFromBytes for i32 {
    fn read(array: [u8; 4]) -> Self {
        Self::from_ne_bytes(array)
    }
}

impl ReadFromBytes for u32 {
    fn read(array: [u8; 4]) -> Self {
        Self::from_ne_bytes(array)
    }
}
