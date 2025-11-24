use crate::vm::error::{Error, Result};
use crate::vm::exception::helpers::{
    throw_array_index_out_of_bounds_exception_with_message, throw_array_store_exception,
    throw_null_pointer_exception,
};
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::vm::helper::{i64_to_vec, undecorate};
use crate::vm::method_area::instance_checker::InstanceChecker;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::method_area::primitives_helper::PRIMITIVE_TYPE_BY_CODE;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::object::identity_hashcode;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use crate::{throw_and_return, unwrap_or_return_err, unwrap_result_or_return_default};

pub(crate) fn current_time_millis_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    let millis = current_time_millis()?;
    Ok(i64_to_vec(millis))
}
fn current_time_millis() -> Result<i64> {
    let now = std::time::SystemTime::now();
    let since_the_epoch = now.duration_since(std::time::UNIX_EPOCH)?;
    Ok(since_the_epoch.as_millis() as i64)
}

pub(crate) fn nano_time_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    let nanos = nano_time()?;
    Ok(i64_to_vec(nanos))
}
fn nano_time() -> Result<i64> {
    let now = std::time::SystemTime::now();
    let since_the_epoch = now.duration_since(std::time::UNIX_EPOCH)?;
    Ok(since_the_epoch.as_nanos() as i64)
}

pub(crate) fn arraycopy_wrp(args: &[i32], stack_frames: &mut StackFrames) -> Result<Vec<i32>> {
    let src_ref = args[0];
    let src_pos = args[1];
    let dest_ref = args[2];
    let dest_pos = args[3];
    let length = args[4];

    unwrap_result_or_return_default!(
        arraycopy(src_ref, src_pos, dest_ref, dest_pos, length, stack_frames),
        vec![]
    );
    Ok(vec![])
}
pub(crate) fn arraycopy(
    src_ref: i32,
    src_pos: i32,
    dest_ref: i32,
    dest_pos: i32,
    length: i32,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<()> {
    if src_ref == 0 || dest_ref == 0 {
        throw_and_return!(throw_null_pointer_exception(stack_frames))
    }

    let src_name = unwrap_or_return_err!(with_heap_read_lock(|h| h.get_instance_name(src_ref)));
    let dest_name = unwrap_or_return_err!(with_heap_read_lock(|h| h.get_instance_name(dest_ref)));
    if !src_name.starts_with('[') || !dest_name.starts_with('[') {
        let msg = if !src_name.starts_with('[') {
            format!(
                "arraycopy: source type {} is not an array",
                to_external(&src_name)
            )
        } else {
            format!(
                "arraycopy: destination type {} is not an array",
                to_external(&dest_name)
            )
        };
        throw_and_return!(throw_array_store_exception(&msg, stack_frames))
    }

    let mut primitive = false;
    let mut fast_path = false;
    if is_primitive_array(&src_name) {
        if !is_primitive_array(&dest_name) {
            let src_type_name = unwrap_or_return_err!(get_primitive_element_type_name(&src_name));
            throw_and_return!(throw_array_store_exception(
                &format!(
                    "arraycopy: type mismatch: can not copy {src_type_name}[] into object array[]"
                ),
                stack_frames
            ))
        } else if src_name != dest_name {
            let src_type_name = unwrap_or_return_err!(get_primitive_element_type_name(&src_name));
            let dest_type_name =
                unwrap_or_return_err!(get_primitive_element_type_name(&dest_name));
            throw_and_return!(throw_array_store_exception(
                &format!("arraycopy: type mismatch: can not copy {src_type_name}[] into {dest_type_name}[]"),
                stack_frames,
            ))
        }
        primitive = true;
    } else {
        // src is object array
        if is_primitive_array(&dest_name) {
            let dest_type_name =
                unwrap_or_return_err!(get_primitive_element_type_name(&dest_name));
            throw_and_return!(throw_array_store_exception(
                &format!("arraycopy: type mismatch: can not copy object array[] into {dest_type_name}[]"),
                stack_frames,
            ))
        } else {
            // check if src and dest types are compatible,
            // if it's so we can use fast path without per-element checks
            fast_path = unwrap_or_return_err!(InstanceChecker::checkcast(&src_name, &dest_name));
        }
    }

    let src_len = unwrap_or_return_err!(with_heap_read_lock(|h| h.get_array_len(src_ref)));
    let dest_len = unwrap_or_return_err!(with_heap_read_lock(|h| h.get_array_len(dest_ref)));
    if src_pos < 0 || dest_pos < 0 || length < 0 {
        let msg = if src_pos < 0 {
            let src_array = if primitive {
                unwrap_or_return_err!(get_primitive_element_type_name(&src_name))
            } else {
                "object array"
            };
            format!("arraycopy: source index {src_pos} out of bounds for {src_array}[{src_len}]")
        } else if dest_pos < 0 {
            let dest_array = if primitive {
                unwrap_or_return_err!(get_primitive_element_type_name(&dest_name))
            } else {
                "object array"
            };
            format!("arraycopy: destination index {dest_pos} out of bounds for {dest_array}[{dest_len}]")
        } else {
            format!("arraycopy: length {length} is negative")
        };
        throw_and_return!(throw_array_index_out_of_bounds_exception_with_message(
            &msg,
            stack_frames
        ))
    }

    if src_pos as u32 + length as u32 > src_len as u32
        || dest_pos as u32 + length as u32 > dest_len as u32
    {
        let msg = if src_pos as u32 + length as u32 > src_len as u32 {
            let src_array = if primitive {
                unwrap_or_return_err!(get_primitive_element_type_name(&src_name))
            } else {
                "object array"
            };
            format!(
                "arraycopy: last source index {} out of bounds for {}[{}]",
                src_pos as u32 + length as u32,
                src_array,
                src_len
            )
        } else {
            let dest_array = if primitive {
                unwrap_or_return_err!(get_primitive_element_type_name(&dest_name))
            } else {
                "object array"
            };
            format!(
                "arraycopy: last destination index {} out of bounds for {}[{}]",
                dest_pos as u32 + length as u32,
                dest_array,
                dest_len
            )
        };
        throw_and_return!(throw_array_index_out_of_bounds_exception_with_message(
            &msg,
            stack_frames
        ))
    }

    // nothing to copy
    if length == 0 {
        return ThrowingResult::ok(());
    }

    let overlapping = src_ref == dest_ref && dest_pos > src_pos && dest_pos < src_pos + length;
    let dest_element_type_name = if !primitive && !fast_path {
        Some(undecorate(&dest_name[1..]).to_string())
    } else {
        None
    };

    let (mut i, end, step): (i32, i32, i32) = if overlapping {
        (length - 1, -1, -1)
    } else {
        (0, length, 1)
    };
    while i != end {
        let value_to_set = unwrap_or_return_err!(with_heap_read_lock(
            |h| h.get_array_value(src_ref, src_pos + i)
        ));

        if let Some(ref dest_element_type) = dest_element_type_name {
            let element_ref = value_to_set[0];
            if element_ref != 0 {
                let element_type_name = unwrap_or_return_err!(with_heap_read_lock(
                    |h| h.get_instance_name(element_ref)
                ));
                if !unwrap_or_return_err!(InstanceChecker::checkcast(
                    &element_type_name,
                    &dest_element_type
                )) {
                    throw_and_return!(throw_array_store_exception(
                        &format!(
                            "arraycopy: element type mismatch: can not cast one of the elements of {}[] to the type of the destination array, {}",
                            to_external(&undecorate(&src_name[1..])),
                            to_external(dest_element_type),
                        ),
                        stack_frames
                    ))
                }
            }
        }

        unwrap_or_return_err!(with_heap_write_lock(|h| h.set_array_value(
            dest_ref,
            dest_pos + i,
            value_to_set
        )));

        i += step;
    }

    ThrowingResult::ok(())
}
fn to_external(internal_name: &str) -> String {
    internal_name.replace('/', ".")
}
fn is_primitive_array(array_name: &str) -> bool {
    matches!(
        array_name,
        "[B" | "[C" | "[D" | "[F" | "[I" | "[J" | "[S" | "[Z"
    )
}
fn get_primitive_element_type_name(array_name: &str) -> Result<&str> {
    array_name
        .get(1..)
        .and_then(|element| PRIMITIVE_TYPE_BY_CODE.get(element))
        .ok_or_else(|| {
            Error::new_execution(&format!("Unknown primitive array type: {}", array_name))
        })
        .copied()
}

pub(crate) fn system_identity_hashcode_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let hashcode = identity_hashcode(obj_ref)?;

    Ok(vec![hashcode])
}

