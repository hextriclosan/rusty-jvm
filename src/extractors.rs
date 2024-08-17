use num_traits::Num;
use std::array::TryFromSliceError;
use std::io::ErrorKind::{InvalidData, InvalidInput};
use std::mem::size_of;
use std::io;
use cesu8::from_java_cesu8;
use crate::error::{Error};
use crate::error::ErrorKind::Io;
use crate::error::Result;
use std::result::Result as StdResult;

pub fn get_int<T>(slice: &[u8], start_from: &mut usize) -> Result<T>
    where
        T: Num + From<u8> + std::ops::Shl<Output=T> + std::ops::BitOr<Output=T> + Copy,
{
    let size = size_of::<T>();
    let sub_slice = slice.get(*start_from..*start_from + size).ok_or_else(|| {
        Error::new_io(
            InvalidInput,
            format!(
                "overflow : attempt to read from {} whereas len is {}",
                *start_from,
                slice.len()
            ).as_str())
    })?;

    let mut value: T = T::from(sub_slice[0]);
    for &byte in &sub_slice[1..] {
        value = (value << 8.into()) | T::from(byte);
    }
    *start_from += size;

    Ok(value)
}

pub fn get_bytes<'a>(
    slice: &'a [u8],
    start_from: &mut usize,
    size: usize,
) -> Result<&'a [u8]> {
    slice
        .get(*start_from..*start_from + size)
        .ok_or_else(|| {
            Error::new_io(
                InvalidInput,
                format!(
                    "Index out of bounds: {} of {}",
                    *start_from + size,
                    slice.len()
                ).as_str())
        })
        .map(|sub_slice| {
            *start_from += size;

            sub_slice
        })
}

pub fn get_bitfield<T>(slice: &[u8], start_from: &mut usize) -> Result<T>
    where
        T: bitflags::Flags<Bits=u16>,
{
    let bits = get_int(slice, start_from)?;

    Ok(T::from_bits_truncate(bits))
}

pub trait FromBeBytes: Sized {
    fn from_be_bytes(bytes: &[u8]) -> StdResult<Self, TryFromSliceError>;
}

impl FromBeBytes for f32 {
    fn from_be_bytes(bytes: &[u8]) -> StdResult<Self, TryFromSliceError> {
        Ok(f32::from_be_bytes(bytes.try_into()?))
    }
}

impl FromBeBytes for f64 {
    fn from_be_bytes(bytes: &[u8]) -> StdResult<Self, TryFromSliceError> {
        Ok(f64::from_be_bytes(bytes.try_into()?))
    }
}

pub fn get_float<T>(data: &[u8], mut start_from: &mut usize) -> Result<T>
    where
        T: FromBeBytes + Sized,
{
    let size = size_of::<T>();
    let bytes = get_bytes(&data, &mut start_from, size)?;

    T::from_be_bytes(bytes).map_err(|e| Error::new(Io(io::Error::new(InvalidData, e))))
}

pub fn get_string(
    data: &&[u8],
    mut start_from: &mut usize,
) -> Result<String> {
    let length: u16 = get_int(&data, &mut start_from)?;
    let mutf8_bytes: &[u8] = get_bytes(&data, &mut start_from, length as usize)?;

    Ok(from_java_cesu8(mutf8_bytes)
        .map_err(|e| Error::new(Io(io::Error::new(InvalidData, e))))?
        .into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitflags::bitflags;

    bitflags! {
    #[derive(Debug, PartialEq)]
        pub struct Flags: u16 {
            const FLAG1 = 0x01;
            const FLAG2 = 0x02;
        }
    }

    #[test]
    fn test_ignore_unmatched_bit() {
        let mut start_from: usize = 0;
        let binding = vec![0b0000_0000, 0b0000_0111];
        let data = binding.as_slice();
        let result = get_bitfield::<Flags>(&data, &mut start_from);

        match result {
            Ok(flags) => {
                assert_eq!(flags, Flags::FLAG1 | Flags::FLAG2);
            }
            _ => panic!("Expected Ok, got {:?}", result),
        }
    }

    #[test]
    fn test_all_matched_bits() {
        let mut start_from: usize = 0;
        let binding = vec![0b0000_0000, 0b0000_0011];
        let data = binding.as_slice();
        let result = get_bitfield::<Flags>(&data, &mut start_from);

        match result {
            Ok(flags) => {
                assert_eq!(flags, Flags::FLAG1 | Flags::FLAG2);
            }
            _ => panic!("Expected Ok, got {:?}", result),
        }
    }
}
