use crate::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::method_area::method_area::with_method_area;

pub(crate) fn get_class_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let obj_ref = args[0];
    let class_ref = get_class(obj_ref)?;

    Ok(vec![class_ref])
}
fn get_class(obj_ref: i32) -> crate::error::Result<i32> {
    let instance_name = with_heap_read_lock(|heap| heap.get_instance_name(obj_ref))?;

    let reflection_ref =
        with_method_area(|method_area| method_area.load_reflection_class(&instance_name))?;

    Ok(reflection_ref)
}

pub(crate) fn clone_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let obj_ref = args[0];
    let cloned_obj_ref = clone(obj_ref)?;

    Ok(vec![cloned_obj_ref])
}
fn clone(obj_ref: i32) -> crate::error::Result<i32> {
    let cloned_obj_ref = with_heap_write_lock(|heap| heap.clone_instance(obj_ref))?[0];

    Ok(cloned_obj_ref)
}
