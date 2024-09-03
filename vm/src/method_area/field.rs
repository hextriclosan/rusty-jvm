#[derive(Debug)]
pub(crate) struct Field {
    value: i32, // todo: support other types
}

impl Field {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
