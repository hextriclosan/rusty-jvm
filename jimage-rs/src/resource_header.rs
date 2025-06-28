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
