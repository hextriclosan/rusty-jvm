mod perf_file;

use crate::vm::error::{Error, Result};
use crate::vm::perf_data::perf_file::PerfFile;
use crate::Arguments;
use once_cell::sync::OnceCell;
pub(crate) use perf_file::{Units, Variability};
use std::sync::Mutex;

const JVM_CAPABILITIES_LEN: usize = 64;

static PERF_FILE: OnceCell<Mutex<PerfFile>> = OnceCell::new();

pub(crate) fn init_perf_file(arguments: &Arguments) -> Result<()> {
    let mut guard = PERF_FILE
        .get_or_try_init(|| Ok::<Mutex<PerfFile>, Error>(Mutex::new(PerfFile::default()?)))?
        .lock()?;

    let java_command = {
        let mut cmd = arguments.entry_point().clone();
        for arg in arguments.program_args() {
            cmd.push(' ');
            cmd.push_str(arg);
        }
        cmd
    };

    guard.create_string("sun.rt.javaCommand", &java_command)?;

    // JDK 9+ uses java.rt.* namespace instead of sun.rt.*
    let vm_args = arguments.jvm_options().join(" ");
    guard.create_string("java.rt.vmArgs", &vm_args)?;

    let vm_flags = arguments.advanced_jvm_options().join(" ");
    guard.create_string("java.rt.vmFlags", &vm_flags)?;

    // JVM capability flags encoded as a fixed-length ASCII string.
    // Each position represents a capability bit; '0' means not supported.
    guard.create_string("sun.rt.jvmCapabilities", &"0".repeat(JVM_CAPABILITIES_LEN))?;

    // VM identification properties exposed to monitoring tools such as VisualVM.
    guard.create_string("java.property.java.vm.name", "Rusty JVM")?;
    guard.create_string("java.property.java.vm.vendor", "rusty-jvm")?;
    guard.create_string("java.property.java.vm.version", env!("CARGO_PKG_VERSION"))?;
    guard.create_string("java.property.java.vm.info", "interpreted mode")?;

    Ok(())
}

pub(crate) fn create_long(
    name: &str,
    variability: u8,
    units: u8,
    value: i64,
) -> Result<(*const u8, usize)> {
    let Some(mutex) = PERF_FILE.get() else {
        return Err(Error::new_native("PerfFile is not initialized"));
    };

    let mut guard = mutex.lock()?;
    guard.create_long(name, variability, units, value)
}

pub(crate) fn create_byte_array(
    name: &str,
    variability: u8,
    units: u8,
    value: &[u8],
    max_len: usize,
) -> Result<(*const u8, usize)> {
    let Some(mutex) = PERF_FILE.get() else {
        return Err(Error::new_native("PerfFile is not initialized"));
    };

    let mut guard = mutex.lock()?;
    guard.create_byte_array(&name, variability, units, value, max_len)
}

pub(crate) fn contains_name(name: &str) -> Result<bool> {
    let Some(mutex) = PERF_FILE.get() else {
        return Err(Error::new_native("PerfFile is not initialized"));
    };

    Ok(mutex.lock()?.contains_name(name))
}
