use std::io;
use std::io::ErrorKind::InvalidInput;
use std::mem::size_of;
use num_traits::Num;

pub fn read_int<T>(slice: &[u8], start_from: &mut usize) -> Result<T, io::Error>
    where
        T: Num + From<u8> + std::ops::Shl<Output=T> + std::ops::BitOr<Output=T> + Copy, {
    let size = size_of::<T>();
    let sub_slice = slice.get(*start_from..*start_from + size)
        .ok_or_else(|| io::Error::new(
            InvalidInput,
            format!("overflow : attempt to read from {} whereas len is {}", *start_from, slice.len())))?;


    let mut value: T = T::from(sub_slice[0]);
    for &byte in &sub_slice[1..] {
        value = (value << 8.into()) | T::from(byte);
    }
    *start_from += size;

    Ok(value)
}

pub fn read_bytes<'a>(slice: &'a [u8], start_from: &mut usize, size: usize) -> Result<&'a [u8], io::Error> {
    slice.get(*start_from..*start_from + size)
        .ok_or_else(|| io::Error::new(
            InvalidInput,
            format!("Index out of bounds: {} of {}", *start_from + size, slice.len())))
        .map(|sub_slice| {
            *start_from += size;

            sub_slice
        })
}

pub fn read_bitfield<T>(slice: &[u8], start_from: &mut usize) -> Result<T, io::Error>
    where
        T: bitflags::Flags<Bits=u16> {
    let bits = read_int(slice, start_from)?;

    Ok(T::from_bits_truncate(bits))
}

#[cfg(test)]
mod tests {
    use bitflags::bitflags;
    use super::*;

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
        let result = read_bitfield::<Flags>(&data, &mut start_from);

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
        let result = read_bitfield::<Flags>(&data, &mut start_from);

        match result {
            Ok(flags) => {
                assert_eq!(flags, Flags::FLAG1 | Flags::FLAG2);
            }
            _ => panic!("Expected Ok, got {:?}", result),
        }
    }
}
