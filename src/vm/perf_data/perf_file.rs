use crate::vm::error::{Error, Result};
use crate::vm::perf_data::perf_file::DataTypes::{Byte, Long};
use crate::vm::perf_data::perf_file::Flags::{FNone, FSupported};
use crate::vm::perf_data::perf_file::Units::UString;
use crate::vm::perf_data::perf_file::Variability::VConstant;
use memmap2::MmapMut;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::fs::OpenOptions;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::{env, fs};

static PERFDATA_CAPACITY: Lazy<usize> = Lazy::new(|| {
    let base = 32 * 1024; // 32 KB
    let page_size = page_size::get();
    if base % page_size == 0 {
        base
    } else {
        base + page_size - (base % page_size) // round up to page size
    }
});
#[cfg(target_endian = "little")]
const PERFDATA_MAGIC: i32 = 0xc0c0fecau32 as i32;
#[cfg(target_endian = "big")]
const PERFDATA_MAGIC: i32 = 0xcafec0c0u32 as i32;

const PERFDATA_PROLOGUE_SIZE: usize = 32;
const PERFDATA_ENTRY_HEADER_SIZE: usize = 20;

const PERFDATA_MAJOR_VERSION: u8 = 2;
const PERFDATA_MINOR_VERSION: u8 = 0;

#[cfg(target_endian = "little")]
const PERFDATA_BYTE_ORDER: u8 = 1;
#[cfg(target_endian = "big")]
const PERFDATA_BYTE_ORDER: u8 = 0;

const PERFDATA_MAX_STRING_LEN: usize = 1024;

struct PerfEntry {
    name: String,
    data_type: u8,
    flags: u8,
    units: u8,
    variability: u8,
    data: Vec<u8>,
    vector_length: i32,
}

impl PerfEntry {
    pub fn to_bytes(&self) -> (Vec<u8>, usize, usize) {
        let name_offset = PERFDATA_ENTRY_HEADER_SIZE as i32;
        let full_name_len = self.name.len() + 1; // includes null terminator
        let name_end = PERFDATA_ENTRY_HEADER_SIZE + full_name_len;
        let data_offset = align_up(name_end, 8);
        let data_end = data_offset + self.data.len();
        let entry_length = align_up(data_end, 8);

        let mut buf = Vec::with_capacity(entry_length);

        buf.extend_from_slice(&(entry_length as i32).to_ne_bytes());
        buf.extend_from_slice(&name_offset.to_ne_bytes());
        buf.extend_from_slice(&self.vector_length.to_ne_bytes());
        buf.push(self.data_type);
        buf.push(self.flags);
        buf.push(self.units);
        buf.push(self.variability);
        buf.extend_from_slice(&(data_offset as i32).to_ne_bytes());

        debug_assert_eq!(buf.len(), PERFDATA_ENTRY_HEADER_SIZE);

        // name (null-terminated)
        buf.extend_from_slice(self.name.as_bytes());
        buf.push(0u8); // null terminator

        // padding between name and data
        buf.resize(data_offset, 0u8);

        // data
        buf.extend_from_slice(&self.data);

        // padding to entry_length
        buf.resize(entry_length, 0u8);

        (buf, data_offset, data_end)
    }
}

#[repr(u8)]
enum Flags {
    FNone = 0x0,
    FSupported = 0x1, // interface is supported - java.* and com.sun.*
}

#[repr(u8)]
#[allow(dead_code)]
enum DataTypes {
    Byte = b'B',
    Boolean = b'Z',
    Long = b'J',
    Int = b'I',
    Short = b'S',
    Char = b'C',
    Double = b'D',
    Float = b'F',
    Void = b'V',
    Reference = b'L',
    Array = b'[',
}

#[repr(u8)]
#[allow(dead_code)]
pub enum Units {
    UNone = 1,
    UBytes = 2,
    UTicks = 3,
    UEvents = 4,
    UString = 5,
    UHertz = 6,
}

