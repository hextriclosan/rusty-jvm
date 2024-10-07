use crate::error::Error;
use crate::heap::java_instance::HeapValue::{Arr, Object};
use crate::heap::java_instance::{Array, HeapValue, JavaInstance};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

static HEAP: Lazy<RwLock<Heap>> = Lazy::new(|| RwLock::new(Heap::new()));

pub(crate) fn with_heap_read_lock<F, R>(f: F) -> R
where
    F: FnOnce(&Heap) -> R,
{
    let heap = HEAP.read().expect("error getting heap read lock");
    f(&heap)
}

pub(crate) fn with_heap_write_lock<F, R>(f: F) -> R
where
    F: FnOnce(&mut Heap) -> R,
{
    let mut heap = HEAP.write().expect("error getting heap write lock");
    f(&mut heap)
}

#[derive(Debug)]
pub(crate) struct Heap {
    data: HashMap<i32, HeapValue>,
    next_id: i32,
}

impl Heap {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn create_instance(&mut self, java_instance: JavaInstance) -> i32 {
        self.next_id = self.next_id + 1; //todo: make me atomic

        self.data.insert(self.next_id, Object(java_instance));

        self.next_id
    }

    pub fn set_object_field_value(
        &mut self,
        objectref: i32,
        class_name: &str,
        field_name_type: &str,
        value: Vec<i32>,
    ) -> crate::error::Result<()> {
        if let Some(Object(instance)) = self.data.get_mut(&objectref) {
            instance.set_field_value(class_name, field_name_type, value)?;
            Ok(())
        } else {
            Err(Error::new_execution("error setting field value"))
        }
    }

    pub fn get_object_field_value(
        &mut self,
        objectref: i32,
        class_name: &str,
        field_name_type: &str,
    ) -> crate::error::Result<Vec<i32>> {
        if let Some(Object(java_instance)) = self.data.get(&objectref) {
            java_instance.get_field_value(class_name, field_name_type)
        } else {
            Err(Error::new_execution("error getting field value"))
        }
    }

    pub fn get_instance_name(&self, objectref: i32) -> crate::error::Result<String> {
        if let Some(Object(java_instance)) = self.data.get(&objectref) {
            Ok(java_instance.instance_name().to_string())
        } else {
            Err(Error::new_execution(&format!(
                "error getting object from heap by ref {objectref}"
            )))
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
