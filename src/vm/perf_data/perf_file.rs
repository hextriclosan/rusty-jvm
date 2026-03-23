use crate::Arguments;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs, io};

// PerfData format constants
// Magic bytes are always {0xca, 0xfe, 0xc0, 0xc0} in the file regardless of endianness.
// On little-endian: store i32 value 0xc0c0feca (LE bytes: ca fe c0 c0)
// On big-endian:    store i32 value 0xcafec0c0 (BE bytes: ca fe c0 c0)
#[cfg(target_endian = "little")]
const PERFDATA_MAGIC: i32 = 0xc0c0fecau32 as i32;
#[cfg(target_endian = "big")]
const PERFDATA_MAGIC: i32 = 0xcafec0c0u32 as i32;

const PERFDATA_MAJOR_VERSION: u8 = 2;
const PERFDATA_MINOR_VERSION: u8 = 0;

#[cfg(target_endian = "little")]
const PERFDATA_BYTE_ORDER: u8 = 1;
#[cfg(target_endian = "big")]
const PERFDATA_BYTE_ORDER: u8 = 0;

const PERFDATA_PROLOGUE_SIZE: usize = 32;
const PERFDATA_ENTRY_HEADER_SIZE: usize = 20;

// The perf file must be a multiple of the OS page size.
// OpenJDK's native Perf.attach() checks: size >= page_size && size % page_size == 0.
// We use 64 KB (OpenJDK's default PerfDataMemorySize) as the file capacity.
// The `used` field in the prologue records how many bytes are actually in use.
const PERFDATA_CAPACITY: usize = 64 * 1024;

// Units
const U_TICKS: u8 = 3;
const U_STRING: u8 = 5;
const U_HERTZ: u8 = 6;

// Variability
const V_CONSTANT: u8 = 1;

// Flags
const F_NONE: u8 = 0x0;
const F_SUPPORTED: u8 = 0x1; // java.* and com.sun.*

// Data types
const T_BYTE: u8 = b'B'; // for string arrays
const T_LONG: u8 = b'J'; // for long values

const PERF_DIR_PREFIX: &str = "hsperfdata";
const PERFDATA_MAX_STRING_LEN: usize = 1024;

pub(crate) fn create_perf_file(arguments: &Arguments) {
    if let Err(e) = try_create_perf_file(arguments) {
        tracing::warn!("Failed to create perf file: {e}");
    }
}

fn try_create_perf_file(arguments: &Arguments) -> io::Result<()> {
    let pid = std::process::id();
    let perf_dir = get_hsperfdata_dir();

    fs::create_dir_all(&perf_dir)?;

    let file_path = perf_dir.join(pid.to_string());
    let data = build_perf_data(arguments);
    fs::write(&file_path, &data)?;

    Ok(())
}

fn get_hsperfdata_dir() -> std::path::PathBuf {
    let tmp_dir = env::temp_dir();
    let username = get_username();
    let safe_username: String = username
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
        .collect();
    tmp_dir.join(format!("{}_{}", PERF_DIR_PREFIX, safe_username))
}

fn get_username() -> String {
    #[cfg(not(target_os = "windows"))]
    {
        env::var("USER")
            .or_else(|_| env::var("LOGNAME"))
            .unwrap_or_else(|_| "user".to_string())
    }
    #[cfg(target_os = "windows")]
    {
        env::var("USERNAME").unwrap_or_else(|_| "user".to_string())
    }
}

struct PerfEntry {
    name: String,
    data_type: u8,
    flags: u8,
    units: u8,
    variability: u8,
    data: Vec<u8>,
    vector_length: i32,
}

fn make_string_entry(name: &str, value: &str, variability: u8) -> PerfEntry {
    let value_bytes = value.as_bytes();
    let data_len = if value_bytes.is_empty() {
        1
    } else {
        value_bytes.len().min(PERFDATA_MAX_STRING_LEN) + 1
    };

    let mut data = vec![0u8; data_len];
    let copy_len = value_bytes.len().min(PERFDATA_MAX_STRING_LEN);
    data[..copy_len].copy_from_slice(&value_bytes[..copy_len]);

    let flags = if name.starts_with("java.") || name.starts_with("com.sun.") {
        F_SUPPORTED
    } else {
        F_NONE
    };

    PerfEntry {
        name: name.to_string(),
        data_type: T_BYTE,
        flags,
        units: U_STRING,
        variability,
        vector_length: data_len as i32,
        data,
    }
}

fn make_long_entry(name: &str, value: i64, units: u8, variability: u8) -> PerfEntry {
    let flags = if name.starts_with("java.") || name.starts_with("com.sun.") {
        F_SUPPORTED
    } else {
        F_NONE
    };

    PerfEntry {
        name: name.to_string(),
        data_type: T_LONG,
        flags,
        units,
        variability,
        vector_length: 0,
        data: value.to_ne_bytes().to_vec(),
    }
}

