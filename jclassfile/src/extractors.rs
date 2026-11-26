use crate::error::ErrorKind::Io;
use crate::error::{Error, Result};
use cesu8::from_java_cesu8;
use num_traits::Num;
use std::array::TryFromSliceError;
use std::io;
use std::io::ErrorKind::InvalidData;
use std::mem::size_of;
use std::result::Result as StdResult;

pub fn get_int<T>(slice: &[u8], start_from: &mut usize) -> Result<T>
where
    T: Num + From<u8> + std::ops::Shl<Output = T> + std::ops::BitOr<Output = T> + Copy,
{
    let size = size_of::<T>();
    let sub_slice = read_byte_block(slice, *start_from, size)?;

    let mut value: T = T::from(sub_slice[0]);
    for &byte in &sub_slice[1..] {
        value = (value << 8.into()) | T::from(byte);
    }
    *start_from += size;

    Ok(value)
}

pub fn get_bytes<'a>(slice: &'a [u8], start_from: &mut usize, size: usize) -> Result<&'a [u8]> {
    read_byte_block(slice, *start_from, size).map(|sub_slice| {
        *start_from += size;

        sub_slice
    })
}

pub fn read_byte_block(slice: &[u8], start_from: usize, size: usize) -> Result<&[u8]> {
    start_from
        .checked_add(size)
        .and_then(|end| slice.get(start_from..end))
        .ok_or_else(|| Error::truncated(start_from, size, slice.len()))
}

pub fn get_bitfield<T>(slice: &[u8], start_from: &mut usize) -> Result<T>
where
    T: bitflags::Flags<Bits = u16>,
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

pub fn get_string(data: &&[u8], mut start_from: &mut usize) -> Result<String> {
    let length: u16 = get_int(&data, &mut start_from)?;
    let offset = *start_from;
    let mutf8_bytes: &[u8] = get_bytes(&data, &mut start_from, length as usize)?;

    if !is_modified_utf8(mutf8_bytes) {
        return Err(Error::format(format!(
            "Utf8 constant at offset {offset} is not valid modified UTF-8 (JVMS §4.4.7)"
        )));
    }

    Ok(from_java_cesu8(mutf8_bytes)
        .map(|c| c.into_owned())
        .unwrap_or_else(|_| from_java_cesu8_lossy(mutf8_bytes)))
}

/// Tells whether a byte slice is well-formed modified UTF-8 (JVMS §4.4.7): sequences of one, two
/// or three bytes, with no byte of value 0 and no byte in the range 0xF0 to 0xFF.
/// A surrogate encoded on its own is well-formed here, although it does not decode to a `char`.
fn is_modified_utf8(bytes: &[u8]) -> bool {
    let mut i = 0;

    while i < bytes.len() {
        i += match &bytes[i..] {
            [0x01..=0x7F, ..] => 1,
            [0xC0..=0xDF, 0x80..=0xBF, ..] => 2,
            [0xE0..=0xEF, 0x80..=0xBF, 0x80..=0xBF, ..] => 3,
            _ => return false,
        };
    }

    true
}

