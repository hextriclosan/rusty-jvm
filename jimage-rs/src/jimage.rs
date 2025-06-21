use crate::bytes_utils::read_integer;
use crate::error::{JImageError, Result};
use crate::header::Header;
use memchr::memchr;
use memmap2::Mmap;
use std::borrow::Cow;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/* JImage File Structure

    /------------------------------\
    |          Header              | (Fixed size: 28 bytes)
    |------------------------------|
    |       Index Tables:          |
    |  - Redirect Table            | (table_length * 4 bytes)
    |  - Offsets Table             | (table_length * 4 bytes)
    |  - Location Attributes Table | locations_bytes
    |------------------------------|
    |         String Table         | strings_bytes
    |------------------------------|
    |                              |
    |       Resource Data Blob     |
    |                              |
    \------------------------------/

*/

/// Represents a Java Image (JImage) file, which contains resources used by the Java Virtual Machine (JVM).
#[derive(Debug)]
pub struct JImage {
    mmap: Mmap,
    header: Header,
}

/// Represents the kinds of attributes that can be associated with resources in a JImage file.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AttributeKind {
    END,
    MODULE,
    PARENT,
    BASE,
    EXTENSION,
    OFFSET,
    COMPRESSED,
    UNCOMPRESSED,
    COUNT,
}

impl TryFrom<u8> for AttributeKind {
    type Error = JImageError;

    fn try_from(value: u8) -> Result<Self> {
        if value >= AttributeKind::COUNT as u8 {
            Err(JImageError::Internal(format!(
                "Invalid attribute kind: {}",
                value
            )))
        } else {
            unsafe { Ok(std::mem::transmute(value)) }
        }
    }
}

const HASH_MULTIPLIER: u32 = 0x01000193;

impl JImage {
    /// Creates a new `JImage` instance by mapping the specified file into memory.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        let header = Header::from_bytes(&mmap)?;

        Ok(Self { mmap, header })
    }

    /// Finds a resource by name and returns its data.
    pub fn find_resource(&self, name: &str) -> Result<Option<Cow<[u8]>>> {
        // Find offset index using the hash
        let Some(offset_index) = self.find_offset_index(name)? else {
            return Ok(None);
        };

        // Get the attributes for the location index.
        let attribute_index = self.offset_value(offset_index)?;
        let attribute = self.attributes(attribute_index)?;

        // Verify the full name matches the path reconstructed from attributes.
        if !self.verify(&attribute, name)? {
            return Ok(None); // Hash collision, the name doesn't actually match.
        }

        self.get_resource(&attribute)
    }

    /// Finds the offset index for a given resource name using a hash function.
    fn find_offset_index(&self, name: &str) -> Result<Option<i32>> {
        let items_count = self.header.items_count() as i32;
        let hash = Self::hash_code(name, HASH_MULTIPLIER as i32)?;
        let redirect_index = hash % items_count;
        let redirected_val = self.redirect_value(redirect_index)?;

        match redirected_val {
            val if val < 0 => Ok(Some(-1 - val)),
            val if val > 0 => Ok(Some(Self::hash_code(name, val)? % items_count)),
            _ => Ok(None),
        }
    }

    /// Computes a hash code for a given string using a seed value.
    fn hash_code(string: &str, seed: i32) -> Result<i32> {
        let mut current_hash  = seed as u32;
        for &byte in string.as_bytes() {
            current_hash = current_hash.overflowing_mul(HASH_MULTIPLIER).0 ^ byte as u32;
        }
        Ok((current_hash & 0x7FFFFFFF) as i32)
    }

    fn redirect_value(&self, index: i32) -> Result<i32> {
        let offset = self.header.redirect(index as usize);
        read_integer(&self.mmap, offset)
    }

    fn offset_value(&self, index: i32) -> Result<i32> {
        let offset = self.header.offset(index as usize);
        read_integer(&self.mmap, offset)
    }

    fn get_string(&self, index: usize) -> Result<&str> {
        let offset = self.header.strings(index);
        let string_slice = &self.mmap[offset..];
        let len = memchr(0, string_slice).ok_or(JImageError::Internal(format!(
            "Failed to find null-terminator in string starting from {offset}"
        )))?;
        let value = std::str::from_utf8(&self.mmap[offset..offset + len])?;

        Ok(value)
    }

    fn attributes(&self, index: i32) -> Result<[u64; 8]> {
        let offset = self.header.attributes(index as usize);

        let mut attributes = [0u64; 8];
        let mut pos = offset;
        loop {
            let value = &self.mmap[pos];

            let kind = value >> 3;
            let kind = AttributeKind::try_from(kind)?;
            if kind == AttributeKind::END {
                break;
            }

            let len = (value & 0b0000_0111) + 1;
            let value = self.get_attribute_value(pos + 1, len)?;
            pos += 1 + len as usize;

            attributes[kind as usize] = value;
        }

        Ok(attributes)
    }

    fn get_resource(&self, attributes: &[u64; 8]) -> Result<Option<Cow<[u8]>>> {
        let offset = attributes[AttributeKind::OFFSET as usize] as usize;
        let compressed_size = attributes[AttributeKind::COMPRESSED as usize] as usize;
        let uncompressed_size = attributes[AttributeKind::UNCOMPRESSED as usize] as usize;

        let start = self.header.data(offset);
        if compressed_size == 0 {
            Ok(Some(Cow::Borrowed(
                &self.mmap[start..start + uncompressed_size],
            )))
        } else {
            let compressed_data = &self.mmap[start..start + compressed_size];
            let mut zlib_decoder = flate2::read::ZlibDecoder::new(compressed_data);
            let mut uncompressed_data = vec![0u8; uncompressed_size];
            zlib_decoder.read_exact(&mut uncompressed_data)?;

            Ok(Some(Cow::Owned(uncompressed_data)))
        }
    }

    /// Verify the attributes of the resource.
    /// Full path format: /{module}/{parent}/{base}.{extension}
    fn verify(&self, attributes: &[u64; 8], full_name: &str) -> Result<bool> {
        let parts_to_check = [
            (AttributeKind::MODULE, "/"),
            (AttributeKind::PARENT, "/"),
            (AttributeKind::BASE, "/"),
            (AttributeKind::EXTENSION, "."),
        ];

        let mut remaining_name = full_name;
        for (kind, prefix) in &parts_to_check {
            let offset = attributes[*kind as usize] as usize;
            let part = self.get_string(offset)?;

            if !part.is_empty() {
                remaining_name = if let Some(stripped) = remaining_name.strip_prefix(prefix) {
                    stripped
                } else {
                    return Ok(false);
                };

                remaining_name = if let Some(stripped) = remaining_name.strip_prefix(part) {
                    stripped
                } else {
                    return Ok(false);
                };
            }
        }

        Ok(remaining_name.is_empty())
    }

    fn get_attribute_value(&self, pos: usize, len: u8) -> Result<u64> {
        if !(1..=8).contains(&len) {
            return Err(JImageError::Internal(format!(
                "Invalid attribute length: {len}"
            )));
        }

        let mut value = 0u64;
        for i in 0..len as usize {
            value <<= 8;
            value |= self.mmap[i + pos] as u64;
        }

        Ok(value)
    }
}
