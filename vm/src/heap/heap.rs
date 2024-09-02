use crate::error::Error;
use crate::heap::java_instance::JavaInstance;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Heap<'a> {
    data: HashMap<i32, JavaInstance<'a>>,
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

        self.data.insert(self.next_id, java_instance);

        self.next_id
    }

    pub fn set_object_field_value(
        &mut self,
        objectref: i32,
        fieldname: &str,
        value: i32,
    ) -> crate::error::Result<()> {
        if let Some(instance) = self.data.get_mut(&objectref) {
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
    ) -> crate::error::Result<i32> {
        if let Some(java_instance) = self.data.get(&objectref) {
            java_instance.get_field_value(fieldname)
        } else {
            Err(Error::new_execution("error getting field value"))
        }
    }
}
