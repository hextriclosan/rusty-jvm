use crate::error::Error;
use crate::method_area::field::Field;
use crate::method_area::java_class::JavaClass;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct JavaInstance {
    _class_ref: Rc<JavaClass>, // todo use me or delete
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
pub(crate) enum HeapValue {
    Object(JavaInstance),
    Arr(Array),
}

impl<'a> JavaInstance {
    pub fn new(class_ref: Rc<JavaClass>) -> crate::error::Result<Self> {
        Ok(Self {
            _class_ref: Rc::clone(&class_ref),
            fields: class_ref //todo: refactor me: remove static fields, put this code in right place
                .field_descriptors
                .descriptor_by_name
                .iter()
                .map(|(name, descriptor)| (name.clone(), Field::new(descriptor.clone())))
                .collect(),
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
            .ok_or_else(|| {
                Error::new_execution(&format!(
                    "error setting value for instance field {fieldname}"
                ))
            })
    }

    pub fn get_field_value(&self, fieldname: &str) -> crate::error::Result<&Vec<i32>> {
        self.fields
            .get(fieldname)
            .and_then(|v| Some(v.raw_value()))
            .ok_or(Error::new_execution("error getting instance field value"))
    }
}
