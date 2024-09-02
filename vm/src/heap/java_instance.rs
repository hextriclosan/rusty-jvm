use crate::error::Error;
use crate::method_area::java_class::JavaClass;
use crate::util::get_fields;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct JavaInstance<'a> {
    #[allow(dead_code)]
    class_ref: &'a JavaClass,
    fields: HashMap<String, Field>,
}

impl<'a> JavaInstance<'a> {
    pub fn new(class_ref: &'a JavaClass) -> crate::error::Result<Self> {
        Ok(Self {
            class_ref,
            fields: get_fields(&class_ref.class_file)?,
        })
    }

    pub fn set_field_value(&mut self, fieldname: &str, value: i32) -> crate::error::Result<()> {
        self.fields
            .get_mut(fieldname)
            .and_then(|v| Some(v.set_value(value)))
            .ok_or(Error::new_execution("error setting instance field value"))
    }

    pub fn get_field_value(&self, fieldname: &str) -> crate::error::Result<i32> {
        self.fields
            .get(fieldname)
            .and_then(|v| Some(v.value))
            .ok_or(Error::new_execution("error getting instance field value"))
    }
}

#[derive(Debug)]
pub(crate) struct Field {
    value: i32, // todo: support other types
}

impl Field {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}
