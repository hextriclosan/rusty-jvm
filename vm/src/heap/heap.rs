use crate::error::Error;
use crate::heap::java_instance::HeapValue::{Arr, Object};
use crate::heap::java_instance::{Array, HeapValue, JavaInstance};
use crate::method_area::method_area::with_method_area;
use indexmap::IndexMap;
use once_cell::sync::Lazy;
use serde::Serialize;
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

#[derive(Debug, Serialize)]
pub(crate) struct Heap {
    data: IndexMap<i32, HeapValue>,
    next_id: i32,
    ref_by_stringvalue: HashMap<String, i32>,
}

impl Heap {
    fn new() -> Self {
        Self {
            data: IndexMap::new(),
            next_id: 0,
            ref_by_stringvalue: HashMap::new(),
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
        &self,
        objectref: i32,
        class_name: &str,
        field_name_type: &str,
    ) -> crate::error::Result<Vec<i32>> {
        if let Some(Object(java_instance)) = self.data.get(&objectref) {
            java_instance.get_field_value(class_name, field_name_type)
        } else {
            Err(Error::new_execution(&format!(
                "error getting field value {class_name}.{field_name_type}"
            )))
        }
    }

    pub fn get_instance_name(&self, objectref: i32) -> crate::error::Result<String> {
        self.data.get(&objectref).map_or_else(
            || {
                Err(Error::new_execution(&format!(
                    "error getting object from heap by ref {objectref}"
                )))
            },
            |obj| match obj {
                Object(java_instance) => Ok(java_instance.instance_name().to_string()),
                Arr(array) => Ok(array.type_name().to_string()),
            },
        )
    }

    pub(crate) fn create_array(&mut self, type_name: &str, len: i32) -> i32 {
        self.next_id = self.next_id + 1; //todo: make me atomic

        //ensure creation of ephemeral array class
        with_method_area(|method_area| method_area.create_array_class_if_needed(type_name))
            .expect("error creating array class");

        self.data
            .insert(self.next_id, Arr(Array::new(type_name, len)));

        self.next_id
    }

    pub(crate) fn create_array_with_values(&mut self, type_name: &str, array: &[i32]) -> i32 {
        self.next_id = self.next_id + 1; //todo: make me atomic

        self.data
            .insert(self.next_id, Arr(Array::new_with_values(type_name, array)));

        self.next_id
    }

    pub(crate) fn get_entire_array(&self, array_ref: i32) -> crate::error::Result<Array> {
        if let Some(Arr(array)) = self.data.get(&array_ref) {
            Ok(array.clone())
        } else {
            Err(Error::new_execution("error getting array"))
        }
    }

    pub(crate) fn set_entire_array(
        &mut self,
        array_ref: i32,
        array_val: Array,
    ) -> crate::error::Result<()> {
        if let Some(Arr(array)) = self.data.get_mut(&array_ref) {
            array.set_entire_value(array_val)
        } else {
            Err(Error::new_execution("error getting array"))
        }
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

    pub(crate) fn get_const_string_ref(&self, string: &str) -> Option<i32> {
        self.ref_by_stringvalue.get(string).map(|v| *v)
    }

    pub(crate) fn put_const_string_ref(&mut self, string: &str, reference: i32) -> Option<i32> {
        self.ref_by_stringvalue
            .insert(string.to_string(), reference)
    }

    #[allow(dead_code)]
    pub(crate) fn dump(&self) -> crate::error::Result<()> {
        let json_string = serde_json::to_string(self)?;
        println!("HEAP DUMP: {json_string}");
        Ok(())
    }

    pub(crate) fn clone_instance(&mut self, objectref: i32) -> crate::error::Result<Vec<i32>> {
        if let Some(Object(instance)) = self.data.get(&objectref) {
            let new_instance = instance.clone();
            let new_instance_ref = self.create_instance(new_instance);
            Ok(vec![new_instance_ref])
        } else if let Some(Arr(array)) = self.data.get(&objectref) {
            let new_array = array.clone();
            let new_array_ref = self.create_array(new_array.type_name(), new_array.get_length());
            self.set_entire_array(new_array_ref, new_array)?;
            Ok(vec![new_array_ref])
        } else {
            Err(Error::new_execution(&format!(
                "error cloning object with ref {objectref}"
            )))
        }
    }
}
