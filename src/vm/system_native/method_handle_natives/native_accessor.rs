use crate::vm::error::Error;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::with_heap_read_lock;
use crate::vm::heap::java_instance::Array;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::stack::stack_value::StackValueKind;

pub fn native_accessor_invoke0(
    method_ref: i32,
    obj_ref: i32,
    args_ref: i32,
) -> crate::vm::error::Result<i32> {
    let (clazz_ref, slot, entire_array_args) = with_heap_read_lock(|heap| {
        let method_name = heap.get_instance_name(method_ref)?;
        let clazz_ref = heap.get_object_field_value(method_ref, method_name.as_str(), "clazz")?[0];
        let slot = heap.get_object_field_value(method_ref, method_name.as_str(), "slot")?[0];

        let entire_array_args = heap.get_entire_array(args_ref)?;

        Ok::<(i32, i32, Array), Error>((clazz_ref, slot, entire_array_args))
    })?;

    let jc = with_method_area(|a| {
        let clazz_name = a.get_from_reflection_table(clazz_ref)?;
        a.get(&clazz_name)
    })?;

    let method = jc.get_method_by_index(slot as i64)?;
    let entire_value = entire_array_args.get_entire_value();
    let args = entire_value
        .iter()
        .map(|v| v[0].into())
        .collect::<Vec<StackValueKind>>();

    let ret = if obj_ref == 0 {
        Executor::invoke_static_method(method.class_name(), method.name_signature(), &args)?[0]
    } else {
        todo!("Handle non-static method invocation in native_accessor_invoke0");
    };

    Ok(ret)
}
