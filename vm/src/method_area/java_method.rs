use crate::method_area::signature::Signature;
use crate::stack::stack_frame::StackFrame;

#[derive(Debug)]
pub(crate) struct JavaMethod {
    signature: Signature,
    max_stack: u16,
    max_locals: u16,
    bytecode: Vec<u8>,
    class_name: String,
}

impl JavaMethod {
    pub fn new(
        signature: Signature,
        max_stack: u16,
        max_locals: u16,
        bytecode: Vec<u8>,
        class_name: &str,
    ) -> Self {
        Self {
            signature,
            max_stack,
            max_locals,
            bytecode,
            class_name: class_name.to_string(),
        }
    }

    pub fn new_stack_frame(&self) -> StackFrame {
        StackFrame::new(
            self.max_locals as usize,
            self.max_stack as usize,
            &self.bytecode,
            self.class_name.clone(),
        )
    }

    pub fn get_signature(&self) -> &Signature {
        &self.signature
    }
}
