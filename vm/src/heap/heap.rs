use crate::error::Error;
use crate::heap::java_instance::HeapValue::{Arr, Object};
use crate::heap::java_instance::{Array, HeapValue, JavaInstance};
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Heap<'a> {
    data: HashMap<i32, HeapValue<'a>>,
    next_id: i32,
}

impl<'a> Heap<'a> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn create_instance(&mut self, java_instance: JavaInstance<'a>) -> i32 {
        self.next_id = self.next_id + 1; //todo: make me atomic

        self.data.insert(self.next_id, Object(java_instance));

        self.next_id
    }

    pub fn set_object_field_value(
        &mut self,
        objectref: i32,
        fieldname: &str,
        value: Vec<i32>,
    ) -> crate::error::Result<()> {
        if let Some(Object(instance)) = self.data.get_mut(&objectref) {
            instance.set_field_value(fieldname, value)?;
            Ok(())
        } else {
            Err(Error::new_execution("error setting field value"))
        }
    }

    pub fn get_object_field_value(
        &mut self,
        objectref: i32,
        fieldname: &str,
    ) -> crate::error::Result<&Vec<i32>> {
        if let Some(Object(java_instance)) = self.data.get(&objectref) {
            java_instance.get_field_value(fieldname)
        } else {
            Err(Error::new_execution("error getting field value"))
        }
    }

    pub(crate) fn create_array(&mut self, len: i32) -> i32 {
        self.next_id = self.next_id + 1; //todo: make me atomic

        self.data.insert(self.next_id, Arr(Array::new(len)));

        self.next_id
    }

    pub(crate) fn get_array_value(
        &self,
        arrayref: i32,
        index: i32,
    ) -> crate::error::Result<&Vec<i32>> {
        if let Some(Arr(arr)) = self.data.get(&arrayref) {
            arr.get_value(index)
        } else {
            Err(Error::new_execution("error getting array value"))
        }
    }

    pub(crate) fn set_array_value(
        &mut self,
        arrayref: i32,
        index: i32,
        value: Vec<i32>,
    ) -> crate::error::Result<()> {
        if let Some(Arr(arr)) = self.data.get_mut(&arrayref) {
            arr.set_value(index, value)
        } else {
            Err(Error::new_execution("error setting array value"))
        }
    }

    pub(crate) fn get_array_len(&self, arrayref: i32) -> crate::error::Result<i32> {
        if let Some(Arr(arr)) = self.data.get(&arrayref) {
            Ok(arr.get_length())
        } else {
            Err(Error::new_execution("error getting array length"))
        }
    }
}
