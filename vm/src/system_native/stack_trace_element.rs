use crate::error::Error;
use crate::execution_engine::string_pool_helper::StringPoolHelper;
use crate::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::helper::vec_to_i64;
use crate::method_area::java_method::JavaMethod;
use crate::method_area::method_area::with_method_area;
use crate::system_native::throwable::NATIVE_METHOD;

const NATIVE_MARKER: i32 = -2;
const CLASS_NAME: &'static str = "java/lang/StackTraceElement";

pub(crate) fn init_stack_trace_elements_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let element_array_ref = args[0];
    let backtrace_ref = args[1];
    let depth = args[2];

    init_stack_trace_elements(element_array_ref, backtrace_ref, depth)?;
    Ok(vec![])
}
fn init_stack_trace_elements(
    element_array_ref: i32,
    backtrace_ref: i32,
    depth: i32,
) -> crate::error::Result<()> {
    // extract arrays from Object[] Throwable.backtrace
    let (class_array_ref, method_array_ref, line_array_ref, tag_array_ref) =
        with_heap_read_lock(|heap| {
            let class_array_ref = heap.get_array_value(backtrace_ref, 0)?[0];
            let method_array_ref = heap.get_array_value(backtrace_ref, 1)?[0];
            let line_array_ref = heap.get_array_value(backtrace_ref, 2)?[0];
            let tag_array_ref = heap.get_array_value(backtrace_ref, 3)?[0];
            Ok::<_, Error>((
                class_array_ref,
                method_array_ref,
                line_array_ref,
                tag_array_ref,
            ))
        })?;

    for index in 0..depth {
        let class_ref =
            with_heap_read_lock(|heap| heap.get_array_value(class_array_ref, index))?[0];

        let jc = {
            let class_name = with_method_area(|area| area.get_from_reflection_table(class_ref))?;
            with_method_area(|area| area.get(&class_name))?
        };

        let class_name_ref = {
            let external_name = jc.external_name();
            StringPoolHelper::get_string(external_name.to_string())?
        };

        let method_name_ref = {
            let method_ref = vec_to_i64(&with_heap_read_lock(|heap| {
                heap.get_array_value(method_array_ref, index)
            })?);
            let java_method = method_ref as *const JavaMethod;
            let method_name = unsafe { (*java_method).name() };
            StringPoolHelper::get_string(method_name.to_string())?
        };

        let file_name_ref = {
            let source_file = jc.source_file().as_deref().unwrap_or("Unknown");
            StringPoolHelper::get_string(source_file.to_string())?
        };

        let line_number = {
            let tag = with_heap_read_lock(|heap| heap.get_array_value(tag_array_ref, index))?[0];
            if tag == NATIVE_METHOD {
                NATIVE_MARKER
            } else {
                with_heap_read_lock(|heap| heap.get_array_value(line_array_ref, index))?[0]
            }
        };

        with_heap_write_lock(|heap| {
            let element_ref = heap.get_array_value(element_array_ref, index)?[0];
            heap.set_object_field_value(
                element_ref,
                CLASS_NAME,
                "declaringClassObject",
                vec![class_ref],
            )?;
            heap.set_object_field_value(
                element_ref,
                CLASS_NAME,
                "declaringClass",
                vec![class_name_ref],
            )?;
            heap.set_object_field_value(
                element_ref,
                CLASS_NAME,
                "methodName",
                vec![method_name_ref],
            )?;
            heap.set_object_field_value(element_ref, CLASS_NAME, "fileName", vec![file_name_ref])?;
            heap.set_object_field_value(
                element_ref,
                CLASS_NAME,
                "lineNumber",
                vec![line_number.into()],
            )?;

            Ok::<(), Error>(())
        })?;
    }

    Ok(())
}
