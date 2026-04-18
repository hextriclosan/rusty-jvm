use crate::vm::error::Result;
use crate::vm::exception::helpers::{
    throw_illegal_argument_exception, throw_null_pointer_exception,
};
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::i32toi64;
use crate::vm::perf_data::Units::{UHertz, UString};
use crate::vm::perf_data::Variability::{VConstant, VMonotonic, VVariable};
use crate::vm::perf_data::{contains_name, create_byte_array, create_long};
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use crate::{throw_and_return, unwrap_or_return_err, unwrap_result_or_return_default};

pub(crate) fn perf_create_long_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let perf_ref = args[0];
    let name_ref = args[1];
    let variability = args[2];
    let units = args[3];
    let value = i32toi64(args[5], args[4]);

    let class_ref = unwrap_result_or_return_default!(
        perf_create_long(perf_ref, name_ref, variability, units, value, stack_frames),
        vec![]
    );
    Ok(vec![class_ref])
}
fn perf_create_long(
    _perf_ref: i32,
    name_ref: i32,
    variability: i32,
    units: i32,
    value: i64,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<i32> {
    if name_ref == 0 {
        throw_and_return!(throw_null_pointer_exception(stack_frames))
    }

    if units <= 0 || units > UHertz as i32 {
        throw_and_return!(throw_illegal_argument_exception(
            &format!("Invalid units: {units}"),
            stack_frames
        ))
    }

    if variability != VConstant as i32
        && variability != VMonotonic as i32
        && variability != VVariable as i32
    {
        throw_and_return!(throw_illegal_argument_exception(
            &format!("Invalid variability: {variability}"),
            stack_frames
        ))
    }

    let name = unwrap_or_return_err!(get_utf8_string_by_ref(name_ref));
    if unwrap_or_return_err!(contains_name(&name)) {
        throw_and_return!(throw_illegal_argument_exception(
            &format!("A perf data entry with name '{name}' already exists"),
            stack_frames
        ))
    }

    let (ptr, len) =
        unwrap_or_return_err!(create_long(&name, variability as u8, units as u8, value));
    let ptr = ptr as i64;
    let len = len as i64;

    let byte_buffer_ref_res = crate::vm::concurrency::block_on_async(
        Executor::invoke_args_constructor(
            "java/nio/DirectByteBuffer",
            "<init>:(JJ)V",
            &[ptr.into(), len.into()],
            Some("native image buffer creation"),
        )
    );
    let byte_buffer_ref = unwrap_or_return_err!(byte_buffer_ref_res);

    ThrowingResult::ok(byte_buffer_ref)
}

pub(crate) fn perf_create_byte_array_wrp(
    args: &[i32],
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let perf_ref = args[0];
    let name_ref = args[1];
    let variability = args[2];
    let units = args[3];
    let byte_arr_ref = args[4];
    let max_len = args[5];

    let class_ref = unwrap_result_or_return_default!(
        perf_create_byte_array(
            perf_ref,
            name_ref,
            variability,
            units,
            byte_arr_ref,
            max_len,
            stack_frames,
        ),
        vec![]
    );

    Ok(vec![class_ref])
}
fn perf_create_byte_array(
    _perf_ref: i32,
    name_ref: i32,
    variability: i32,
    units: i32,
    byte_arr_ref: i32,
    max_len: i32,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<i32> {
    if name_ref == 0 || byte_arr_ref == 0 {
        throw_and_return!(throw_null_pointer_exception(stack_frames))
    }

    if variability != VConstant as i32 && variability != VVariable as i32 {
        throw_and_return!(throw_illegal_argument_exception(
            &format!("Invalid variability: {variability}"),
            stack_frames
        ))
    }

    if units != UString as i32 {
        throw_and_return!(throw_illegal_argument_exception(
            &format!("Invalid units: {units}"),
            stack_frames
        ))
    }

    let name = unwrap_or_return_err!(get_utf8_string_by_ref(name_ref));
    if unwrap_or_return_err!(contains_name(&name)) {
        throw_and_return!(throw_illegal_argument_exception(
            &format!("A perf data entry with name '{name}' already exists"),
            stack_frames
        ))
    }

    let byte_array_data = {
        let guard = unwrap_or_return_err!(HEAP.get_entire_raw_data(byte_arr_ref));
        guard.to_vec()
    };
    let (ptr, len) = unwrap_or_return_err!(create_byte_array(
        &name,
        variability as u8,
        units as u8,
        &byte_array_data,
        max_len as usize,
    ));

    let ptr = ptr as i64;
    let len = len as i64;

    let byte_buffer_ref_res = crate::vm::concurrency::block_on_async(
        Executor::invoke_args_constructor(
            "java/nio/DirectByteBuffer",
            "<init>:(JJ)V",
            &[ptr.into(), len.into()],
            Some("native image buffer creation"),
        )
    );
    let byte_buffer_ref = unwrap_or_return_err!(byte_buffer_ref_res);

    ThrowingResult::ok(byte_buffer_ref)
}
