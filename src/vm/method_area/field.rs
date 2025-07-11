use crate::vm::error::Result;
use crate::vm::helper::default_value;
use derive_new::new;
use getset::{CopyGetters, Getters};
use jdescriptor::TypeDescriptor;
use serde::Serialize;
use std::sync::RwLock;

#[derive(Debug, Serialize, new, Getters, CopyGetters)]
pub(crate) struct FieldInfo {
    #[get = "pub"]
    type_descriptor: TypeDescriptor,
    #[get_copy = "pub"]
    flags: u16,
}

#[derive(Debug, Serialize)]
pub(crate) struct FieldValue {
    value: RwLock<Vec<i32>>,
}

impl FieldValue {
    pub fn new(type_descriptor: TypeDescriptor) -> Result<Self> {
        let value = default_value(&type_descriptor)?;
        Ok(Self {
            value: RwLock::new(value),
        })
    }

    pub fn set_raw_value(&self, value: Vec<i32>) -> Result<()> {
        let mut guard = self.value.write()?;
        *guard = value;
        Ok(())
    }

    pub fn raw_value(&self) -> Result<Vec<i32>> {
        let guard = self.value.read()?;
        Ok(guard.clone())
    }
}

impl Clone for FieldValue {
    fn clone(&self) -> Self {
        Self {
            value: RwLock::new(self.value.read().unwrap().clone()),
        }
    }
}
