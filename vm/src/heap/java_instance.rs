use crate::error::Error;
use crate::method_area::field::Field;
use crate::method_area::java_class::JavaClass;
use crate::util::get_fields;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct JavaInstance<'a> {
    #[allow(dead_code)]
    class_ref: &'a JavaClass,
    fields: HashMap<String, Field>,
}

#[derive(Debug)]
pub(crate) struct Array {
    data: Vec<Vec<i32>>,
}

impl Array {
    pub fn new(len: i32) -> Self {
        Self {
            data: vec![vec![0, 0]; len as usize], //todo: use either 1 or 2 elements vector for corresponding type
        }
    }

    pub fn get_value(&self, index: i32) -> crate::error::Result<&Vec<i32>> {
        if let Some(arr_value) = self.data.get(index as usize) {
            Ok(arr_value)
        } else {
            Err(Error::new_execution("error getting array value"))
        }
    }

    pub fn set_value(&mut self, index: i32, value: Vec<i32>) -> crate::error::Result<()> {
        if let Some(arr_value) = self.data.get_mut(index as usize) {
            *arr_value = value;
            Ok(())
        } else {
            Err(Error::new_execution("error setting array value"))
        }
    }

    pub fn get_length(&self) -> i32 {
        self.data.len() as i32
    }
}

#[derive(Debug)]
pub(crate) enum HeapValue<'a> {
    Object(JavaInstance<'a>),
    Arr(Array),
}

impl<'a> JavaInstance<'a> {
    pub fn new(class_ref: &'a JavaClass) -> crate::error::Result<Self> {
        Ok(Self {
            class_ref,
            fields: get_fields(&class_ref.class_file)?,
        })
    }

    pub fn set_field_value(
        &mut self,
        fieldname: &str,
        value: Vec<i32>,
    ) -> crate::error::Result<()> {
        self.fields
            .get_mut(fieldname)
            .and_then(|v| Some(v.set_raw_value(value)))
            .ok_or(Error::new_execution("error setting instance field value"))
    }

    pub fn get_field_value(&self, fieldname: &str) -> crate::error::Result<&Vec<i32>> {
        self.fields
            .get(fieldname)
            .and_then(|v| Some(v.raw_value()))
            .ok_or(Error::new_execution("error getting instance field value"))
    }
}
