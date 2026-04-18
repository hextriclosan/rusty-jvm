use crate::vm::error::Result;
use crate::vm::exception::helpers::throw_unsatisfied_link_error;
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{i32toi64, i64_to_vec};
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::properties_provider::properties::sun_boot_library_path;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use crate::{throw_and_return, unwrap_or_return_err, unwrap_result_or_return_default};
use dashmap::DashMap;
use derive_new::new;
#[cfg(unix)]
use libloading::os::unix::*;
#[cfg(windows)]
use libloading::os::windows::*;
use std::ffi::{c_void, CString};
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::sync::LazyLock;
use tracing::{enabled, trace, warn};

static REGISTRY: LazyLock<DashMap<usize, LibraryEntry>> = LazyLock::new(DashMap::default);

#[derive(Debug, new)]
struct LibraryEntry {
    library: Library,
    symbols: DashMap<String, usize>,
}

pub(crate) fn find_builtin_lib_wrp(args: &[i32]) -> Result<Vec<i32>> {
    if enabled!(tracing::Level::TRACE) {
        let name_ref = args[0];
        let name = get_utf8_string_by_ref(name_ref)?;
        trace!("findBuiltinLib: {name}");
    }

    Ok(vec![0]) // we don't have static libraries, so we always return null
}

pub(crate) fn native_libraries_load_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let native_lib_impl_ref = args[0];
    let name_ref = args[1];
    let builtin = args[2] != 0;
    let throw_exception_if_fail = args[3] != 0;

    let result = unwrap_result_or_return_default!(
        native_libraries_load(
            native_lib_impl_ref,
            name_ref,
            builtin,
            throw_exception_if_fail,
            stack_frames
        ),
        vec![]
    );
    Ok(vec![if result { 1 } else { 0 }])
}
fn native_libraries_load(
    native_lib_impl_ref: i32,
    name_ref: i32,
    _builtin: bool,
    throw_exception_if_fail: bool,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<bool> {
    let name = unwrap_or_return_err!(get_utf8_string_by_ref(name_ref));

    // skip loading of jdk system libraries (libzip, libnio, libjimage and so on) since we have this functionality built-in
    let path = Path::new(&name);
    if let Some(parent_dir) = path.parent() {
        let canonic_parent_dir = unwrap_or_return_err!(fs::canonicalize(parent_dir));
        let canonic_boot_lib_path =
            unwrap_or_return_err!(fs::canonicalize(Path::new(sun_boot_library_path())));
        if canonic_parent_dir == canonic_boot_lib_path {
            if path
                .file_name()
                .and_then(|n| n.to_str())
                .and_then(|s| extract_lib_name(s))
                .is_some_and(|lib_name| ["nio", "jimage", "zip", "net"].contains(&lib_name))
            {
                return ThrowingResult::ok(true);
            }
        }
    }

    match unsafe { Library::new(&name) } {
        Ok(lib) => {
            let raw_ptr = lib.into_raw();
            let entry = REGISTRY.entry(raw_ptr as usize).or_insert_with(|| {
                // Safety: raw_ptr was just obtained from lib.into_raw() and is not yet
                // wrapped by another Library instance.
                let lib = unsafe { Library::from_raw(raw_ptr) };
                LibraryEntry::new(lib, DashMap::default())
            });
            let result = HEAP.set_object_field_value(
                native_lib_impl_ref,
                "jdk/internal/loader/NativeLibraries$NativeLibraryImpl",
                "handle",
                i64_to_vec(*entry.key() as i64),
            );
            unwrap_or_return_err!(result);

            ThrowingResult::ok(true)
        }
        Err(_) => {
            if throw_exception_if_fail {
                throw_and_return!(throw_unsatisfied_link_error(
                    &format!("Failed to load {name}"),
                    stack_frames
                ))
            } else {
                ThrowingResult::ok(false)
            }
        }
    }
}

fn extract_lib_name(s: &str) -> Option<&str> {
    let s = s.strip_prefix("lib").unwrap_or(s);
    Some(s.rsplit_once('.')?.0)
}

pub(crate) fn native_libraries_find_entry0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let handle = i32toi64(args[1], args[0]);
    let name_ref = args[2];

    let entry_id = native_libraries_find_entry0(handle, name_ref)?;
    Ok(i64_to_vec(entry_id))
}

fn native_libraries_find_entry0(handle: i64, name_ref: i32) -> Result<i64> {
    let lib = match REGISTRY.get(&(handle as usize)) {
        Some(lib) => lib,
        None => {
            warn!("Library not found in registry for handle: {handle}");
            return Ok(0);
        }
    };

    let name = get_utf8_string_by_ref(name_ref)?;
    let entry = lib.value();

    let c_name = CString::from_str(&name)?;
    let symbol = entry.symbols.entry(name.clone()).or_insert_with(|| {
        let sym: core::result::Result<Symbol<*const c_void>, libloading::Error> =
            unsafe { entry.library.get(c_name.as_bytes_with_nul()) };
        match sym {
            Ok(sym) => sym.as_raw_ptr() as usize,
            Err(_) => 0,
        }
    });

    let raw_ptr = *symbol.value();
    if raw_ptr == 0 {
        return Ok(0);
    }

    Ok(raw_ptr as i64)
}
