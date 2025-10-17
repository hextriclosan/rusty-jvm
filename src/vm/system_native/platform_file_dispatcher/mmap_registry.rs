use crate::vm::error::{Error, Result};
use dashmap::DashMap;
use memmap2::{Mmap, MmapAsRawDesc, MmapMut, MmapOptions};
use std::sync::LazyLock;

pub(super) enum MmapVariant {
    ReadOnly(Mmap),
    ReadWrite(MmapMut),
    Private(MmapMut),
}

static REGISTRY: LazyLock<DashMap<i64, MmapVariant>> = LazyLock::new(DashMap::default);

const MAP_RO: i32 = 0;
const MAP_RW: i32 = 1;
const MAP_PV: i32 = 2;
impl MmapVariant {
    pub(super) fn register<T: MmapAsRawDesc>(
        handle: T,
        map_mode: i32,
        offset: u64,
        length: usize,
    ) -> Result<i64> {
        let variant = Self::new(handle, map_mode, offset, length)?;
        let address = variant.as_ptr() as i64;
        REGISTRY.insert(address, variant);
        Ok(address)
    }

    pub(super) fn flush(address: i64, length: usize) -> Result<()> {
        let entry = REGISTRY
            .get(&address)
            .ok_or_else(|| Error::new_native("Invalid mmap address"))?;
        match &*entry {
            MmapVariant::ReadOnly(_) => Ok(()), // nothing to flush
            MmapVariant::ReadWrite(m) | MmapVariant::Private(m) => Ok(m.flush_range(0, length)?),
        }
    }

    fn new<T: MmapAsRawDesc>(
        handle: T,
        map_mode: i32,
        offset: u64,
        length: usize,
    ) -> Result<Self> {
        let mut options = MmapOptions::new();
        options.offset(offset).len(length);
        match map_mode {
            MAP_RO => {
                let mmap = unsafe { options.map(handle)? };
                Ok(MmapVariant::ReadOnly(mmap))
            }
            MAP_RW => {
                let mmap = unsafe { options.map_mut(handle)? };
                Ok(MmapVariant::ReadWrite(mmap))
            }
            MAP_PV => {
                let mmap = unsafe { options.map_copy(handle)? };
                Ok(MmapVariant::Private(mmap))
            }
            _ => Err(Error::new_native(&format!(
                "Invalid mapping mode: {map_mode}"
            ))),
        }
    }

    fn as_ptr(&self) -> *const u8 {
        match self {
            MmapVariant::ReadOnly(m) => m.as_ptr(),
            MmapVariant::ReadWrite(m) | MmapVariant::Private(m) => m.as_ptr(),
        }
    }
}
