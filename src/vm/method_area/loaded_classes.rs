use crate::vm::error::{Error, Result};
use crate::vm::helper::undecorate;
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::method_area::{with_method_area, MethodArea};
use indexmap::IndexMap;
use parking_lot::RwLock;
use std::sync::{Arc, LazyLock};

pub(crate) static CLASSES: LazyLock<LoadedClasses> = LazyLock::new(LoadedClasses::default);

#[derive(Debug, Default)]
pub(crate) struct LoadedClasses {
    loaded_classes: RwLock<IndexMap<String, Arc<JavaClass>>>,
}

impl LoadedClasses {
    /// Gets class by its internal id
    ///
    /// Used by Object.header.klass_id
    pub fn get_by_id(&self, id: usize) -> Result<Arc<JavaClass>> {
        self.loaded_classes
            .read()
            .get_index(id)
            .map(|(_key, klass)| Arc::clone(klass))
            .ok_or_else(|| Error::new_execution(&format!("Class with id {id} not found")))
    }

    /// Used by various places where class is needed by name
    pub fn get(&self, fully_qualified_class_name: &str) -> Result<Arc<JavaClass>> {
        self.get_full(fully_qualified_class_name)
            .map(|(_id, _key, klass)| klass)
    }

    /// Checks if class is already loaded
    ///
    /// Used by java/lang/ClassLoader:findLoadedClass0
    pub fn is_loaded(&self, fully_qualified_class_name: &str) -> bool {
        let fully_qualified_class_name = undecorate(fully_qualified_class_name);
        self.loaded_classes
            .read()
            .contains_key(fully_qualified_class_name)
    }

    /// Inserts class into loaded classes map if not already present
    ///
    /// Used by:
    /// - MethodArea::new() to insert synthetic classes for primitive types
    /// - MethodArea::create_metaclass() to create class dynamically from bytecode byte-array
    pub fn insert_klass(&self, klass: Arc<JavaClass>) {
        let this_class_name = klass.this_class_name().to_string();
        self.get_or_create_impl(&this_class_name, klass);
    }

    /// Gets class by name, loading it if necessary
    /// Multistage loading will be here including using class loaders
    ///
    /// Used by MethodArea::create_instance_with_default_fields(), instances creation entry point
    pub fn get_full(
        &self,
        fully_qualified_class_name: &str,
    ) -> Result<(usize, String, Arc<JavaClass>)> {
        let fully_qualified_class_name = undecorate(fully_qualified_class_name);
        {
            let reader = self.loaded_classes.read();
            if let Some((id, key, klass)) = reader.get_full(fully_qualified_class_name) {
                return Ok((id, key.to_string(), Arc::clone(klass)));
            }
        }

        let klass = if fully_qualified_class_name.starts_with('[') {
            MethodArea::generate_synthetic_array_class(fully_qualified_class_name)
        } else {
            with_method_area(|a| a.load_class_file(fully_qualified_class_name))?
        };

        Ok(self.get_or_create_impl(fully_qualified_class_name, klass))
    }

    fn get_or_create_impl(
        &self,
        fully_qualified_class_name: &str,
        klass: Arc<JavaClass>,
    ) -> (usize, String, Arc<JavaClass>) {
        let fully_qualified_class_name = undecorate(fully_qualified_class_name);
        let mut writer = self.loaded_classes.write();
        // Double check locking, maybe another thread created it while we waited for the lock
        if let Some((id, key, klass)) = writer.get_full(fully_qualified_class_name) {
            return (id, key.to_string(), Arc::clone(klass));
        }

        let name = fully_qualified_class_name.to_string();
        let (id, _value) = writer.insert_full(name.clone(), Arc::clone(&klass));
        (id, name, Arc::clone(&klass))
    }
}
