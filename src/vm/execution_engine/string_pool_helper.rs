use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::stack::rooted_slots::RootedSlots;
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_value::StackValueKind;

pub struct StringPoolHelper {}

impl StringPoolHelper {
    pub(crate) fn get_string(cpool_string: &str) -> Result<i32> {
        if let Some(value) = HEAP.get_const_string_ref(cpool_string) {
            return Ok(value);
        }
        // todo: possible race condition here
        let string = Self::create_string(cpool_string)?;
        HEAP.put_const_string_ref(cpool_string, string[0].value());
        Ok(string[0].value())
    }

    fn create_string(string: &str) -> Result<RootedSlots> {
        if string.is_empty() {
            return Self::create_empty_string();
        }

        let codepoints = Self::string_to_codepoints(string);
        let array_ref = HEAP.create_array_with_values("[I", &codepoints);
        let _array = JavaThread::hold_temp_roots(vec![Slot::Ref(array_ref)]);

        let args = vec![
            StackValueKind::Ref(array_ref),
            0.into(),
            (codepoints.len() as i32).into(),
        ];
        let string_instance_ref = Executor::invoke_args_constructor(
            "java/lang/String",
            "<init>:([III)V",
            &args,
            Some(string),
        )?;

        Ok(string_instance_ref)
    }

    // todo: consider creating all CPool strings like this
    fn create_empty_string() -> Result<RootedSlots> {
        let byte_array_ref = HEAP.create_array_with_values("[B", &[]);
        let _array = JavaThread::hold_temp_roots(vec![Slot::Ref(byte_array_ref)]);
        let args = vec![
            StackValueKind::Ref(byte_array_ref),
            0.into(), /*coder LATIN1*/
        ];
        let string_instance_ref = Executor::invoke_args_constructor(
            "java/lang/String",
            "<init>:([BB)V",
            &args,
            Some(""),
        )?;
        Ok(string_instance_ref)
    }

    fn string_to_codepoints(s: &str) -> Vec<i32> {
        s.chars().map(|c| c as i32).collect()
    }
}
