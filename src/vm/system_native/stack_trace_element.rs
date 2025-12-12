use crate::vm::error::Result;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{klass, vec_to_i64};
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::system_native::throwable::NATIVE_METHOD;

const NATIVE_MARKER: i32 = -2;
const CLASS_NAME: &'static str = "java/lang/StackTraceElement";

pub(crate) fn init_stack_trace_elements_wrp(args: &[i32]) -> Result<Vec<i32>> {
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
) -> Result<()> {
    // extract arrays from Object[] Throwable.backtrace
    let class_array_ref = HEAP.get_array_value(backtrace_ref, 0)?[0];
    let method_array_ref = HEAP.get_array_value(backtrace_ref, 1)?[0];
    let line_array_ref = HEAP.get_array_value(backtrace_ref, 2)?[0];
    let tag_array_ref = HEAP.get_array_value(backtrace_ref, 3)?[0];

    for index in 0..depth {
        let class_ref = HEAP.get_array_value(class_array_ref, index)?[0];

        let klass = klass(class_ref)?;
        let class_name_ref = {
            let external_name = klass.external_name();
            StringPoolHelper::get_string(external_name)?
        };

        let method_name_ref = {
            let method_ref = vec_to_i64(&HEAP.get_array_value(method_array_ref, index)?);
            let java_method = method_ref as *const JavaMethod;
            let method_name = unsafe { (*java_method).name() };
            StringPoolHelper::get_string(method_name)?
        };

        let file_name_ref = {
            let source_file = klass.source_file().as_deref().unwrap_or("Unknown");
            StringPoolHelper::get_string(source_file)?
        };

        let line_number = {
            let tag = HEAP.get_array_value(tag_array_ref, index)?[0];
            if tag == NATIVE_METHOD {
                NATIVE_MARKER
            } else {
                HEAP.get_array_value(line_array_ref, index)?[0]
            }
        };

        let element_ref = HEAP.get_array_value(element_array_ref, index)?[0];
        HEAP.set_object_field_value(
            element_ref,
            CLASS_NAME,
            "declaringClassObject",
            vec![class_ref],
        )?;
        HEAP.set_object_field_value(
            element_ref,
            CLASS_NAME,
            "declaringClass",
            vec![class_name_ref],
        )?;
        HEAP.set_object_field_value(element_ref, CLASS_NAME, "methodName", vec![method_name_ref])?;
        HEAP.set_object_field_value(element_ref, CLASS_NAME, "fileName", vec![file_name_ref])?;
        HEAP.set_object_field_value(
            element_ref,
            CLASS_NAME,
            "lineNumber",
            vec![line_number.into()],
        )?;
    }

    Ok(())
}
