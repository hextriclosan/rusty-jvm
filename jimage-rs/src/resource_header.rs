use crate::bytes_utils::read_integer_mut;
use crate::error::{JImageError, Result};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ResourceHeader {
    resource_size: u64,
    uncompressed_size: u64,
    decompressor_name_offset: u32,
    decompressor_config_offset: u32,
    is_terminal: u8,

    flipped: bool,
}

impl ResourceHeader {
    pub const SIZE: usize = 29;

    pub(crate) fn from_bytes(raw_header: &[u8]) -> Result<Self> {
        const MAGIC: u32 = 0xCAFEFAFA;
        const FLIPPED_MAGIC: u32 = 0xFAFAFECA;
        let mut pos = 0usize;

        let flipped = match read_integer_mut(raw_header, &mut pos, false)? {
            MAGIC => false,
            FLIPPED_MAGIC => true,
            other => return Err(JImageError::Magic { magic: other }),
        };

        let size = read_integer_mut(raw_header, &mut pos, flipped)?;
        let uncompressed_size = read_integer_mut(raw_header, &mut pos, flipped)?;
        let decompressor_name_offset = read_integer_mut(raw_header, &mut pos, flipped)?;
        let decompressor_config_offset = read_integer_mut(raw_header, &mut pos, flipped)?;
        let is_terminal = raw_header[pos];

        Ok(Self {
            resource_size: size,
            uncompressed_size,
            decompressor_name_offset,
            decompressor_config_offset,
            is_terminal,
            flipped,
        })
    }

    pub fn resource_size(&self) -> u64 {
        self.resource_size
    }

    pub fn uncompressed_size(&self) -> u64 {
        self.uncompressed_size
    }
}
