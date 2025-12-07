use crate::vm::error::{Error, Result};
use crate::vm::heap::java_instance::JavaInstance::{Base, Class};
use crate::vm::method_area::field::FieldValue;
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::loaded_classes::CLASSES;
use derive_new::new;
use indexmap::IndexMap;
use serde::Serialize;
use std::sync::Arc;

pub type ClassName = String;
pub type FieldNameType = String;

#[derive(Debug, Serialize, Clone, new)]
pub(crate) struct JavaInstanceBase {
    klass_id: usize,
    fields: IndexMap<ClassName, IndexMap<FieldNameType, FieldValue>>,
}

#[derive(Debug, Serialize, Clone, new)]
pub(crate) struct JavaInstanceClass {
    instance: JavaInstanceBase,
    mirror_klass_id: usize,
}

#[derive(Debug, Serialize, Clone)]
pub(crate) enum JavaInstance {
    Base(JavaInstanceBase),
    Class(JavaInstanceClass),
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Array {
    type_name: String,
    data: Vec<u8>,
}

impl Array {
    pub fn new(type_name: &str, len: i32) -> Self {
        Self {
            type_name: type_name.to_string(),
            data: Self::allocate_data(type_name, len as usize),
        }
    }

    fn allocate_data(type_name: &str, len: usize) -> Vec<u8> {
        vec![0; len * Self::bytes_size(type_name)]
    }

    fn bytes_size(type_name: &str) -> usize {
        match type_name {
            "[B" => 1,
            "[C" => 2,
            "[D" => 8,
            "[F" => 4,
            "[I" => 4,
            "[J" => 8,
            "[S" => 2,
            "[Z" => 1,
            _ => 4,
        }
    }

    pub fn new_with_values(type_name: &str, array: &[i32]) -> Self {
        let size = Self::bytes_size(type_name);
        let converted_arr: Vec<u8> = array
            .iter()
            .flat_map(|&value| {
                let bytes = value.to_ne_bytes();
                if cfg!(target_endian = "big") {
                    bytes[4 - size..4].to_vec()
                } else {
                    bytes[0..size].to_vec()
                }
            })
            .collect();

        Self {
            type_name: type_name.to_string(),
            data: converted_arr,
        }
    }

    pub fn get_value(&self, index: i32) -> Result<Vec<i32>> {
        let size = Self::bytes_size(&self.type_name);
        let offset = index as usize * size;

        self.get_value_by_raw_offset(offset, size)
    }

    pub fn get_value_by_raw_offset(&self, offset: usize, size: usize) -> Result<Vec<i32>> {
        let src = self.data.get(offset..offset + size)
            .ok_or_else(|| Error::new_execution(&format!(
                "get_value_by_raw_offset: offset out of bounds: offset={offset}, size={size}, data_len={}",
                self.data.len()
            )))?;

        match size {
            1 => Ok(vec![src[0] as i8 as i32]),
            2 => {
                let bytes: [u8; 2] = src.try_into()?;
                let value = if self.type_name == "[C" {
                    u16::from_ne_bytes(bytes) as i32
                } else {
                    i16::from_ne_bytes(bytes) as i32
                };
                Ok(vec![value])
            }
            4 => {
                let bytes: [u8; 4] = src.try_into()?;
                Ok(vec![i32::from_ne_bytes(bytes)])
            }
            8 => {
                let bytes: [u8; 8] = src.try_into()?;
                let (high, low) = bytes.split_at(4);
                let high = i32::from_ne_bytes(high.try_into()?);
                let low = i32::from_ne_bytes(low.try_into()?);
                if cfg!(target_endian = "big") {
                    Ok(vec![high, low])
                } else {
                    Ok(vec![low, high])
                }
            }
            _ => Err(Error::new_execution(&format!("Invalid size: {size}"))),
        }
    }

    pub fn set_value(&mut self, index: i32, value: Vec<i32>) -> Result<()> {
        let size = Self::bytes_size(&self.type_name);
        let offset = index as usize * size;

        self.set_array_value_by_raw_offset(offset, value, size)
    }

