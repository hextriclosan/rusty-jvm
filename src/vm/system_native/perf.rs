use crate::vm::error::{Error, Result};
use crate::vm::exception::pending_helpers::{
    set_pending_illegal_argument_exception, set_pending_null_pointer_exception,
};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::perf_data::Units::{UHertz, UString};
use crate::vm::perf_data::Variability::{VConstant, VMonotonic, VVariable};
use crate::vm::perf_data::{
    create_byte_array_in_perf_file, create_long_in_perf_file, PerfDataError,
};
use crate::vm::system_native::string::get_utf8_string_by_ref;

/// `jdk.internal.perf.Perf.registerNatives()V`
pub(crate) fn register_natives() -> Result<()> {
    Ok(()) // todo: implement me
}

/// `jdk.internal.perf.Perf.createLong(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;`
pub(crate) fn create_long(
    _perf_ref: i32,
    name_ref: i32,
    variability: i32,
    units: i32,
    value: i64,
) -> Result<i32> {
    if name_ref == 0 {
        set_pending_null_pointer_exception()?;
        return Ok(0);
    }

    if units <= 0 || units > UHertz as i32 {
        set_pending_illegal_argument_exception(&format!("Invalid units: {units}"))?;
        return Ok(0);
    }

    if variability != VConstant as i32
        && variability != VMonotonic as i32
        && variability != VVariable as i32
    {
        set_pending_illegal_argument_exception(&format!("Invalid variability: {variability}"))?;
        return Ok(0);
    }

    let name = get_utf8_string_by_ref(name_ref)?;
    let (ptr, len) = match create_long_in_perf_file(&name, variability as u8, units as u8, value) {
        Ok((ptr, len)) => (ptr, len),
        Err(e) if matches!(e, PerfDataError::AlreadyExists(_)) => {
            set_pending_illegal_argument_exception(&e.to_string())?;
            return Ok(0);
        }
        Err(e) => return Err(Error::new_execution(&e.to_string())),
    };
    let ptr = ptr as i64;
    let len = len as i64;

    let byte_buffer_ref = Executor::invoke_args_constructor(
        "java/nio/DirectByteBuffer",
        "<init>:(JJ)V",
        &[ptr.into(), len.into()],
        Some("native image buffer creation"),
    )?;

    Ok(byte_buffer_ref)
}

/// `jdk.internal.perf.Perf.createByteArray(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;`
pub(crate) fn create_byte_array(
    _perf_ref: i32,
    name_ref: i32,
    variability: i32,
    units: i32,
    byte_arr_ref: i32,
    max_len: i32,
) -> Result<i32> {
    if name_ref == 0 || byte_arr_ref == 0 {
        set_pending_null_pointer_exception()?;
        return Ok(0);
    }

    if variability != VConstant as i32 && variability != VVariable as i32 {
        set_pending_illegal_argument_exception(&format!("Invalid variability: {variability}"))?;
        return Ok(0);
    }

    if units != UString as i32 {
        set_pending_illegal_argument_exception(&format!("Invalid units: {units}"))?;
        return Ok(0);
    }

    let name = get_utf8_string_by_ref(name_ref)?;

    let byte_array_data = {
        let guard = HEAP.get_entire_raw_data(byte_arr_ref)?;
        guard.to_vec()
    };
    let (ptr, len) = match create_byte_array_in_perf_file(
        &name,
        variability as u8,
        units as u8,
        &byte_array_data,
        max_len as usize,
    ) {
        Ok((ptr, len)) => (ptr, len),
        Err(e) if matches!(e, PerfDataError::AlreadyExists(_)) => {
            set_pending_illegal_argument_exception(&e.to_string())?;
            return Ok(0);
        }
        Err(e) => return Err(Error::new_execution(&e.to_string())),
    };

    let ptr = ptr as i64;
    let len = len as i64;

    let byte_buffer_ref = Executor::invoke_args_constructor(
        "java/nio/DirectByteBuffer",
        "<init>:(JJ)V",
        &[ptr.into(), len.into()],
        Some("native image buffer creation"),
    )?;

    Ok(byte_buffer_ref)
}
