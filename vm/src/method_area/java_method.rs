use crate::error::Error;
use crate::stack::stack_frame::StackFrame;
use jdescriptor::MethodDescriptor;

#[derive(Debug)]
pub(crate) struct JavaMethod {
    method_descriptor: MethodDescriptor,
    class_name: String,
    name_signature: String,
    code_context: Option<CodeContext>,
}

#[derive(Debug)]
pub(crate) struct CodeContext {
    max_stack: u16,
    max_locals: u16,
    bytecode: Vec<u8>,
}

impl CodeContext {
    pub fn new(max_stack: u16, max_locals: u16, bytecode: Vec<u8>) -> Self {
        Self {
            max_stack,
            max_locals,
            bytecode,
        }
    }

    pub fn max_stack(&self) -> u16 {
        self.max_stack
    }

    pub fn max_locals(&self) -> u16 {
        self.max_locals
    }

    pub fn bytecode(&self) -> &Vec<u8> {
        &self.bytecode
    }
}

impl JavaMethod {
    pub fn new(
        method_descriptor: MethodDescriptor,
        class_name: &str,
        name_signature: &str,
        code_context: Option<CodeContext>,
    ) -> Self {
        Self {
            method_descriptor,
            class_name: class_name.to_string(),
            name_signature: name_signature.to_string(),
            code_context,
        }
    }

    pub fn new_stack_frame(&self) -> crate::error::Result<StackFrame> {
        match &self.code_context {
            Some(context) => Ok(StackFrame::new(
                context.max_locals() as usize,
                context.max_stack() as usize,
                context.bytecode().to_owned(), //todo: use reference instead of coping
                self.class_name.clone(),
            )),
            None => Err(Error::new_execution(&format!(
                "Code context is missing for {}.{}",
                self.class_name, self.name_signature
            ))),
        }
    }

    pub fn get_method_descriptor(&self) -> &MethodDescriptor {
        &self.method_descriptor
    }
}
