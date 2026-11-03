use crate::vm::error::{Error, Result};
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
    pub(crate) fn root_refs(&self) -> Result<Vec<i32>> {
        let mut roots: Vec<_> = self.registry.iter().map(|entry| *entry.value()).collect();
        let patches = self.base_classes_to_patch.try_lock().ok_or_else(|| {
            Error::new_execution("Module patch list locked during root snapshot")
        })?;
        if let Some(patches) = patches.as_ref() {
            roots.extend(patches.iter().copied());
        }
        Ok(roots)
    }

    pub fn new() -> Self {
        Self {
            registry: DashMap::new(),
            base_classes_to_patch: Mutex::new(Some(HashSet::new())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gc_roots_modules_include_pending_patches_and_fail_closed_on_contention() {
        let modules = Modules::new();
        modules.registry.insert("java.base".into(), 31);
        modules
            .base_classes_to_patch
            .lock()
            .as_mut()
            .unwrap()
            .insert(32);

        assert_eq!(modules.root_refs().unwrap(), vec![31, 32]);
        let mut held = modules.base_classes_to_patch.lock();
        assert!(modules.root_refs().is_err());
        *held = None;
        drop(held);
        assert_eq!(modules.root_refs().unwrap(), vec![31]);
    }
}
