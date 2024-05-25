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
