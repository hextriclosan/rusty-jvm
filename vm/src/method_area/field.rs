use jdescriptor::{default_value, TypeDescriptor};

#[derive(Debug)]
pub(crate) struct Field {
    type_descriptor: TypeDescriptor,
    value: Vec<i32>,
}

impl Field {
    pub fn new(type_descriptor: TypeDescriptor) -> Self {
        let value = default_value(&type_descriptor);
        Self {
            type_descriptor,
            value,
        }
    }

    pub fn set_raw_value(&mut self, value: Vec<i32>) {
        self.value = value;
    }

    pub fn raw_value(&self) -> &Vec<i32> {
        &self.value
    }

    pub fn type_descriptor(&self) -> &TypeDescriptor {
        &self.type_descriptor
    }
}