#[repr(u8)]
#[allow(dead_code)]
pub enum Variability {
    VConstant = 1,
    VMonotonic = 2,
    VVariable = 3,
}

pub(super) struct PerfFile {
    mmap: Option<MmapMut>,
    path: PathBuf,
    names: HashSet<String>,
}

impl Drop for PerfFile {
    fn drop(&mut self) {
        // Flush and unmap before removing the file (important on Windows,
        // where you cannot delete a file that has an open mapping).
        drop(self.mmap.take());
        let _ = fs::remove_file(&self.path);
    }
}

impl PerfFile {
    pub(crate) fn default() -> Result<Self> {
        let pid = std::process::id();
        let perf_dir = get_hsperfdata_dir()?;

        fs::create_dir_all(&perf_dir)?;

        let file_path = perf_dir.join(pid.to_string());

        // Create / truncate the file and extend it to PERFDATA_CAPACITY bytes so
        // that the JDK's native Perf.attach() can mmap it (it requires the file
        // size to be a non-zero multiple of the OS page size).
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file_path)?;
        file.set_len(*PERFDATA_CAPACITY as u64)?;

        #[cfg(unix)]
        file.set_permissions(fs::Permissions::from_mode(0o600))?; // rw------- permissions for security

        // SAFETY: We just created this file and hold the only file descriptor.
        // No other process should be mapping it yet.
        let mut mmap = unsafe { MmapMut::map_mut(&file)? };

        let prologue_bytes = make_prologue_bytes()?;
        mmap[..prologue_bytes.len()].copy_from_slice(&prologue_bytes);
        mmap.flush()?;

        Ok(Self {
            mmap: Some(mmap),
            path: file_path,
            names: HashSet::new(),
        })
    }

    pub(super) fn create_string(&mut self, name: &str, value: &str) -> Result<(*const u8, usize)> {
        self.create_byte_array(
            name,
            VConstant as u8,
            UString as u8,
            value.as_bytes(),
            PERFDATA_MAX_STRING_LEN,
        )
    }

    pub(crate) fn create_long(
        &mut self,
        name: &str,
        variability: u8,
        units: u8,
        value: i64,
    ) -> Result<(*const u8, usize)> {
        let entry = make_long_entry(name, value, units, variability);
        self.append_entry_to_mmap(entry)
    }

    pub(crate) fn create_byte_array(
        &mut self,
        name: &str,
        variability: u8,
        units: u8,
        value: &[u8],
        max_len: usize,
    ) -> Result<(*const u8, usize)> {
        let entry = make_byte_array_entry(name, value, units, variability, max_len);
        self.append_entry_to_mmap(entry)
    }

    fn append_entry_to_mmap(&mut self, entry: PerfEntry) -> Result<(*const u8, usize)> {
        let Some(mmap) = self.mmap.as_mut() else {
            return Err(Error::new_execution("Perf file mmap not available"));
        };
        let used = i32::from_ne_bytes(mmap[8..12].try_into()?) as usize;
        let new_num_entries = i32::from_ne_bytes(mmap[28..32].try_into()?) + 1;

        let (entry_bytes, data_offset, data_end) = entry.to_bytes();
        let new_used = used + entry_bytes.len();

        let data_offset = data_offset + used;
        let data_end = data_end + used;

        if new_used > *PERFDATA_CAPACITY {
            // OpenJDK sets overflow to 1 if the file is full and continue working,
            // we just fall here for the sake of simplicity.
            mmap[12..16]
                .copy_from_slice(&(new_used as i32 - *PERFDATA_CAPACITY as i32).to_ne_bytes()); // set overflow
            return Err(Error::new_execution(
                "Not enough space in perf data file for new entry",
            ));
        }

        mmap[used..new_used].copy_from_slice(&entry_bytes);
        mmap[8..12].copy_from_slice(&(new_used as i32).to_ne_bytes());
        mmap[16..24].copy_from_slice(&time_stamp_ns()?.to_ne_bytes());
        mmap[28..32].copy_from_slice(&(new_num_entries).to_ne_bytes());
        mmap.flush()?;

        self.names.insert(entry.name);
        Ok((mmap[data_offset..data_end].as_ptr(), data_end - data_offset))
    }

    pub(crate) fn contains_name(&self, name: &str) -> bool {
        self.names.contains(name)
    }
}

