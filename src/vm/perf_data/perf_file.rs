use crate::Arguments;
use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::path::PathBuf;
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

// jvmCapabilities is a 64-character string where each position is '0' or '1'.
// Position 0: isAttachable (JVMTI attach mechanism supported).
// rusty-jvm does not implement the attach mechanism, so all positions are '0'.
const JVM_CAPABILITIES: &str = "0000000000000000000000000000000000000000000000000000000000000000";

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

/// Handle to a live perf (hsperfdata) file.
///
/// The file is memory-mapped for the lifetime of this handle — matching how
/// OpenJDK manages its own hsperfdata file.  When the handle is dropped (i.e.
/// when the JVM exits), the mapping is flushed, unmapped, and the file is
/// deleted, so that `jps` / VisualVM no longer list the process.
pub(crate) struct PerfFile {
    mmap: Option<MmapMut>,
    path: PathBuf,
}

impl Drop for PerfFile {
    fn drop(&mut self) {
        // Flush and unmap before removing the file (important on Windows,
        // where you cannot delete a file that has an open mapping).
        drop(self.mmap.take());
        let _ = fs::remove_file(&self.path);
    }
}

pub(crate) fn create_perf_file(arguments: &Arguments) -> Option<PerfFile> {
    match try_create_perf_file(arguments) {
        Ok(pf) => Some(pf),
        Err(e) => {
            tracing::warn!("Failed to create perf file: {e}");
            None
        }
    }
}

