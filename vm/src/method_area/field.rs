use crate::helper::default_value;
use jdescriptor::TypeDescriptor;
use serde::Serialize;
use std::sync::RwLock;

#[derive(Debug, Serialize)]
pub(crate) struct Field {
    type_descriptor: TypeDescriptor,
    value: RwLock<Vec<i32>>,
}

impl Field {
    pub fn new(type_descriptor: TypeDescriptor) -> crate::error::Result<Self> {
        let value = default_value(&type_descriptor)?;
        Ok(Self {
            type_descriptor,
            value: RwLock::new(value),
        })
    }

    pub fn set_raw_value(&self, value: Vec<i32>) -> crate::error::Result<()> {
        let mut guard = self.value.write()?;
        *guard = value;
        Ok(())
    }

    pub fn raw_value(&self) -> crate::error::Result<Vec<i32>> {
        let guard = self.value.read()?;
        Ok(guard.clone())
    }

    pub fn type_descriptor(&self) -> &TypeDescriptor {
        &self.type_descriptor
    }
}

impl Clone for Field {
    fn clone(&self) -> Self {
        Self {
            type_descriptor: self.type_descriptor.clone(),
            value: RwLock::new(
                self.value
                    .read()
                    .expect("error getting lock to clone field")
                    .clone(),
            ),
        }
    }
}