pub(crate) fn set_out0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let print_stream_ref = args[0];
    set_out0(print_stream_ref)?;

    Ok(vec![])
}
fn set_out0(print_stream_ref: i32) -> Result<()> {
    let (_, field_ref) = {
        let field_ref = with_method_area(|method_area| {
            method_area.lookup_for_static_field("java/lang/System", "out")
        })?;
        field_ref
    };

    field_ref.set_raw_value(vec![print_stream_ref])
}

pub(crate) fn set_err0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let print_stream_ref = args[0];
    set_err0(print_stream_ref)?;

    Ok(vec![])
}
fn set_err0(print_stream_ref: i32) -> Result<()> {
    let (_, field_ref) = {
        let field_ref = with_method_area(|method_area| {
            method_area.lookup_for_static_field("java/lang/System", "err")
        })?;
        field_ref
    };

    field_ref.set_raw_value(vec![print_stream_ref])
}

pub(crate) fn system_map_library_name_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let name_ref = args[0];
    let library_name_ref = map_library_name(name_ref)?;

    Ok(vec![library_name_ref])
}
fn map_library_name(name_ref: i32) -> Result<i32> {
    let name = get_utf8_string_by_ref(name_ref)?;
    let library_name = libloading::library_filename(name).into_string()?;
    let library_name_ref = StringPoolHelper::get_string(&library_name)?;

    Ok(library_name_ref)
}
