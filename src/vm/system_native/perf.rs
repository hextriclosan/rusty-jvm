use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::i32toi64;
use crate::vm::perf_data::{create_byte_array, create_long};
use crate::vm::system_native::string::get_utf8_string_by_ref;

pub(crate) fn perf_create_long_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let perf_ref = args[0];
    let name_ref = args[1];
    let variability = args[2];
    let units = args[3];
    let value = i32toi64(args[5], args[4]);

    let class_ref = perf_create_long(perf_ref, name_ref, variability, units, value)?;

    Ok(vec![class_ref])
}
fn perf_create_long(
    _perf_ref: i32,
    name_ref: i32,
    variability: i32,
    units: i32,
    value: i64,
) -> Result<i32> {
    let name = get_utf8_string_by_ref(name_ref)?;

    let (ptr, len) = create_long(&name, variability as u8, units as u8, value)?;
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

pub(crate) fn perf_create_byte_array_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let perf_ref = args[0];
    let name_ref = args[1];
    let variability = args[2];
    let units = args[3];
    let byte_arr_ref = args[4];
    let max_len = args[5];

    let class_ref = perf_create_byte_array(
        perf_ref,
        name_ref,
        variability,
        units,
        byte_arr_ref,
        max_len,
    )?;

    Ok(vec![class_ref])
}
fn perf_create_byte_array(
    _perf_ref: i32,
    name_ref: i32,
    variability: i32,
    units: i32,
    byte_arr_ref: i32,
    max_len: i32,
) -> Result<i32> {
    let name = get_utf8_string_by_ref(name_ref)?;

    let byte_array = HEAP.get_entire_raw_data(byte_arr_ref)?;
    let (ptr, len) = create_byte_array(
        &name,
        variability as u8,
        units as u8,
        &byte_array,
        max_len as usize,
    )?;

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
