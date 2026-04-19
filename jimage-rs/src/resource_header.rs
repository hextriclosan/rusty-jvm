use crate::bytes_utils::{detect_endianness_resource, read_integer_mut};
use crate::error::Result;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ResourceHeader {
    compressed_size: u64,
    uncompressed_size: u64,
    decompressor_name_offset: u32,
    decompressor_config_offset: u32,
    is_terminal: u8,
}

impl ResourceHeader {
    pub const SIZE: usize = 29;

    pub(crate) fn from_bytes(raw_header: &[u8]) -> Result<Self> {
        let endianness = detect_endianness_resource(raw_header)?;

        let mut pos = 4usize;
        let compressed_size = read_integer_mut(raw_header, &mut pos, &endianness)?;
        let uncompressed_size = read_integer_mut(raw_header, &mut pos, &endianness)?;
        let decompressor_name_offset = read_integer_mut(raw_header, &mut pos, &endianness)?;
        let decompressor_config_offset = read_integer_mut(raw_header, &mut pos, &endianness)?;
        let is_terminal = raw_header[pos];

        Ok(Self {
            compressed_size,
            uncompressed_size,
            decompressor_name_offset,
            decompressor_config_offset,
            is_terminal,
        })
    }

    pub fn compressed_size(&self) -> u64 {
        self.compressed_size
    }

    pub fn uncompressed_size(&self) -> u64 {
        self.uncompressed_size
    }

    pub fn decompressor_name_offset(&self) -> u32 {
        self.decompressor_name_offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::JImageError;

    fn build_resource_header(
        endianness: crate::jimage::Endianness,
        compressed_size: u64,
        uncompressed_size: u64,
        decompressor_name_offset: u32,
        decompressor_config_offset: u32,
        is_terminal: u8,
    ) -> Vec<u8> {
        let mut v = Vec::with_capacity(ResourceHeader::SIZE);
        match endianness {
            crate::jimage::Endianness::Big => {
                v.extend_from_slice(&0xCAFEFAFAu32.to_be_bytes());
                v.extend_from_slice(&compressed_size.to_be_bytes());
                v.extend_from_slice(&uncompressed_size.to_be_bytes());
                v.extend_from_slice(&decompressor_name_offset.to_be_bytes());
                v.extend_from_slice(&decompressor_config_offset.to_be_bytes());
            }
            crate::jimage::Endianness::Little => {
                v.extend_from_slice(&0xCAFEFAFAu32.to_le_bytes());
                v.extend_from_slice(&compressed_size.to_le_bytes());
                v.extend_from_slice(&uncompressed_size.to_le_bytes());
                v.extend_from_slice(&decompressor_name_offset.to_le_bytes());
                v.extend_from_slice(&decompressor_config_offset.to_le_bytes());
            }
        }
        v.push(is_terminal);
        v
    }

    #[test]
    fn parses_big_endian_resource_header() {
        let bytes = build_resource_header(crate::jimage::Endianness::Big, 123, 456, 7, 8, 1);
        let h = ResourceHeader::from_bytes(&bytes).unwrap();
        assert_eq!(h.compressed_size(), 123);
        assert_eq!(h.uncompressed_size(), 456);
        assert_eq!(h.decompressor_name_offset(), 7);
        assert_eq!(h.is_terminal, 1);
    }

    #[test]
    fn parses_little_endian_resource_header() {
        let bytes = build_resource_header(crate::jimage::Endianness::Little, 10, 20, 30, 40, 0);
        let h = ResourceHeader::from_bytes(&bytes).unwrap();
        assert_eq!(h.compressed_size(), 10);
        assert_eq!(h.uncompressed_size(), 20);
        assert_eq!(h.decompressor_name_offset(), 30);
        assert_eq!(h.is_terminal, 0);
    }

    #[test]
    fn rejects_invalid_magic() {
        let mut bytes = build_resource_header(crate::jimage::Endianness::Big, 0, 0, 0, 0, 0);
        bytes[0..4].copy_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);
        assert!(matches!(
            ResourceHeader::from_bytes(&bytes),
            Err(JImageError::Magic {
                context: "resource",
                ..
            })
        ));
    }
}