/// Decodes a byte slice encoded in Java's CESU-8 format into a String, replacing invalid sequences with '?'.
fn from_java_cesu8_lossy(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        match &bytes[i..] {
            // 1. 6-byte Surrogate Pair (CESU-8 Supplementary Character)
            [0xED, b2 @ 0xA0..=0xAF, b3 @ 0x80..=0xBF, 0xED, b5 @ 0xB0..=0xBF, b6 @ 0x80..=0xBF, ..] =>
            {
                let u1 = ((0xED as u32 & 0x0F) << 12)
                    | ((*b2 as u32 & 0x3F) << 6)
                    | (*b3 as u32 & 0x3F);
                let u2 = ((0xED as u32 & 0x0F) << 12)
                    | ((*b5 as u32 & 0x3F) << 6)
                    | (*b6 as u32 & 0x3F);
                let codepoint = 0x10000 + ((u1 - 0xD800) << 10) + (u2 - 0xDC00);
                out.push(char::from_u32(codepoint).unwrap_or('?'));
                i += 6;
            }

            // 2. 3-byte sequence (Normal BMP character)
            [b1 @ 0xE0..=0xEF, b2 @ 0x80..=0xBF, b3 @ 0x80..=0xBF, ..] => {
                let u =
                    ((*b1 as u32 & 0x0F) << 12) | ((*b2 as u32 & 0x3F) << 6) | (*b3 as u32 & 0x3F);
                out.push(char::from_u32(u).unwrap_or('?'));
                i += 3;
            }

            // 3. Truncated 3-byte sequence (Matches 2 valid bytes but 3rd is missing)
            [0xE0..=0xEF, 0x80..=0xBF, ..] => {
                out.push('?');
                i += 2;
            }

            // 4. 2-byte sequence (Modified UTF-8 / CESU-8)
            [b1 @ 0xC0..=0xDF, b2 @ 0x80..=0xBF, ..] => {
                let u = ((*b1 as u32 & 0x1F) << 6) | (*b2 as u32 & 0x3F);
                out.push(char::from_u32(u).unwrap_or('?'));
                i += 2;
            }

            // 5. 1-byte ASCII (0x00 - 0x7F)
            [b @ 0x00..=0x7F, ..] => {
                out.push(*b as char);
                i += 1;
            }

            // 6. Catch-all for invalid lead bytes or truncated 2-byte sequences
            [_, ..] => {
                out.push('?');
                i += 1;
            }

            [] => break,
        }
    }
    out
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

    #[test]
    fn test_ascii() {
        let input = b"Hello, World!";
        assert_eq!(from_java_cesu8_lossy(input), "Hello, World!");
    }

    #[test]
    fn test_java_null_byte() {
        // Java Modified UTF-8 encodes U+0000 as 0xC0 0x80
        let input = [0xC0, 0x80];
        assert_eq!(from_java_cesu8_lossy(&input), "\0");

        // Literal null should also work (standard ASCII)
        assert_eq!(from_java_cesu8_lossy(&[0x00]), "\0");
    }

    #[test]
    fn test_bmp_multibyte() {
        assert_eq!(from_java_cesu8_lossy(&[0xD1, 0x97]), "ї");
        assert_eq!(from_java_cesu8_lossy(&[0xE2, 0x82, 0xAC]), "€");
    }

    #[test]
    fn test_cesu8_emojis() {
        // Crab 🦀 (U+1F980)
        let crab_bytes = [0xED, 0xA0, 0xBE, 0xED, 0xB6, 0x80];
        assert_eq!(from_java_cesu8_lossy(&crab_bytes), "🦀");

        // Yo-yo 🪀 (U+1FA80)
        let yoyo_bytes = [0xED, 0xA0, 0xBE, 0xED, 0xBA, 0x80];
        assert_eq!(from_java_cesu8_lossy(&yoyo_bytes), "🪀");
    }

    #[test]
    fn test_unpaired_surrogates_lossy() {
        // Lone high surrogate (ED A0 BE)
        assert_eq!(from_java_cesu8_lossy(&[0xED, 0xA0, 0xBE]), "?");

        // Lone low surrogate (ED BA 80)
        assert_eq!(from_java_cesu8_lossy(&[0xED, 0xBA, 0x80]), "?");

        // High surrogate followed by something else
        let input = [0xED, 0xA0, 0xBE, 0x41]; // High surrogate + 'A'
        assert_eq!(from_java_cesu8_lossy(&input), "?A");
    }

    #[test]
    fn test_truncated_sequences() {
        // Truncated 2-byte sequence
        assert_eq!(from_java_cesu8_lossy(&[0xC2]), "?");

        // Truncated 3-byte sequence
        assert_eq!(from_java_cesu8_lossy(&[0xE2, 0x82]), "?");

        // Truncated surrogate pair (only high surrogate)
        assert_eq!(from_java_cesu8_lossy(&[0xED, 0xA0, 0xBE, 0xED]), "??");
    }

    #[test]
    fn test_invalid_bytes() {
        // 0xFF and 0xFE are never valid in UTF-8/CESU-8
        assert_eq!(from_java_cesu8_lossy(&[0xFF, 0xFE]), "??");

        // Invalid continuation byte
        assert_eq!(from_java_cesu8_lossy(&[0xC2, 0x31]), "?1");
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(from_java_cesu8_lossy(&[]), "");
    }

    #[test]
    fn test_well_formed_modified_utf8() {
        assert!(is_modified_utf8(&[]));
        assert!(is_modified_utf8(b"Hello, World!"));
        // U+0000 in its two-byte form
        assert!(is_modified_utf8(&[0xC0, 0x80]));
        assert!(is_modified_utf8(&[0xD1, 0x97]));
        assert!(is_modified_utf8(&[0xE2, 0x82, 0xAC]));
        // Supplementary character as a surrogate pair
        assert!(is_modified_utf8(&[0xED, 0xA0, 0xBE, 0xED, 0xB6, 0x80]));
        // Lone surrogate: not a `char`, but a valid three-byte sequence
        assert!(is_modified_utf8(&[0xED, 0xA0, 0xBE]));
    }

    #[test]
    fn test_malformed_modified_utf8() {
        // No byte may have the value 0
        assert!(!is_modified_utf8(&[0x41, 0x00]));
        // No byte may lie in the range 0xF0 to 0xFF, four-byte UTF-8 included
        assert!(!is_modified_utf8(&[0xF0, 0x9F, 0xA6, 0x80]));
        assert!(!is_modified_utf8(&[0xFF]));
        // Continuation byte with nothing to continue
        assert!(!is_modified_utf8(&[0x80]));
        // Truncated two-byte and three-byte sequences
        assert!(!is_modified_utf8(&[0xC2]));
        assert!(!is_modified_utf8(&[0xE2, 0x82]));
        // Lead byte followed by something that is not a continuation byte
        assert!(!is_modified_utf8(&[0xC2, 0x31]));
    }

    #[test]
    fn test_get_string_rejects_malformed_modified_utf8() {
        let data: &[u8] = &[0x00, 0x02, 0x41, 0xFF];
        let mut start_from: usize = 0;

        let error = get_string(&data, &mut start_from).unwrap_err();

        assert_eq!(
            error.to_string(),
            "Format Error: Utf8 constant at offset 2 is not valid modified UTF-8 (JVMS §4.4.7)"
        );
    }

    #[test]
    fn test_read_past_the_end_is_truncation() {
        let mut start_from: usize = 1;

        let error = get_int::<u16>(&[0x00, 0x01], &mut start_from).unwrap_err();

        assert!(error.is_truncated());
        assert_eq!(start_from, 1);
    }

    #[test]
    fn test_complex_mix() {
        let mut input = Vec::new();
        input.push(0x41); // "A" (ASCII)
        input.extend_from_slice(&[0xC0, 0x80]); // NULL
        input.extend_from_slice(&[0xD1, 0x97]); // "ї"

        // bytes for Crab (🦀) (U+1F980):
        // High Surrogate (U+D83E): ED A0 BE
        // Low Surrogate  (U+DD80): ED B6 80
        input.extend_from_slice(&[0xED, 0xA0, 0xBE, 0xED, 0xB6, 0x80]);

        input.push(0xFF); // Invalid

        assert_eq!(from_java_cesu8_lossy(&input), "A\0ї🦀?");
    }
}
