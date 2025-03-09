use crate::helper::default_value;
use crate::method_area::java_class::FieldProperty;
use jdescriptor::TypeDescriptor;
use serde::Serialize;
use std::sync::RwLock;

#[derive(Debug, Serialize)]
pub(crate) struct Field {
    type_descriptor: TypeDescriptor,
    value: RwLock<Vec<i32>>,
    flags: u16,
}

impl Field {
    pub fn new(type_descriptor: TypeDescriptor, flags: u16) -> crate::error::Result<Self> {
        let value = default_value(&type_descriptor)?;
        Ok(Self {
            type_descriptor,
            value: RwLock::new(value),
            flags,
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

    pub fn flags(&self) -> u16 {
        self.flags
    }
}

impl TryFrom<&FieldProperty> for Field {
    type Error = crate::error::Error;

    fn try_from(property: &FieldProperty) -> crate::error::Result<Field> {
        let value = default_value(&property.type_descriptor())?;
        Ok(Field {
            type_descriptor: property.type_descriptor().clone(),
            value: RwLock::new(value),
            flags: *property.flags(),
        })
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
            flags: self.flags,
        }
    }
}
