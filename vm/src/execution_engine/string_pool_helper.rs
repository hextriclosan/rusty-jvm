use crate::error::Error;
use crate::execution_engine::engine::Engine;
use crate::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::method_area::method_area::with_method_area;

pub struct StringPoolHelper {}

impl StringPoolHelper {
    pub(crate) fn get_string(cpool_string: String) -> crate::error::Result<i32> {
        if let Some(value) = with_heap_read_lock(|heap| heap.get_const_string_ref(&cpool_string)) {
            return Ok(value);
        }
        // todo: possible race condition here
        let string_ref = Self::create_string(&cpool_string)?;
        with_heap_write_lock(|heap| heap.put_const_string_ref(&cpool_string, string_ref));
        Ok(string_ref)
    }

    fn create_string(string: &str) -> crate::error::Result<i32> {
        if string.is_empty() {
            return Self::create_empty_string();
        }

        let codepoints = Self::string_to_codepoints(&string);

        let array_ref =
            with_heap_write_lock(|heap| heap.create_array_with_values("[I", &codepoints));
        let string_class_name = "java/lang/String".to_string();
        let instance_with_default_fields = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields(&string_class_name)
        });

        let string_instance_ref =
            with_heap_write_lock(|heap| heap.create_instance(instance_with_default_fields));

        let full_signature = "<init>:([III)V";
        let rc = with_method_area(|method_area| method_area.get(string_class_name.as_str()))?;
        let special_method = rc
            .methods
            .method_by_signature
            .get(full_signature)
            .ok_or_else(|| Error::new_constant_pool(&format!("Error getting JavaMethod by class name {string_class_name} and full signature {full_signature} invoking special")))?;

        let mut engine = Engine::new();
        let mut stack_frame = special_method.new_stack_frame()?;
        stack_frame.set_local(0, string_instance_ref);
        stack_frame.set_local(1, array_ref);
        stack_frame.set_local(2, 0);
        stack_frame.set_local(3, codepoints.len() as i32);

        engine.execute(stack_frame, &format!("creating \"{string}\""))?;

        // todo: ensure that array_ref is collected by GC
        Ok(string_instance_ref)
    }

    // todo: consider creating all CPool strings like this
    fn create_empty_string() -> crate::error::Result<i32> {
        let byte_array_ref =
            with_heap_write_lock(|heap| heap.create_array_with_values("[b", &vec![]));
        let string_class_name = "java/lang/String".to_string();
        let instance_with_default_fields = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields(&string_class_name)
        });

        let string_instance_ref =
            with_heap_write_lock(|heap| heap.create_instance(instance_with_default_fields));

        let full_signature = "<init>:([BB)V";
        let rc = with_method_area(|method_area| method_area.get(string_class_name.as_str()))?;
        let special_method = rc
            .methods
            .method_by_signature
            .get(full_signature)
            .ok_or_else(|| Error::new_constant_pool(&format!("Error getting JavaMethod by class name {string_class_name} and full signature {full_signature} invoking special")))?;

        let mut engine = Engine::new();
        let mut stack_frame = special_method.new_stack_frame()?;
        stack_frame.set_local(0, string_instance_ref);
        stack_frame.set_local(1, byte_array_ref);
        stack_frame.set_local(2, 0); // coder LATIN1

        engine.execute(stack_frame, "creating \"\"")?;

        Ok(string_instance_ref)
    }

    fn string_to_codepoints(s: &str) -> Vec<i32> {
        s.chars().map(|c| c as i32).collect()
    }
}
