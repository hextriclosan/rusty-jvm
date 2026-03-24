mod perf_file;

use crate::vm::error::Result;
use crate::vm::perf_data::perf_file::PerfFile;
use crate::Arguments;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static PERF_FILE: Lazy<Mutex<PerfFile>> =
    Lazy::new(|| Mutex::new(PerfFile::default().expect("Failed to create perf file")));

pub(crate) fn init_perf_file(arguments: &Arguments) -> Result<()> {
    let mut guard = PERF_FILE.lock()?;

    let java_command = {
        let mut cmd = arguments.entry_point().clone();
        for arg in arguments.program_args() {
            cmd.push(' ');
            cmd.push_str(arg);
        }
        cmd
    };

    guard.create_string("sun.rt.javaCommand", &java_command)?;

    let vm_args = arguments.jvm_options().join(" ");
    guard.create_string("sun.rt.vmArgs", &vm_args)?;

    let vm_flags = arguments.advanced_jvm_options().join(" ");
    guard.create_string("sun.rt.vmFlags", &vm_flags)?;

    Ok(())
}

pub(crate) fn create_long(
    name: &str,
    variability: u8,
    units: u8,
    value: i64,
) -> Result<(*const u8, usize)> {
    let mut guard = PERF_FILE.lock()?;
    guard.create_long(&name, variability, units, value)
}

pub(crate) fn create_byte_array(
    name: &str,
    variability: u8,
    units: u8,
    value: &[u8],
    max_len: usize,
) -> Result<(*const u8, usize)> {
    let mut guard = PERF_FILE.lock()?;
    guard.create_byte_array(&name, variability, units, value, max_len)
}
