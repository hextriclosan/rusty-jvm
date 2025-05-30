use crate::vm::error::{Error, Result};
use crate::vm::method_area::field::Field;
use derive_new::new;
use indexmap::IndexMap;
use serde::Serialize;

pub type ClassName = String;
pub type FieldNameType = String;

#[derive(Debug, Serialize, Clone, new)]
pub(crate) struct JavaInstance {
    instance_name: String,
    fields: IndexMap<ClassName, IndexMap<FieldNameType, Field>>,
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
                bytes[0..size].to_vec()
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
        match size {
            1..=4 => {
                let mut buf = [0u8; 4];
                if cfg!(target_endian = "big") {
                    buf[4 - size..4].copy_from_slice(&self.data[offset..offset + size]);
                } else {
                    buf[0..size].copy_from_slice(&self.data[offset..offset + size]);
                }
                let value = i32::from_ne_bytes(buf);
                Ok(vec![value])
            }
            8 => {
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&self.data[offset..offset + size]);
                let high = i32::from_ne_bytes(buf[0..4].try_into().unwrap());
                let low = i32::from_ne_bytes(buf[4..8].try_into().unwrap());
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

        self.set_array_value_by_raw_offset(offset, value)
    }

    pub fn set_array_value_by_raw_offset(&mut self, offset: usize, value: Vec<i32>) -> Result<()> {
        let size = Self::bytes_size(&self.type_name);
        match size {
            1..=4 => {
                let int_buf = value[0].to_ne_bytes();
                if cfg!(target_endian = "big") {
                    self.data[offset..offset + size].copy_from_slice(&int_buf[4 - size..4]);
                } else {
                    self.data[offset..offset + size].copy_from_slice(&int_buf[0..size]);
                }
            }
            8 => {
                let mut buf = [0u8; 8];
                if cfg!(target_endian = "big") {
                    buf[0..4].copy_from_slice(&value[0].to_ne_bytes());
                    buf[4..8].copy_from_slice(&value[1].to_ne_bytes());
                } else {
                    buf[0..4].copy_from_slice(&value[1].to_ne_bytes());
                    buf[4..8].copy_from_slice(&value[0].to_ne_bytes());
                }
                self.data[offset..offset + size].copy_from_slice(&buf);
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
}

#[derive(Debug, Serialize)]
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
    ) -> Option<&Field> {
        match self.fields.get_index_of(starting_from_class_name) {
            Some(start_index) => self
                .fields
                .iter()
                .take(start_index + 1)
                .rev()
                .find_map(|(_, map)| map.get(field_name_type)),
            None => None,
        }
    }

    fn lookup_for_field_mut(
        &mut self,
        starting_from_class_name: &str,
        field_name_type: &str,
    ) -> Option<&mut Field> {
        match self.fields.get_index_of(starting_from_class_name) {
            Some(start_index) => self
                .fields
                .iter_mut()
                .take(start_index + 1)
                .rev()
                .find_map(|(_, map)| map.get_mut(field_name_type)),
            None => None,
        }
    }

    pub fn instance_name(&self) -> &str {
        &self.instance_name
    }
}
