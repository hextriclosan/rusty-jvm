use jdescriptor::{default_value, TypeDescriptor};
use serde::Serialize;
use std::sync::RwLock;

#[derive(Debug, Serialize)]
pub(crate) struct Field {
    type_descriptor: TypeDescriptor,
    value: RwLock<Vec<i32>>,
}

impl Field {
    pub fn new(type_descriptor: TypeDescriptor) -> Self {
        let value = default_value(&type_descriptor);
        Self {
            type_descriptor,
            value: RwLock::new(value),
        }
    }

    pub fn set_raw_value(&self, value: Vec<i32>) {
        let mut guard = self
            .value
            .write()
            .expect("error getting lock to set field value");
        *guard = value;
    }

    pub fn raw_value(&self) -> Vec<i32> {
        let guard = self
            .value
            .read()
            .expect("error getting lock to get field value");
        guard.clone()
    }

    pub fn type_descriptor(&self) -> &TypeDescriptor {
        &self.type_descriptor
    }
}
