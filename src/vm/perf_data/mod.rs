mod perf_file;

use crate::vm::error::{Error, Result};
use crate::vm::perf_data::perf_file::PerfFile;
use crate::Arguments;
use once_cell::sync::OnceCell;
pub(crate) use perf_file::{Units, Variability};
use std::fmt::Display;
use std::sync::Mutex;

static PERF_FILE: OnceCell<Mutex<PerfFile>> = OnceCell::new();

pub(crate) enum PerfDataError {
    AlreadyExists(String),
    NonRecoverable(String),
}

impl Display for PerfDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PerfDataError::AlreadyExists(name) => {
                write!(f, "Perf data entry '{}' already exists", name)
            }
            PerfDataError::NonRecoverable(msg) => write!(f, "Non-recoverable error: {}", msg),
        }
    }
}

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

    let vm_args = arguments.jvm_options().join(" ");
    guard.create_string("sun.rt.vmArgs", &vm_args)?;

    let vm_flags = arguments.advanced_jvm_options().join(" ");
    guard.create_string("sun.rt.vmFlags", &vm_flags)?;

    Ok(())
}

pub(crate) fn create_long_in_perf_file(
    name: &str,
    variability: u8,
    units: u8,
    value: i64,
) -> std::result::Result<(*const u8, usize), PerfDataError> {
    let Some(mutex) = PERF_FILE.get() else {
        return Err(PerfDataError::NonRecoverable(
            "Perf file not initialized".to_string(),
        ));
    };

    let mut guard = match mutex.lock() {
        Ok(guard) => guard,
        Err(e) => {
            return Err(PerfDataError::NonRecoverable(format!(
                "Failed to acquire lock: {e}"
            )));
        }
    };

    if guard.contains_name(name) {
        return Err(PerfDataError::AlreadyExists(name.to_string()));
    }

    match guard.create_long(name, variability, units, value) {
        Ok(result) => Ok(result),
        Err(e) => Err(PerfDataError::NonRecoverable(format!(
            "Failed to create long in perf file: {e}"
        ))),
    }
}

pub(crate) fn create_byte_array_in_perf_file(
    name: &str,
    variability: u8,
    units: u8,
    value: &[u8],
    max_len: usize,
) -> std::result::Result<(*const u8, usize), PerfDataError> {
    let Some(mutex) = PERF_FILE.get() else {
        return Err(PerfDataError::NonRecoverable(
            "Perf file not initialized".to_string(),
        ));
    };

    let mut guard = match mutex.lock() {
        Ok(guard) => guard,
        Err(e) => {
            return Err(PerfDataError::NonRecoverable(format!(
                "Failed to acquire lock: {e}"
            )))
        }
    };

    if guard.contains_name(name) {
        return Err(PerfDataError::AlreadyExists(name.to_string()));
    }

    match guard.create_byte_array(name, variability, units, value, max_len) {
        Ok(result) => Ok(result),
        Err(e) => Err(PerfDataError::NonRecoverable(format!(
            "Failed to create byte array in perf file: {e}"
        ))),
    }
}