fn build_perf_data(arguments: &Arguments) -> Vec<u8> {
    let mut entries: Vec<PerfEntry> = Vec::new();

    let java_command = {
        let mut cmd = arguments.entry_point().clone();
        for arg in arguments.program_args() {
            cmd.push(' ');
            cmd.push_str(arg);
        }
        cmd
    };
    entries.push(make_string_entry("sun.rt.javaCommand", &java_command, V_CONSTANT));

    let vm_args = arguments.jvm_options().join(" ");
    entries.push(make_string_entry("sun.rt.vmArgs", &vm_args, V_CONSTANT));

    let vm_flags = arguments.advanced_jvm_options().join(" ");
    entries.push(make_string_entry("sun.rt.vmFlags", &vm_flags, V_CONSTANT));

    entries.push(make_string_entry(
        "java.rt.vmName",
        "rusty-jvm",
        V_CONSTANT,
    ));
    entries.push(make_string_entry(
        "java.rt.vmVendor",
        "rusty-jvm",
        V_CONSTANT,
    ));
    entries.push(make_string_entry(
        "java.rt.vmVersion",
        env!("CARGO_PKG_VERSION"),
        V_CONSTANT,
    ));

    entries.push(make_long_entry(
        "sun.os.hrt.frequency",
        1_000_000_000,
        U_HERTZ,
        V_CONSTANT,
    ));

    let start_time_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);
    entries.push(make_long_entry(
        "sun.rt.createVmBeginTime",
        start_time_ms,
        U_TICKS,
        V_CONSTANT,
    ));

    serialize(entries)
}

fn serialize(entries: Vec<PerfEntry>) -> Vec<u8> {
    let entry_bufs: Vec<Vec<u8>> = entries.iter().map(serialize_entry).collect();

    let num_entries = entries.len() as i32;
    let entry_offset = PERFDATA_PROLOGUE_SIZE as i32;
    let entries_total_size: usize = entry_bufs.iter().map(|b| b.len()).sum();
    let total_used = (PERFDATA_PROLOGUE_SIZE + entries_total_size) as i32;

    // The file must be at least PERFDATA_CAPACITY bytes.
    // The native Perf.attach() rejects files whose size is not a multiple of the OS
    // page size; using 64 KB matches OpenJDK's default PerfDataMemorySize.
    let file_size = PERFDATA_CAPACITY.max(total_used as usize);

    let mut buf = Vec::with_capacity(file_size);

    // PerfDataPrologue (32 bytes)
    buf.extend_from_slice(&PERFDATA_MAGIC.to_ne_bytes()); // magic
    buf.push(PERFDATA_BYTE_ORDER); // byte_order
    buf.push(PERFDATA_MAJOR_VERSION); // major_version
    buf.push(PERFDATA_MINOR_VERSION); // minor_version
    buf.push(1u8); // accessible = 1 (data is valid)
    buf.extend_from_slice(&total_used.to_ne_bytes()); // used (actual bytes in use)
    buf.extend_from_slice(&0i32.to_ne_bytes()); // overflow
    buf.extend_from_slice(&0i64.to_ne_bytes()); // mod_time_stamp
    buf.extend_from_slice(&entry_offset.to_ne_bytes()); // entry_offset
    buf.extend_from_slice(&num_entries.to_ne_bytes()); // num_entries

    debug_assert_eq!(buf.len(), PERFDATA_PROLOGUE_SIZE);

    for entry_buf in entry_bufs {
        buf.extend_from_slice(&entry_buf);
    }

    // Pad to file_size with zeros so the file size is a page-size multiple
    buf.resize(file_size, 0u8);

    buf
}

fn serialize_entry(entry: &PerfEntry) -> Vec<u8> {
    let name_offset = PERFDATA_ENTRY_HEADER_SIZE as i32;
    let full_name_len = entry.name.len() + 1; // includes null terminator
    let name_end = PERFDATA_ENTRY_HEADER_SIZE + full_name_len;
    let data_offset = align_up(name_end, 8);
    let data_end = data_offset + entry.data.len();
    let entry_length = align_up(data_end, 8);

    let mut buf = Vec::with_capacity(entry_length);

    buf.extend_from_slice(&(entry_length as i32).to_ne_bytes()); // entry_length
    buf.extend_from_slice(&name_offset.to_ne_bytes()); // name_offset
    buf.extend_from_slice(&entry.vector_length.to_ne_bytes()); // vector_length
    buf.push(entry.data_type); // data_type
    buf.push(entry.flags); // flags
    buf.push(entry.units); // data_units
    buf.push(entry.variability); // data_variability
    buf.extend_from_slice(&(data_offset as i32).to_ne_bytes()); // data_offset

    debug_assert_eq!(buf.len(), PERFDATA_ENTRY_HEADER_SIZE);

    // name (null-terminated)
    buf.extend_from_slice(entry.name.as_bytes());
    buf.push(0u8); // null terminator

    // padding between name and data
    buf.resize(data_offset, 0u8);

    // data
    buf.extend_from_slice(&entry.data);

    // padding to entry_length
    buf.resize(entry_length, 0u8);

    buf
}