    pub fn set_array_value_by_raw_offset(
        &mut self,
        offset: usize,
        value: Vec<i32>,
        size: usize,
    ) -> Result<()> {
        let data_len = self.data.len();
        match size {
            1..=4 => {
                let bytes = value[0].to_ne_bytes();
                let slice = if cfg!(target_endian = "big") {
                    &bytes[4 - size..]
                } else {
                    &bytes[..size]
                };
                self.data
                    .get_mut(offset..offset + size)
                    .ok_or_else(|| Error::new_execution(&format!(
                        "set_array_value_by_raw_offset: offset out of bounds (offset={offset}, size={size}, data_len={data_len})"
                    )))?
                    .copy_from_slice(slice);
            }
            8 => {
                let mut buf = [0u8; 8];
                if cfg!(target_endian = "big") {
                    buf[..4].copy_from_slice(&value[0].to_ne_bytes());
                    buf[4..].copy_from_slice(&value[1].to_ne_bytes());
                } else {
                    buf[..4].copy_from_slice(&value[1].to_ne_bytes());
                    buf[4..].copy_from_slice(&value[0].to_ne_bytes());
                }

                self.data
                    .get_mut(offset..offset + 8)
                    .ok_or_else(|| Error::new_execution(&format!(
                        "set_array_value_by_raw_offset: offset out of bounds (offset={offset}, size=8, data_len={data_len})"
                    )))?
                    .copy_from_slice(&buf);
            }
            _ => return Err(Error::new_execution(&format!("Invalid size: {size}"))),
        }

        Ok(())
    }

    pub fn set_entire_value(&mut self, value: Array) -> Result<()> {
        self.data = value.data;
        Ok(())
    }

    pub fn get_entire_value(&self) -> Vec<Vec<i32>> {
        let size = Self::bytes_size(&self.type_name);
        let mut result = Vec::new();
        for i in 0..self.data.len() / size {
            let value = self.get_value(i as i32).unwrap();
            result.push(value);
        }
        result
    }

    pub fn get_length(&self) -> i32 {
        let size = Self::bytes_size(&self.type_name);
        let length = self.data.len() / size;
        length as i32
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    pub fn raw_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn raw_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
}

#[derive(Debug, Serialize, Clone)]
pub(crate) enum HeapValue {
    Object(JavaInstance),
    Arr(Array),
}

impl JavaInstance {
    pub fn set_field_value(
        &mut self,
        class_name: &str,
        field_name_type: &str,
        value: Vec<i32>,
    ) -> Result<()> {
        self.lookup_for_field_mut(class_name, field_name_type)
            .and_then(|v| Some(v.set_raw_value(value)))
            .ok_or_else(|| {
                Error::new_execution(&format!(
                    "error setting value for instance field {class_name}.{field_name_type}"
                ))
            })?
    }

    pub fn get_field_value(&self, class_name: &str, field_name_type: &str) -> Result<Vec<i32>> {
        self.lookup_for_field(class_name, field_name_type)
            .and_then(|v| Some(v.raw_value()))
            .ok_or(Error::new_execution(&format!(
                "error getting instance field value {class_name}.{field_name_type}"
            )))?
    }

    fn lookup_for_field(
        &self,
        starting_from_class_name: &str,
        field_name_type: &str,
    ) -> Option<&FieldValue> {
        let instance = self.get_instance();
        match instance.fields.get_index_of(starting_from_class_name) {
            Some(start_index) => instance
                .fields
                .iter()
                .take(start_index + 1)
                .rev()
                .find_map(|(_, map)| map.get(field_name_type)),
            None => None,
        }
    }

    fn get_instance(&self) -> &JavaInstanceBase {
        match self {
            Base(instance) => instance,
            Class(class_instance) => &class_instance.instance,
        }
    }

    fn get_instance_mut(&mut self) -> &mut JavaInstanceBase {
        match self {
            Base(ref mut instance) => instance,
            Class(ref mut class_instance) => &mut class_instance.instance,
        }
    }

    fn lookup_for_field_mut(
        &mut self,
        starting_from_class_name: &str,
        field_name_type: &str,
    ) -> Option<&mut FieldValue> {
        let instance = self.get_instance_mut();
        match instance.fields.get_index_of(starting_from_class_name) {
            Some(start_index) => instance
                .fields
                .iter_mut()
                .take(start_index + 1)
                .rev()
                .find_map(|(_, map)| map.get_mut(field_name_type)),
            None => None,
        }
    }

    pub fn klass(&self) -> Result<Arc<JavaClass>> {
        let instance = self.get_instance();
        CLASSES.get_by_id(instance.klass_id)
    }

    pub fn instance_name(&self) -> Result<String> {
        self.klass().map(|klass| klass.this_class_name().clone()) // fixme!!! cloning string
    }
}
