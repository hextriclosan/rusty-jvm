use crate::vm::commons::auto_dash_map::auto_dash_map::AutoDashMap;
use crate::vm::commons::auto_dash_map::auto_dash_map_i32::AutoDashMapI32;
use crate::vm::error::{Error, Result};
use crate::vm::heap::java_instance::HeapValue::{Arr, Object};
use crate::vm::heap::java_instance::{Array, HeapValue, JavaInstance};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::sync::{LazyLock, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

static HEAP: LazyLock<RwLock<Heap>> = LazyLock::new(RwLock::default);

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

#[derive(Debug, Serialize, Default)]
pub(crate) struct Heap {
    data: AutoDashMapI32<HeapValue>,
    ref_by_stringvalue: HashMap<String, i32>,
}

impl Heap {
    pub fn create_instance(&mut self, java_instance: JavaInstance) -> i32 {
        self.data.insert_auto(Object(java_instance))
    }

    pub fn set_object_field_value(
        &mut self,
        objectref: i32,
        class_name: &str,
        field_name_type: &str,
        value: Vec<i32>,
    ) -> Result<()> {
        if objectref == 0 {
            return Err(Error::new_execution(&format!(
                "error setting field value: {class_name} to null-object"
            ))); // throw an appropriate exception here
        }

        self.data.get_mut(objectref)
            .and_then(|mut entry| {
                match entry.value_mut() {
                    Object(instance) => {
                        instance.set_field_value(class_name, field_name_type, value.clone()).ok()?;
                        Some(())
                    },
                    _ => None,
                }
            })
            .ok_or_else(|| Error::new_execution(&format!("error setting field value: objectref={objectref} class_name={class_name}, field_name_type={field_name_type}, value={value:?}")))
    }

    pub fn get_object_field_value(
        &self,
        objectref: i32,
        class_name: &str,
        field_name_type: &str,
    ) -> Result<Vec<i32>> {
        if objectref == 0 {
            return Err(Error::new_execution(&format!(
                "error getting field value: {class_name} from null-object"
            ))); // throw an appropriate exception here
        }

        self.data.get(objectref)
            .and_then(|entry| {
                match entry.value() {
                    Object(java_instance) => {
                        let ret = java_instance.get_field_value(class_name, field_name_type).ok()?;
                        Some(ret)
                    },
                    _ => None,
                }
            })
            .ok_or_else(|| Error::new_execution(&format!("error getting field value: objectref={objectref} class_name={class_name}, field_name_type={field_name_type}")))
    }

    pub fn get_instance_name(&self, objectref: i32) -> Result<String> {
        self.data
            .get(objectref)
            .and_then(|entry| match entry.value() {
                Object(java_instance) => {
                    let name = java_instance.instance_name().to_string();
                    Some(name)
                }
                Arr(array) => {
                    let name = array.type_name().to_string();
                    Some(name)
                }
            })
            .ok_or_else(|| {
                Error::new_execution(&format!(
                    "error getting object from heap by ref {objectref}"
                ))
            })
    }

    pub(crate) fn create_array(&mut self, type_name: &str, len: i32) -> i32 {
        self.data.insert_auto(Arr(Array::new(type_name, len)))
    }

    pub(crate) fn create_array_with_values(&mut self, type_name: &str, array: &[i32]) -> i32 {
        self.data
            .insert_auto(Arr(Array::new_with_values(type_name, array)))
    }

    pub(crate) fn get_entire_array(&self, array_ref: i32) -> Result<Array> {
        if array_ref == 0 {
            return Err(Error::new_execution(
                "NullPointerException: null array reference",
            )); //todo: throw an appropriate exception here
        }
        self.data
            .get(array_ref)
            .and_then(|entry| match entry.value() {
                Arr(array) => {
                    let cloned = array.clone();
                    Some(cloned)
                }
                _ => None,
            })
            .ok_or_else(|| Error::new_execution("error getting array"))
    }

    pub(crate) fn set_entire_array(&mut self, array_ref: i32, array_val: Array) -> Result<()> {
        self.data
            .get_mut(array_ref)
            .and_then(|mut entry| match entry.value_mut() {
                Arr(array) => {
                    array.set_entire_value(array_val).ok()?;
                    Some(())
                }
                _ => None,
            })
            .ok_or_else(|| Error::new_execution("error getting array"))
    }

    pub(crate) fn get_array_value(&self, arrayref: i32, index: i32) -> Result<Vec<i32>> {
        self.data
            .get(arrayref)
            .and_then(|entry| match entry.value() {
                Arr(arr) => {
                    let value = arr.get_value(index).ok()?;
                    Some(value)
                }
                _ => None,
            })
            .ok_or_else(|| Error::new_execution("error getting array value from heap"))
    }

    pub(crate) fn get_array_value_by_raw_offset(
        &self,
        arrayref: i32,
        offset: usize,
        len: usize,
    ) -> Result<Vec<i32>> {
        self.data
            .get(arrayref)
            .and_then(|entry| match entry.value() {
                Arr(arr) => {
                    let value = arr.get_value_by_raw_offset(offset, len).ok()?;
                    Some(value)
                }
                _ => None,
            })
            .ok_or_else(|| Error::new_execution("error getting array value from heap"))
    }

    pub(crate) fn set_array_value(
        &mut self,
        arrayref: i32,
        index: i32,
        value: Vec<i32>,
    ) -> Result<()> {
        self.data
            .get_mut(arrayref)
            .and_then(|mut entry| match entry.value_mut() {
                Arr(arr) => {
                    arr.set_value(index, value).ok()?;
                    Some(())
                }
                _ => None,
            })
            .ok_or_else(|| Error::new_execution("error setting array value"))
    }

    pub(crate) fn set_array_value_by_raw_offset(
        &mut self,
        arrayref: i32,
        offset: usize,
        value: Vec<i32>,
    ) -> Result<()> {
        self.data
            .get_mut(arrayref)
            .and_then(|mut entry| match entry.value_mut() {
                Arr(arr) => {
                    arr.set_array_value_by_raw_offset(offset, value).ok()?;
                    Some(())
                }
                _ => None,
            })
            .ok_or_else(|| Error::new_execution("error setting array value"))
    }

    pub(crate) fn get_array_len(&self, arrayref: i32) -> Result<i32> {
        self.data
            .get(arrayref)
            .and_then(|entry| match entry.value() {
                Arr(arr) => {
                    let len = arr.get_length();
                    Some(len)
                }
                _ => None,
            })
            .ok_or_else(|| Error::new_execution(&format!("error getting array length by ref={arrayref}")))
    }

    pub(crate) fn get_const_string_ref(&self, string: &str) -> Option<i32> {
        self.ref_by_stringvalue.get(string).map(|v| *v)
    }

    pub(crate) fn put_const_string_ref(&mut self, string: &str, reference: i32) -> Option<i32> {
        self.ref_by_stringvalue
            .insert(string.to_string(), reference)
    }

    #[allow(dead_code)]
    pub(crate) fn dump(&self) -> Result<()> {
        let json_string = serde_json::to_string(self)?;
        let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        File::create_new(format!("heap_dump_{ts}.json")).and_then(|mut file| {
            use std::io::Write;
            file.write_all(json_string.as_bytes())
        })?;

        Ok(())
    }

    pub(crate) fn clone_instance(&mut self, objectref: i32) -> Result<i32> {
        let cloned_heap_value = {
            self.data
                .get(objectref)
                .and_then(|entry| Some(entry.value().clone()))
                .ok_or_else(|| {
                    Error::new_execution(&format!(
                        "error getting object by ref {objectref} for cloning"
                    ))
                })
        }?;

        let cloned_ref = match cloned_heap_value {
            Object(new_instance) => self.create_instance(new_instance),
            Arr(new_array) => {
                let new_array_ref =
                    self.create_array(new_array.type_name(), new_array.get_length());
                self.set_entire_array(new_array_ref, new_array)?;
                new_array_ref
            }
        };

        Ok(cloned_ref)
    }
}