fn align_up(val: usize, align: usize) -> usize {
    (val + align - 1) & !(align - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_size_is_capacity() {
        let entries: Vec<PerfEntry> = Vec::new();
        let data = serialize(entries);
        assert_eq!(data.len(), PERFDATA_CAPACITY);
    }

    #[test]
    fn test_prologue_at_start_of_capacity_file() {
        let entries: Vec<PerfEntry> = Vec::new();
        let data = serialize(entries);
        // used = PERFDATA_PROLOGUE_SIZE (only the header when no entries)
        let used = i32::from_ne_bytes(data[8..12].try_into().unwrap());
        assert_eq!(used as usize, PERFDATA_PROLOGUE_SIZE);
        // rest of file is zeros
        assert!(data[PERFDATA_PROLOGUE_SIZE..].iter().all(|&b| b == 0));
    }

    #[test]
    fn test_magic_bytes() {
        let entries: Vec<PerfEntry> = Vec::new();
        let data = serialize(entries);
        // Magic bytes must always be {0xca, 0xfe, 0xc0, 0xc0} regardless of endianness
        assert_eq!(&data[0..4], &[0xca, 0xfe, 0xc0, 0xc0]);
    }

    #[test]
    fn test_major_minor_version() {
        let entries: Vec<PerfEntry> = Vec::new();
        let data = serialize(entries);
        assert_eq!(data[5], PERFDATA_MAJOR_VERSION);
        assert_eq!(data[6], PERFDATA_MINOR_VERSION);
    }

    #[test]
    fn test_accessible_flag() {
        let entries: Vec<PerfEntry> = Vec::new();
        let data = serialize(entries);
        assert_eq!(data[7], 1u8); // accessible = 1
    }

    #[test]
    fn test_entry_offset() {
        let entries: Vec<PerfEntry> = Vec::new();
        let data = serialize(entries);
        let entry_offset =
            i32::from_ne_bytes(data[24..28].try_into().unwrap());
        assert_eq!(entry_offset, PERFDATA_PROLOGUE_SIZE as i32);
    }

    #[test]
    fn test_string_entry_serialization() {
        let entry = make_string_entry("sun.rt.javaCommand", "MainClass arg1", V_CONSTANT);
        let buf = serialize_entry(&entry);

        // entry_length must be 8-byte aligned
        assert_eq!(buf.len() % 8, 0);

        // name_offset is at buf[4..8]
        let name_offset = i32::from_ne_bytes(buf[4..8].try_into().unwrap());
        assert_eq!(name_offset, PERFDATA_ENTRY_HEADER_SIZE as i32);

        // data_type is 'B'
        assert_eq!(buf[12], b'B');

        // units is U_STRING
        assert_eq!(buf[14], U_STRING);

        // variability is V_CONSTANT
        assert_eq!(buf[15], V_CONSTANT);

        // name starts at name_offset
        let name_bytes = &buf[name_offset as usize..name_offset as usize + "sun.rt.javaCommand".len()];
        assert_eq!(name_bytes, b"sun.rt.javaCommand");
    }

    #[test]
    fn test_long_entry_serialization() {
        let entry = make_long_entry("sun.os.hrt.frequency", 1_000_000_000, U_HERTZ, V_CONSTANT);
        let buf = serialize_entry(&entry);

        // entry_length must be 8-byte aligned
        assert_eq!(buf.len() % 8, 0);

        // data_type is 'J'
        assert_eq!(buf[12], b'J');

        // units is U_HERTZ
        assert_eq!(buf[14], U_HERTZ);

        // vector_length is 0 for scalars
        let vector_length = i32::from_ne_bytes(buf[8..12].try_into().unwrap());
        assert_eq!(vector_length, 0);

        // check data value
        let data_offset = i32::from_ne_bytes(buf[16..20].try_into().unwrap()) as usize;
        let value = i64::from_ne_bytes(buf[data_offset..data_offset + 8].try_into().unwrap());
        assert_eq!(value, 1_000_000_000i64);
    }

    #[test]
    fn test_supported_flag_for_java_namespace() {
        let entry = make_string_entry("java.rt.vmName", "rusty-jvm", V_CONSTANT);
        let buf = serialize_entry(&entry);
        assert_eq!(buf[13], F_SUPPORTED); // flags byte
    }

    #[test]
    fn test_no_supported_flag_for_sun_namespace() {
        let entry = make_string_entry("sun.rt.javaCommand", "cmd", V_CONSTANT);
        let buf = serialize_entry(&entry);
        assert_eq!(buf[13], F_NONE); // flags byte
    }

    #[test]
    fn test_full_data_has_correct_num_entries() {
        let arguments = Arguments::new_with_entry_point("MainClass".to_string());
        let data = build_perf_data(&arguments);

        // num_entries is at offset 28
        let num_entries = i32::from_ne_bytes(data[28..32].try_into().unwrap());
        assert_eq!(num_entries, 8); // 8 entries total
    }
}