fn try_create_perf_file(arguments: &Arguments) -> io::Result<PerfFile> {
    let pid = std::process::id();
    let perf_dir = get_hsperfdata_dir();

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
    file.set_len(PERFDATA_CAPACITY as u64)?;

    // SAFETY: We just created this file and hold the only file descriptor.
    // No other process should be mapping it yet.
    let mut mmap = unsafe { MmapMut::map_mut(&file)? };

    let data = build_perf_data(arguments);
    mmap[..data.len()].copy_from_slice(&data);
    mmap.flush()?;

    Ok(PerfFile {
        mmap: Some(mmap),
        path: file_path,
    })
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

    // --- runtime ---
    let java_command = {
        let mut cmd = arguments.entry_point().clone();
        for arg in arguments.program_args() {
            cmd.push(' ');
            cmd.push_str(arg);
        }
        cmd
    };
    entries.push(make_string_entry("sun.rt.javaCommand", &java_command, V_CONSTANT));

    // NOTE: JDK 9+ uses java.rt.vmArgs / java.rt.vmFlags (not sun.rt.*).
    // MonitoredVmUtil.jvmArgs() / jvmFlags() in JDK 17 read the java.rt.* names.
    let vm_args = arguments.jvm_options().join(" ");
    entries.push(make_string_entry("java.rt.vmArgs", &vm_args, V_CONSTANT));

    let vm_flags = arguments.advanced_jvm_options().join(" ");
    entries.push(make_string_entry("java.rt.vmFlags", &vm_flags, V_CONSTANT));

    entries.push(make_string_entry("java.rt.vmName", "Rusty JVM", V_CONSTANT));
    entries.push(make_string_entry("java.rt.vmVendor", "rusty-jvm", V_CONSTANT));
    entries.push(make_string_entry(
        "java.rt.vmVersion",
        env!("CARGO_PKG_VERSION"),
        V_CONSTANT,
    ));

    // jvmCapabilities: 64-char string, position 0 = isAttachable.
    // rusty-jvm does not implement the JVMTI attach mechanism, so all zeros.
    entries.push(make_string_entry(
        "sun.rt.jvmCapabilities",
        JVM_CAPABILITIES,
        V_CONSTANT,
    ));

    // java.property.* counters used by VisualVM's overview panel and
    // MonitoredVmUtil.vmVersion().
    entries.push(make_string_entry(
        "java.property.java.vm.name",
        "Rusty JVM",
        V_CONSTANT,
    ));
    entries.push(make_string_entry(
        "java.property.java.vm.vendor",
        "rusty-jvm",
        V_CONSTANT,
    ));
    entries.push(make_string_entry(
        "java.property.java.vm.version",
        env!("CARGO_PKG_VERSION"),
        V_CONSTANT,
    ));
    entries.push(make_string_entry(
        "java.property.java.vm.info",
        "interpreted mode",
        V_CONSTANT,
    ));

    // --- HRT / timing ---
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

    let mut buf = Vec::with_capacity(PERFDATA_PROLOGUE_SIZE + entries_total_size);

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

    // The caller (try_create_perf_file) writes this into a PERFDATA_CAPACITY-byte
    // mmap; the remaining bytes are already zeroed by the OS.
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
        let entry_offset = i32::from_ne_bytes(data[24..28].try_into().unwrap());
        assert_eq!(entry_offset, PERFDATA_PROLOGUE_SIZE as i32);
    }

    #[test]
    fn test_used_equals_prologue_when_no_entries() {
        let entries: Vec<PerfEntry> = Vec::new();
        let data = serialize(entries);
        let used = i32::from_ne_bytes(data[8..12].try_into().unwrap());
        assert_eq!(used as usize, PERFDATA_PROLOGUE_SIZE);
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
        let name_bytes =
            &buf[name_offset as usize..name_offset as usize + "sun.rt.javaCommand".len()];
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
        let entry = make_string_entry("java.rt.vmName", "Rusty JVM", V_CONSTANT);
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
        // 3 runtime strings (javaCommand, vmArgs, vmFlags)
        // + 3 java.rt strings (vmName, vmVendor, vmVersion)
        // + 1 sun.rt.jvmCapabilities
        // + 4 java.property strings (vm.name, vm.vendor, vm.version, vm.info)
        // + 2 longs (hrt.frequency, createVmBeginTime)
        assert_eq!(num_entries, 13);
    }

    #[test]
    fn test_vm_args_counter_uses_java_rt_namespace() {
        let arguments = Arguments::new_with_entry_point("MainClass".to_string());
        let data = build_perf_data(&arguments);

        // Scan all entries for the vmArgs counter name
        let entry_offset = PERFDATA_PROLOGUE_SIZE;
        let total_used = i32::from_ne_bytes(data[8..12].try_into().unwrap()) as usize;
        let mut pos = entry_offset;
        let mut found_java_rt = false;
        let mut found_sun_rt = false;
        while pos < total_used {
            let entry_len = i32::from_ne_bytes(data[pos..pos + 4].try_into().unwrap()) as usize;
            if entry_len == 0 {
                break;
            }
            let name_offset = i32::from_ne_bytes(data[pos + 4..pos + 8].try_into().unwrap()) as usize;
            let name_start = pos + name_offset;
            let name_end = data[name_start..].iter().position(|&b| b == 0).unwrap_or(0) + name_start;
            let name = std::str::from_utf8(&data[name_start..name_end]).unwrap_or("");
            if name == "java.rt.vmArgs" {
                found_java_rt = true;
            }
            if name == "sun.rt.vmArgs" {
                found_sun_rt = true;
            }
            pos += entry_len;
        }
        assert!(found_java_rt, "java.rt.vmArgs counter not found");
        assert!(!found_sun_rt, "old sun.rt.vmArgs counter should not be present");
    }

    #[test]
    fn test_jvm_capabilities_present() {
        let arguments = Arguments::new_with_entry_point("MainClass".to_string());
        let data = build_perf_data(&arguments);

        let entry_offset = PERFDATA_PROLOGUE_SIZE;
        let total_used = i32::from_ne_bytes(data[8..12].try_into().unwrap()) as usize;
        let mut pos = entry_offset;
        let mut found = false;
        while pos < total_used {
            let entry_len = i32::from_ne_bytes(data[pos..pos + 4].try_into().unwrap()) as usize;
            if entry_len == 0 {
                break;
            }
            let name_offset = i32::from_ne_bytes(data[pos + 4..pos + 8].try_into().unwrap()) as usize;
            let name_start = pos + name_offset;
            let name_end = data[name_start..].iter().position(|&b| b == 0).unwrap_or(0) + name_start;
            let name = std::str::from_utf8(&data[name_start..name_end]).unwrap_or("");
            if name == "sun.rt.jvmCapabilities" {
                found = true;
                // Verify the value is exactly 64 ASCII chars ('0' or '1')
                let data_offset = i32::from_ne_bytes(data[pos + 16..pos + 20].try_into().unwrap()) as usize;
                let vec_len = i32::from_ne_bytes(data[pos + 8..pos + 12].try_into().unwrap()) as usize;
                assert_eq!(vec_len, 65, "jvmCapabilities vector_length should be 65 (64 chars + null)");
                let cap_bytes = &data[pos + data_offset..pos + data_offset + 64];
                assert!(
                    cap_bytes.iter().all(|&b| b == b'0' || b == b'1'),
                    "jvmCapabilities must contain only '0' or '1' characters"
                );
            }
            pos += entry_len;
        }
        assert!(found, "sun.rt.jvmCapabilities counter not found");
    }
}