fn get_hsperfdata_dir() -> Result<PathBuf> {
    let tmp_dir = env::temp_dir();
    let username = resolve_username()?;
    let safe_username: String = username
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let full_path = tmp_dir.join(format!("hsperfdata_{}", safe_username));
    Ok(full_path)
}

fn make_prologue_bytes() -> Result<Vec<u8>> {
    let num_entries = 0i32; // no entries yet
    let actual_bytes_in_use = PERFDATA_PROLOGUE_SIZE as i32; // prologue only, no entries yet
    let entry_offset = PERFDATA_PROLOGUE_SIZE as i32; // entries start immediately after prologue
    let time_stamp = time_stamp_ns()?;
    let accessible = 1u8; // 1 means data is valid
    let overflow = 0i32; // no overflow

    let mut buf = Vec::with_capacity(PERFDATA_PROLOGUE_SIZE);
    buf.extend_from_slice(&PERFDATA_MAGIC.to_ne_bytes());
    buf.push(PERFDATA_BYTE_ORDER);
    buf.push(PERFDATA_MAJOR_VERSION);
    buf.push(PERFDATA_MINOR_VERSION);
    buf.push(accessible);
    buf.extend_from_slice(&actual_bytes_in_use.to_ne_bytes());
    buf.extend_from_slice(&overflow.to_ne_bytes());
    buf.extend_from_slice(&time_stamp.to_ne_bytes());
    buf.extend_from_slice(&entry_offset.to_ne_bytes());
    buf.extend_from_slice(&num_entries.to_ne_bytes());

    debug_assert_eq!(buf.len(), PERFDATA_PROLOGUE_SIZE);

    Ok(buf)
}

fn make_long_entry(name: &str, value: i64, units: u8, variability: u8) -> PerfEntry {
    let flags = resolve_flags(name);

    PerfEntry {
        name: name.to_string(),
        data_type: Long as u8,
        flags,
        units,
        variability,
        vector_length: 0,
        data: value.to_ne_bytes().to_vec(),
    }
}

fn make_byte_array_entry(
    name: &str,
    value: &[u8],
    units: u8,
    variability: u8,
    max_len: usize,
) -> PerfEntry {
    // Reserve at least 1 byte so there is always space for the null terminator.
    let data_len = max_len.max(1);
    let mut data = vec![0u8; data_len];
    // Copy value, leaving at least one zero byte at the end.
    let copy_len = value.len().min(data_len.saturating_sub(1));
    data[..copy_len].copy_from_slice(&value[..copy_len]);

    let flags = resolve_flags(name);
    PerfEntry {
        name: name.to_string(),
        data_type: Byte as u8,
        flags,
        units,
        variability,
        vector_length: data_len as i32,
        data,
    }
}

fn align_up(val: usize, align: usize) -> usize {
    (val + align - 1) & !(align - 1)
}

fn resolve_flags(name: &str) -> u8 {
    (if name.starts_with("java.") || name.starts_with("com.sun.") {
        FSupported
    } else {
        FNone
    }) as u8
}

fn time_stamp_ns() -> Result<i64> {
    Ok(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos() as i64)
}

fn resolve_username() -> Result<String> {
    if let Ok(username) = whoami::username() {
        return Ok(username);
    }

    // Fallbacks
    let username = env::var("USER").or_else(|_| env::var("LOGNAME"))?;
    Ok(username)
}
