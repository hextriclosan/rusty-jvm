use crate::error::{JImageError, Result};
use crate::jimage::Endianness;
use byteorder::ByteOrder;

pub(crate) fn read_integer<T: ReadFromBytes>(
    bytes: &[u8],
    offset: usize,
    endianness: &Endianness,
) -> Result<T> {
    read_integer_internal(bytes, offset, endianness, None)
}

pub(crate) fn read_integer_mut<T: ReadFromBytes>(
    bytes: &[u8],
    offset: &mut usize,
    endianness: &Endianness,
) -> Result<T> {
    read_integer_internal(bytes, *offset, endianness, Some(offset))
}

fn read_integer_internal<T: ReadFromBytes>(
    bytes: &[u8],
    from: usize,
    endianness: &Endianness,
    offset_out: Option<&mut usize>,
) -> Result<T> {
    let size = T::SIZE;
    let to = from + size;
    let slice = bytes
        .get(from..to)
        .ok_or(JImageError::RawRead { from, to })?;

    if let Some(offset) = offset_out {
        *offset = to;
    }

    T::read(slice, endianness)
}

pub(crate) trait ReadFromBytes: Sized {
    const SIZE: usize = std::mem::size_of::<Self>();
    fn read(bytes: &[u8], endianness: &Endianness) -> Result<Self>;
}

macro_rules! define_detect_endianness {
    ($fn_name:ident, $magic:expr, $label:expr) => {
        pub fn $fn_name(bytes: &[u8]) -> Result<Endianness> {
            use byteorder::{BigEndian, LittleEndian};
            const LEN: usize = 4;
            let magic_bytes = bytes
                .get(..LEN)
                .ok_or(JImageError::RawRead { from: 0, to: LEN })?;
            if LittleEndian::read_u32(magic_bytes) == $magic {
                Ok(Endianness::Little)
            } else if BigEndian::read_u32(magic_bytes) == $magic {
                Ok(Endianness::Big)
            } else {
                Err(JImageError::Magic {
                    magic: magic_bytes.try_into().unwrap_or([0; LEN]),
                    context: $label,
                })
            }
        }
    };
}
define_detect_endianness!(detect_endianness_header, 0xCAFEDADA, "header");
define_detect_endianness!(detect_endianness_resource, 0xCAFEFAFA, "resource");

macro_rules! impl_read_from_bytes {
    ($t:ty, $read_fn:ident, $size:expr) => {
        impl ReadFromBytes for $t {
            fn read(bytes: &[u8], endianness: &Endianness) -> Result<Self> {
                use byteorder::{BigEndian, LittleEndian};
                if bytes.len() < $size {
                    return Err(JImageError::Internal {
                        value: format!("not enough bytes for {}", stringify!($t)),
                    });
                }
                Ok(match endianness {
                    Endianness::Little => LittleEndian::$read_fn(bytes),
                    Endianness::Big => BigEndian::$read_fn(bytes),
                })
            }
        }
    };
}
impl_read_from_bytes!(i32, read_i32, 4);
impl_read_from_bytes!(u32, read_u32, 4);
impl_read_from_bytes!(u64, read_u64, 8);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::JImageError;

    #[test]
    fn read_integer_decodes_big_and_little_endian() {
        // 0x01020304 in BE
        let bytes = [0x01u8, 0x02, 0x03, 0x04];
        let v: u32 = read_integer(&bytes, 0, &Endianness::Big).unwrap();
        assert_eq!(v, 0x01020304);
        let v: u32 = read_integer(&bytes, 0, &Endianness::Little).unwrap();
        assert_eq!(v, 0x04030201);

        let v: i32 = read_integer(&[0xFF, 0xFF, 0xFF, 0xFF], 0, &Endianness::Big).unwrap();
        assert_eq!(v, -1);

        let bytes8 = [0x00u8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42];
        let v: u64 = read_integer(&bytes8, 0, &Endianness::Big).unwrap();
        assert_eq!(v, 0x42);
        let v: u64 = read_integer(&bytes8, 0, &Endianness::Little).unwrap();
        assert_eq!(v, 0x4200_0000_0000_0000);
    }

    #[test]
    fn read_integer_mut_advances_offset() {
        let bytes = [0x00u8, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
        let mut pos = 0;
        let a: u32 = read_integer_mut(&bytes, &mut pos, &Endianness::Big).unwrap();
        assert_eq!(a, 1);
        assert_eq!(pos, 4);
        let b: u32 = read_integer_mut(&bytes, &mut pos, &Endianness::Big).unwrap();
        assert_eq!(b, 2);
        assert_eq!(pos, 8);
    }

    #[test]
    fn read_integer_returns_raw_read_when_slice_too_small() {
        let bytes = [0x01u8, 0x02];
        let res: Result<u32> = read_integer(&bytes, 0, &Endianness::Big);
        assert!(matches!(res, Err(JImageError::RawRead { from: 0, to: 4 })));
    }

    #[test]
    fn detect_endianness_header_works() {
        // 0xCAFEDADA in big-endian byte order
        let be = [0xCAu8, 0xFE, 0xDA, 0xDA];
        assert_eq!(detect_endianness_header(&be).unwrap(), Endianness::Big);
        // little-endian byte order
        let le = [0xDAu8, 0xDA, 0xFE, 0xCA];
        assert_eq!(detect_endianness_header(&le).unwrap(), Endianness::Little);
        // invalid magic
        let bad = [0xDEu8, 0xAD, 0xBE, 0xEF];
        assert!(matches!(
            detect_endianness_header(&bad),
            Err(JImageError::Magic { magic, context }) if magic == bad && context == "header"
        ));
        // too short
        let short = [0x01u8];
        assert!(matches!(
            detect_endianness_header(&short),
            Err(JImageError::RawRead { from: 0, to: 4 })
        ));
    }

    #[test]
    fn detect_endianness_resource_works() {
        // 0xCAFEFAFA
        let be = [0xCAu8, 0xFE, 0xFA, 0xFA];
        assert_eq!(detect_endianness_resource(&be).unwrap(), Endianness::Big);
        let le = [0xFAu8, 0xFA, 0xFE, 0xCA];
        assert_eq!(detect_endianness_resource(&le).unwrap(), Endianness::Little);
        let bad = [0x00u8, 0x00, 0x00, 0x00];
        assert!(matches!(
            detect_endianness_resource(&bad),
            Err(JImageError::Magic { context, .. }) if context == "resource"
        ));
    }
}
