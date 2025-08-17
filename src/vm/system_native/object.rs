use crate::vm::error::Result;
use crate::vm::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::vm::method_area::method_area::with_method_area;
use murmur3::murmur3_32;
use std::io::Cursor;

pub(crate) fn get_class_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let class_ref = get_class(obj_ref)?;

    Ok(vec![class_ref])
}
fn get_class(obj_ref: i32) -> Result<i32> {
    let instance_name = with_heap_read_lock(|heap| heap.get_instance_name(obj_ref))?;

    let reflection_ref =
        with_method_area(|method_area| method_area.load_reflection_class(&instance_name))?;

    Ok(reflection_ref)
}

pub(crate) fn clone_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let cloned_obj_ref = clone(obj_ref)?;

    Ok(vec![cloned_obj_ref])
}
fn clone(obj_ref: i32) -> Result<i32> {
    let cloned_obj_ref = with_heap_write_lock(|heap| heap.clone_instance(obj_ref))?;

    Ok(cloned_obj_ref)
}

pub(crate) fn object_hashcode_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let obj_ref = args[0];
    let hashcode = identity_hashcode(obj_ref)?;

    Ok(vec![hashcode])
}
pub(crate) fn identity_hashcode(obj_ref: i32) -> Result<i32> {
    if obj_ref == 0 {
        return Ok(0);
    }

    let mut cursor = Cursor::new(obj_ref.to_le_bytes());
    let hashcode = murmur3_32(&mut cursor, 0)?;
    Ok(hashcode as i32)
}
