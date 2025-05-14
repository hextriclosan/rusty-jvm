use crate::error::Error;
use crate::execution_engine::string_pool_helper::StringPoolHelper;
use crate::heap::heap::with_heap_write_lock;
use crate::method_area::method_area::with_method_area;
use crate::system_native::object::identity_hashcode;
use crate::system_native::string::get_utf8_string_by_ref;

pub(crate) fn current_time_millis_wrp(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let millis = current_time_millis()?;

    let high = ((millis >> 32) & 0xFFFFFFFF) as i32;
    let low = (millis & 0xFFFFFFFF) as i32;

    Ok(vec![high, low])
}
fn current_time_millis() -> crate::error::Result<i64> {
    let now = std::time::SystemTime::now();
    let since_the_epoch = now.duration_since(std::time::UNIX_EPOCH)?;
    Ok(since_the_epoch.as_millis() as i64)
}

pub(crate) fn arraycopy_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let src_ref = args[0];
    let src_pos = args[1];
    let dest_ref = args[2];
    let dest_pos = args[3];
    let length = args[4];

    arraycopy(src_ref, src_pos, dest_ref, dest_pos, length)?;
    Ok(vec![])
}
pub(crate) fn arraycopy(
    src_ref: i32,
    src_pos: i32,
    dest_ref: i32,
    dest_pos: i32,
    length: i32,
) -> crate::error::Result<()> {
    with_heap_write_lock(|heap| -> crate::error::Result<()> {
        let src_array = heap.get_entire_array(src_ref)?;
        let mut dest_array = if src_ref == dest_ref {
            src_array.clone()
        } else {
            heap.get_entire_array(dest_ref)?
        };

        if src_pos < 0 || dest_pos < 0 || length < 0 {
            return Err(Error::new_execution("Negative array index"));
        }
        if (src_pos + length) > src_array.get_length()
            || (dest_pos + length) > dest_array.get_length()
        {
            return Err(Error::new_execution("Array index out of bounds"));
        }

        for i in 0..length {
            //todo: check if dest instance is assignable from src instance
            dest_array.set_value(dest_pos + i, src_array.get_value(src_pos + i)?.clone())?;
        }

        heap.set_entire_array(dest_ref, dest_array)?;

        Ok(())
    })
}

pub(crate) fn system_identity_hashcode_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let obj_ref = args[0];
    let hashcode = identity_hashcode(obj_ref)?;

    Ok(vec![hashcode])
}

pub(crate) fn set_out0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let print_stream_ref = args[0];
    set_out0(print_stream_ref)?;

    Ok(vec![])
}
fn set_out0(print_stream_ref: i32) -> crate::error::Result<()> {
    let (_, field_ref) = {
        let field_ref = with_method_area(|method_area| {
            method_area.lookup_for_static_field("java/lang/System", "out")
        })?;
        field_ref
    };

    field_ref.set_raw_value(vec![print_stream_ref])
}

pub(crate) fn set_err0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let print_stream_ref = args[0];
    set_err0(print_stream_ref)?;

    Ok(vec![])
}
fn set_err0(print_stream_ref: i32) -> crate::error::Result<()> {
    let (_, field_ref) = {
        let field_ref = with_method_area(|method_area| {
            method_area.lookup_for_static_field("java/lang/System", "err")
        })?;
        field_ref
    };

    field_ref.set_raw_value(vec![print_stream_ref])
}

pub(crate) fn system_map_library_name_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let name_ref = args[0];
    let library_name_ref = map_library_name(name_ref)?;

    Ok(vec![library_name_ref])
}
fn map_library_name(name_ref: i32) -> crate::error::Result<i32> {
    let name = get_utf8_string_by_ref(name_ref)?;
    let library_name = libloading::library_filename(name).into_string()?;
    let library_name_ref = StringPoolHelper::get_string(library_name.to_string())?;

    Ok(library_name_ref)
}
