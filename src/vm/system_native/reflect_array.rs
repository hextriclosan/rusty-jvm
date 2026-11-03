use crate::vm::error::Result;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{decorate, klass};
use crate::vm::stack::rooted_slots::RootedSlots;

/// `java.lang.reflect.Array.newArray(Ljava/lang/Class;I)Ljava/lang/Object;`
pub(crate) fn new_array(component_type_clazz_ref: i32, length: i32) -> Result<i32> {
    let klass = klass(component_type_clazz_ref)?;
    let class_name = klass.this_class_name().to_owned();
    let decorated = decorate(class_name);
    let decorated_array = format!("[{decorated}");
    HEAP.create_array(&decorated_array, length, &mut RootedSlots::new())
}
