use crate::error::Error;
use crate::method_area::field::Field;
use indexmap::IndexMap;
use serde::Serialize;

pub type ClassName = String;
pub type FieldNameType = String;

#[derive(Debug, Serialize, Clone)]
pub(crate) struct JavaInstance {
    instance_name: String,
    fields: IndexMap<ClassName, IndexMap<FieldNameType, Field>>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Array {
    type_name: String,
    data: Vec<Vec<i32>>,
}

impl Array {
    pub fn new(type_name: &str, len: i32) -> Self {
        Self {
            type_name: type_name.to_string(),
            data: vec![vec![0, 0]; len as usize], //todo: use either 1 or 2 elements vector for corresponding type
        }
    }

    pub fn new_with_values(type_name: &str, array: &[i32]) -> Self {
        //todo: is not suitable for long and double
        let converted_arr = array.iter().map(|item| vec![*item]).collect();
        Self {
            type_name: type_name.to_string(),
            data: converted_arr,
        }
    }

    pub fn get_value(&self, index: i32) -> crate::error::Result<&Vec<i32>> {
        if let Some(arr_value) = self.data.get(index as usize) {
            Ok(arr_value)
        } else {
            Err(Error::new_execution(&format!(
                "error getting array value at index={index}, from data={:?}",
                self.data
            )))
        }
    }

    pub fn set_value(&mut self, index: i32, value: Vec<i32>) -> crate::error::Result<()> {
        if let Some(arr_value) = self.data.get_mut(index as usize) {
            *arr_value = value;
            Ok(())
        } else {
            Err(Error::new_execution(&format!(
                "error setting array value: {value:?}"
            )))
        }
    }

    pub fn set_entire_value(&mut self, value: Array) -> crate::error::Result<()> {
        self.data = value.data;
        Ok(())
    }

    pub fn get_entire_value(&self) -> &Vec<Vec<i32>> {
        &self.data
    }

    pub fn get_length(&self) -> i32 {
        self.data.len() as i32
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }
}

#[derive(Debug, Serialize)]
pub(crate) enum HeapValue {
    Object(JavaInstance),
    Arr(Array),
}

impl JavaInstance {
    pub fn new(
        instance_name: String,
        fields: IndexMap<ClassName, IndexMap<FieldNameType, Field>>,
    ) -> Self {
        Self {
            instance_name,
            fields,
        }
    }

    pub fn set_field_value(
        &mut self,
        class_name: &str,
        field_name_type: &str,
        value: Vec<i32>,
    ) -> crate::error::Result<()> {
        self.lookup_for_field_mut(class_name, field_name_type)
            .and_then(|v| Some(v.set_raw_value(value)))
            .ok_or_else(|| {
                Error::new_execution(&format!(
                    "error setting value for instance field {class_name}.{field_name_type}"
                ))
            })?
    }

    pub fn get_field_value(
        &self,
        class_name: &str,
        field_name_type: &str,
    ) -> crate::error::Result<Vec<i32>> {
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
