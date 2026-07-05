use crate::vm::error::{Error, Result};
use crate::vm::exception::pending_helpers::{
    set_pending_array_index_out_of_bounds_exception, set_pending_array_store_exception,
    set_pending_null_pointer_exception,
};
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::undecorate;
use crate::vm::method_area::instance_checker::InstanceChecker;
use crate::vm::method_area::lookup;
use crate::vm::method_area::primitives_helper::PRIMITIVE_TYPE_BY_CODE;
use crate::vm::system_native::string::get_utf8_string_by_ref;

/// `java.lang.System.currentTimeMillis()J`
pub(crate) fn current_time_millis() -> Result<i64> {
    let now = std::time::SystemTime::now();
    let since_the_epoch = now.duration_since(std::time::UNIX_EPOCH)?;
    Ok(since_the_epoch.as_millis() as i64)
}

/// `java.lang.System.nanoTime()J`
pub(crate) fn nano_time() -> Result<i64> {
    let now = std::time::SystemTime::now();
    let since_the_epoch = now.duration_since(std::time::UNIX_EPOCH)?;
    Ok(since_the_epoch.as_nanos() as i64)
}

/// `java.lang.System.arraycopy(Ljava/lang/Object;ILjava/lang/Object;II)V`
pub(crate) fn arraycopy(
    src_ref: i32,
    src_pos: i32,
    dest_ref: i32,
    dest_pos: i32,
    length: i32,
) -> Result<()> {
    if src_ref == 0 || dest_ref == 0 {
        set_pending_null_pointer_exception()?;
        return Ok(());
    }

    let src_name = HEAP.get_instance_name(src_ref)?;
    let dest_name = HEAP.get_instance_name(dest_ref)?;
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
        set_pending_array_store_exception(&msg)?;
        return Ok(());
    }

    let mut primitive = false;
    let mut fast_path = false;
    if is_primitive_array(&src_name) {
        if !is_primitive_array(&dest_name) {
            let src_type_name = get_primitive_element_type_name(&src_name)?;
            set_pending_array_store_exception(&format!(
                "arraycopy: type mismatch: can not copy {src_type_name}[] into object array[]"
            ))?;
            return Ok(());
        } else if src_name != dest_name {
            let src_type_name = get_primitive_element_type_name(&src_name)?;
            let dest_type_name = get_primitive_element_type_name(&dest_name)?;
            set_pending_array_store_exception(&format!(
                "arraycopy: type mismatch: can not copy {src_type_name}[] into {dest_type_name}[]"
            ))?;
            return Ok(());
        }
        primitive = true;
    } else {
        // src is object array
        if is_primitive_array(&dest_name) {
            let dest_type_name = get_primitive_element_type_name(&dest_name)?;
            set_pending_array_store_exception(&format!(
                "arraycopy: type mismatch: can not copy object array[] into {dest_type_name}[]"
            ))?;
            return Ok(());
        } else {
            // check if src and dest types are compatible,
            // if it's so we can use fast path without per-element checks
            fast_path = InstanceChecker::checkcast(&src_name, &dest_name)?;
        }
    }

    let src_len = HEAP.get_array_len(src_ref)?;
    let dest_len = HEAP.get_array_len(dest_ref)?;
    if src_pos < 0 || dest_pos < 0 || length < 0 {
        let msg = if src_pos < 0 {
            let src_array = if primitive {
                get_primitive_element_type_name(&src_name)?
            } else {
                "object array"
            };
            format!("arraycopy: source index {src_pos} out of bounds for {src_array}[{src_len}]")
        } else if dest_pos < 0 {
            let dest_array = if primitive {
                get_primitive_element_type_name(&dest_name)?
            } else {
                "object array"
            };
            format!("arraycopy: destination index {dest_pos} out of bounds for {dest_array}[{dest_len}]")
        } else {
            format!("arraycopy: length {length} is negative")
        };
        set_pending_array_index_out_of_bounds_exception(&msg)?;
        return Ok(());
    }

    if src_pos as u32 + length as u32 > src_len as u32
        || dest_pos as u32 + length as u32 > dest_len as u32
    {
        let msg = if src_pos as u32 + length as u32 > src_len as u32 {
            let src_array = if primitive {
                get_primitive_element_type_name(&src_name)?
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
                get_primitive_element_type_name(&dest_name)?
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
        set_pending_array_index_out_of_bounds_exception(&msg)?;
        return Ok(());
    }

    // nothing to copy
    if length == 0 {
        return Ok(());
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
        let value_to_set = HEAP.get_array_value(src_ref, src_pos + i)?;

        if let Some(ref dest_element_type) = dest_element_type_name {
            let element_ref = value_to_set[0];
            if element_ref != 0 {
                let element_type_name = HEAP.get_instance_name(element_ref)?;
                if !InstanceChecker::checkcast(&element_type_name, dest_element_type)? {
                    set_pending_array_store_exception(&format!(
                        "arraycopy: element type mismatch: can not cast one of the elements of {}[] to the type of the destination array, {}",
                        to_external(undecorate(&src_name[1..])),
                        to_external(dest_element_type),
                    ))?;
                    return Ok(());
                }
            }
        }

        HEAP.set_array_value(dest_ref, dest_pos + i, value_to_set)?;

        i += step;
    }

    Ok(())
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

/// `java.lang.System.setIn0(Ljava/io/InputStream;)V`
pub(crate) fn set_in0(_input_stream_ref: i32) -> Result<()> {
    // todo: implement me
    Ok(())
}

/// `java.lang.System.setOut0(Ljava/io/PrintStream;)V`
pub(crate) fn set_out0(print_stream_ref: i32) -> Result<()> {
    let (_, field_ref) = lookup::lookup_for_static_field("java/lang/System", "out")?
        .ok_or_else(|| Error::new_execution("Field System.out not found"))?;
    field_ref.set_raw_value(vec![print_stream_ref])
}

/// `java.lang.System.setErr0(Ljava/io/PrintStream;)V`
pub(crate) fn set_err0(print_stream_ref: i32) -> Result<()> {
    let (_, field_ref) = lookup::lookup_for_static_field("java/lang/System", "err")?
        .ok_or_else(|| Error::new_execution("Field System.err not found"))?;
    field_ref.set_raw_value(vec![print_stream_ref])
}

/// `java.lang.System.mapLibraryName(Ljava/lang/String;)Ljava/lang/String;`
pub(crate) fn map_library_name(name_ref: i32) -> Result<i32> {
    let name = get_utf8_string_by_ref(name_ref)?;
    let library_name = libloading::library_filename(name).into_string()?;
    let library_name_ref = StringPoolHelper::get_string(&library_name)?;

    Ok(library_name_ref)
}

/// `java.lang.System.registerNatives()V`
pub(crate) fn register_natives() -> Result<()> {
    // todo implement me
    Ok(())
}
