use crate::error::{JImageError, Result};

pub(crate) fn read_integer<T: ReadFromBytes>(
    bytes: &[u8],
    offset: usize,
    flipped: bool,
) -> Result<T> {
    read_integer_internal(bytes, offset, flipped, None)
}

pub(crate) fn read_integer_mut<T: ReadFromBytes>(
    bytes: &[u8],
    offset: &mut usize,
    flipped: bool,
) -> Result<T> {
    read_integer_internal(bytes, *offset, flipped, Some(offset))
}

fn read_integer_internal<T: ReadFromBytes>(
    bytes: &[u8],
    from: usize,
    flipped: bool,
    offset_out: Option<&mut usize>,
) -> Result<T> {
    let size = T::SIZE;
    let to = from + size;
    let slice = bytes
        .get(from..to)
        .ok_or(JImageError::RawRead { from, to })?;

    let mut buf = [0u8; 8];
    buf[..size].copy_from_slice(slice);
    if flipped {
        buf[..size].reverse();
    }

    if let Some(offset) = offset_out {
        *offset = to;
    }

    T::read(&buf[..size])
}

pub(crate) trait ReadFromBytes: Sized {
    const SIZE: usize = size_of::<Self>();
    fn read(bytes: &[u8]) -> Result<Self>;
}

impl ReadFromBytes for i32 {
    fn read(bytes: &[u8]) -> Result<Self> {
        let arr: [u8; Self::SIZE] = bytes.try_into()?;
        Ok(i32::from_ne_bytes(arr))
    }
}

impl ReadFromBytes for u32 {
    fn read(bytes: &[u8]) -> Result<Self> {
        let arr: [u8; Self::SIZE] = bytes.try_into()?;
        Ok(u32::from_ne_bytes(arr))
    }
}

impl ReadFromBytes for u64 {
    fn read(bytes: &[u8]) -> Result<Self> {
        let arr: [u8; Self::SIZE] = bytes.try_into()?;
        Ok(u64::from_ne_bytes(arr))
    }
}
