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
}
