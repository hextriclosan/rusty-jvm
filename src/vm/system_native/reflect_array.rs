use crate::vm::error::Result;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{decorate, klass};

pub(crate) fn new_array_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let component_type_clazz_ref = args[0];
    let length = args[1];

    let array_ref = new_array(component_type_clazz_ref, length)?;

    Ok(vec![array_ref])
}
fn new_array(component_type_clazz_ref: i32, length: i32) -> Result<i32> {
    let klass = klass(component_type_clazz_ref)?;
    let class_name = klass.this_class_name().to_owned();
    let decorated = decorate(class_name);
    let decorated_array = format!("[{decorated}");
    let arr_ref = HEAP.create_array(&decorated_array, length);
    Ok(arr_ref)
}
