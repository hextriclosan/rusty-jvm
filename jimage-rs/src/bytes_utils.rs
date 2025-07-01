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
