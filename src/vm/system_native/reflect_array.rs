use crate::vm::error::Result;
use crate::vm::heap::heap::with_heap_write_lock;
use crate::vm::helper::decorate;
use crate::vm::method_area::method_area::with_method_area;

pub(crate) fn new_array_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let component_type_clazz_ref = args[0];
    let length = args[1];

    let array_ref = new_array(component_type_clazz_ref, length)?;

    Ok(vec![array_ref])
}
fn new_array(component_type_clazz_ref: i32, length: i32) -> Result<i32> {
    let class_name = with_method_area(|method_area| {
        method_area.get_from_reflection_table(component_type_clazz_ref)
    })?;
    let decorated = decorate(class_name);
    let decorated_array = format!("[{decorated}");
    let arr_ref = with_heap_write_lock(|heap| heap.create_array(&decorated_array, length));
    Ok(arr_ref)
}
