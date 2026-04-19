use crate::bytes_utils::{detect_endianness_header, read_integer_mut};
use crate::error::{JImageError, Result};
use crate::jimage::Endianness;

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
    major_version: u16,
    minor_version: u16,
    flags: u32,
    resource_count: u32,
    table_length: u32,
    location_bytes: u32,
    string_bytes: u32,
    table_length_in_bytes: usize,
    endianness: Endianness,
}

impl Header {
    const SIZE: usize = 28;
    pub(crate) fn from_bytes(raw_header: &[u8]) -> Result<Self> {
        const SUPPORTED_MAJOR_VER: u16 = 1;
        const SUPPORTED_MINOR_VER: u16 = 0;

        let endianness = detect_endianness_header(raw_header)?;

        let mut pos = 4usize;
        let version_pair: u32 = read_integer_mut(raw_header, &mut pos, &endianness)?;
        let major_version = (version_pair >> 16) as u16;
        let minor_version = (version_pair & 0xFFFF) as u16;
        if major_version != SUPPORTED_MAJOR_VER || minor_version != SUPPORTED_MINOR_VER {
            return Err(JImageError::Version {
                major_version,
                minor_version,
            });
        }

        let flags = read_integer_mut(raw_header, &mut pos, &endianness)?;
        let resource_count = read_integer_mut(raw_header, &mut pos, &endianness)?;
        let table_length = read_integer_mut(raw_header, &mut pos, &endianness)?;
        let location_bytes = read_integer_mut(raw_header, &mut pos, &endianness)?;
        let string_bytes = read_integer_mut(raw_header, &mut pos, &endianness)?;

        Ok(Self {
            major_version,
            minor_version,
            flags,
            resource_count,
            table_length,
            location_bytes,
            string_bytes,
            table_length_in_bytes: table_length as usize * 4,
            endianness,
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

    pub fn endianness(&self) -> &Endianness {
        &self.endianness
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::JImageError;

    /// Build a 28-byte raw header with the given fields, in big-endian byte order.
    fn build_header_bytes_be(
        major: u16,
        minor: u16,
        flags: u32,
        resource_count: u32,
        table_length: u32,
        location_bytes: u32,
        string_bytes: u32,
    ) -> Vec<u8> {
        let mut v = Vec::with_capacity(Header::SIZE);
        v.extend_from_slice(&0xCAFEDADAu32.to_be_bytes()); // magic
        let version_pair = ((major as u32) << 16) | (minor as u32);
        v.extend_from_slice(&version_pair.to_be_bytes());
        v.extend_from_slice(&flags.to_be_bytes());
        v.extend_from_slice(&resource_count.to_be_bytes());
        v.extend_from_slice(&table_length.to_be_bytes());
        v.extend_from_slice(&location_bytes.to_be_bytes());
        v.extend_from_slice(&string_bytes.to_be_bytes());
        v
    }

    fn build_header_bytes_le(
        major: u16,
        minor: u16,
        flags: u32,
        resource_count: u32,
        table_length: u32,
        location_bytes: u32,
        string_bytes: u32,
    ) -> Vec<u8> {
        let mut v = Vec::with_capacity(Header::SIZE);
        v.extend_from_slice(&0xCAFEDADAu32.to_le_bytes());
        let version_pair = ((major as u32) << 16) | (minor as u32);
        v.extend_from_slice(&version_pair.to_le_bytes());
        v.extend_from_slice(&flags.to_le_bytes());
        v.extend_from_slice(&resource_count.to_le_bytes());
        v.extend_from_slice(&table_length.to_le_bytes());
        v.extend_from_slice(&location_bytes.to_le_bytes());
        v.extend_from_slice(&string_bytes.to_le_bytes());
        v
    }

    #[test]
    fn from_bytes_parses_big_endian() {
        let bytes = build_header_bytes_be(1, 0, 7, 3, 4, 100, 200);
        let h = Header::from_bytes(&bytes).unwrap();
        assert_eq!(h.endianness(), &Endianness::Big);
        assert_eq!(h.items_count(), 4);
    }

    #[test]
    fn from_bytes_parses_little_endian() {
        let bytes = build_header_bytes_le(1, 0, 0, 0, 8, 50, 60);
        let h = Header::from_bytes(&bytes).unwrap();
        assert_eq!(h.endianness(), &Endianness::Little);
        assert_eq!(h.items_count(), 8);
    }

    #[test]
    fn from_bytes_rejects_unsupported_version() {
        let bytes = build_header_bytes_be(2, 0, 0, 0, 0, 0, 0);
        assert!(matches!(
            Header::from_bytes(&bytes),
            Err(JImageError::Version {
                major_version: 2,
                minor_version: 0
            })
        ));
        let bytes = build_header_bytes_be(1, 1, 0, 0, 0, 0, 0);
        assert!(matches!(
            Header::from_bytes(&bytes),
            Err(JImageError::Version {
                major_version: 1,
                minor_version: 1
            })
        ));
    }

    #[test]
    fn from_bytes_rejects_invalid_magic() {
        let mut bytes = build_header_bytes_be(1, 0, 0, 0, 0, 0, 0);
        bytes[0..4].copy_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);
        assert!(matches!(
            Header::from_bytes(&bytes),
            Err(JImageError::Magic {
                context: "header",
                ..
            })
        ));
    }

    #[test]
    fn position_helpers_compute_correct_offsets() {
        // table_length=2 (=> 8 bytes), location_bytes=10, string_bytes=20
        let bytes = build_header_bytes_be(1, 0, 0, 0, 2, 10, 20);
        let h = Header::from_bytes(&bytes).unwrap();
        // redirect begins at SIZE (28)
        assert_eq!(h.redirect(0), 28);
        assert_eq!(h.redirect(1), 28 + 4);
        // offset begins at 28 + table_length_in_bytes(=8) = 36
        assert_eq!(h.offset(0), 36);
        assert_eq!(h.offset(2), 36 + 8);
        // attributes begins at offset_begins_at + 8 = 44
        assert_eq!(h.attributes(0), 44);
        assert_eq!(h.attributes(5), 49);
        // strings begins at attributes_begins_at + location_bytes(=10) = 54
        assert_eq!(h.strings(0), 54);
        assert_eq!(h.strings(3), 57);
        // data begins at strings_begins_at + string_bytes(=20) = 74
        assert_eq!(h.data(0), 74);
        assert_eq!(h.data(100), 174);
    }

    #[test]
    fn from_bytes_fails_on_truncated_input() {
        let truncated: Vec<u8> = build_header_bytes_be(1, 0, 0, 0, 0, 0, 0)
            .into_iter()
            .take(10)
            .collect();
        assert!(Header::from_bytes(&truncated).is_err());
    }
}
