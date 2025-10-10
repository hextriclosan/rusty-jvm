use dashmap::DashMap;
use getset::Getters;
use parking_lot::Mutex;
use std::collections::HashSet;

#[derive(Debug, Getters)]
pub struct Modules {
    #[getset(get = "pub")]
    registry: DashMap<String, i32>,
    #[getset(get = "pub")]
    base_classes_to_patch: Mutex<Option<HashSet<i32>>>,
}

impl Modules {
    pub fn new() -> Self {
        Self {
            registry: DashMap::new(),
            base_classes_to_patch: Mutex::new(Some(HashSet::new())),
        }
    }
}
