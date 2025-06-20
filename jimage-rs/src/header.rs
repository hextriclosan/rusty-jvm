use crate::bytes_utils::read_integer_mut;
use crate::error::{JImageError, Result};

/* JImage File Header Structure

    +-------------------------+
    |   Magic (0xCAFEDADA)    |
    +------------+------------+
    | Major Vers | Minor Vers |
    +------------+------------+
    |          Flags          |
    +-------------------------+
    |      Resource Count     |
    +-------------------------+
    |       Table Length      |
    +-------------------------+
    |      Attributes Size    |
    +-------------------------+
    |       Strings Size      |
    +-------------------------+

*/

/// Represents the header of a jimage file.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Header {
    magic: u32,
    major_version: u16,
    minor_version: u16,
    flags: u32,
    resource_count: u32,
    table_length: u32,
    location_bytes: u32,
    string_bytes: u32,
    table_length_in_bytes: usize,
}

impl Header {
    const SIZE: usize = 28;
    pub(crate) fn from_bytes(raw_header: &[u8]) -> Result<Self> {
        const MAGIC: u32 = 0xCAFEDADA;
        const SUPPORTED_MAJOR_VER: u16 = 1;
        const SUPPORTED_MINOR_VER: u16 = 0;

        let mut pos = 0usize;

        let magic = read_integer_mut(raw_header, &mut pos)?;
        if magic != MAGIC {
            return Err(JImageError::Magic(magic));
        }

        let version_pair: u32 = read_integer_mut(raw_header, &mut pos)?;
        let major_version = (version_pair >> 16) as u16;
        let minor_version = (version_pair & 0xFFFF) as u16;
        if major_version != SUPPORTED_MAJOR_VER || minor_version != SUPPORTED_MINOR_VER {
            return Err(JImageError::Version {
                major_version,
                minor_version,
            });
        }

        let flags = read_integer_mut(raw_header, &mut pos)?;
        let resource_count = read_integer_mut(raw_header, &mut pos)?;
        let table_length = read_integer_mut(raw_header, &mut pos)?;
        let location_bytes = read_integer_mut(raw_header, &mut pos)?;
        let string_bytes = read_integer_mut(raw_header, &mut pos)?;

        Ok(Self {
            magic,
            major_version,
            minor_version,
            flags,
            resource_count,
            table_length,
            location_bytes,
            string_bytes,
            table_length_in_bytes: table_length as usize * 4,
        })
    }

    fn redirect_begins_at() -> usize {
        Self::SIZE
    }

    fn offset_begins_at(&self) -> usize {
        Self::redirect_begins_at() + self.table_length_in_bytes
    }

    fn attributes_begins_at(&self) -> usize {
        self.offset_begins_at() + self.table_length_in_bytes
    }

    fn strings_begins_at(&self) -> usize {
        self.attributes_begins_at() + self.location_bytes as usize
    }

    fn data_begins_at(&self) -> usize {
        self.strings_begins_at() + self.string_bytes as usize
    }

    pub fn items_count(&self) -> u32 {
        self.table_length
    }

    pub fn redirect(&self, pos_item: usize) -> usize {
        Self::redirect_begins_at() + pos_item * 4
    }

    pub fn offset(&self, pos_item: usize) -> usize {
        self.offset_begins_at() + pos_item * 4
    }

    pub fn attributes(&self, pos_byte: usize) -> usize {
        self.attributes_begins_at() + pos_byte
    }

    pub fn strings(&self, pos_byte: usize) -> usize {
        self.strings_begins_at() + pos_byte
    }

    pub fn data(&self, pos_byte: usize) -> usize {
        self.data_begins_at() + pos_byte
    }
}
