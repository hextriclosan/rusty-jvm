use crate::vm::commons::auto_dash_map::auto_dash_map::AutoDashMap;
use crate::vm::commons::auto_dash_map::auto_dash_map_i32::AutoDashMapI32;
use crate::vm::error::{Error, Result};
use crate::vm::helper::undecorate;
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::method_area::{with_method_area, MethodArea};
use dashmap::DashMap;
use std::sync::{Arc, LazyLock};

pub(crate) static CLASSES: LazyLock<LoadedClasses> = LazyLock::new(LoadedClasses::default);

pub(crate) struct LoadedClasses {
    loaded_classes: AutoDashMapI32<Arc<JavaClass>>,
    index_by_name: DashMap<String, i32>,
}

impl Default for LoadedClasses {
    fn default() -> Self {
        Self {
            loaded_classes: AutoDashMapI32::new(1),
            index_by_name: DashMap::default(),
        }
    }
}

impl LoadedClasses {
    pub fn get_by_id(&self, id: i32) -> Result<Arc<JavaClass>> {
        self.loaded_classes
            .get(id)
            .and_then(|class| Some(Arc::clone(class.value())))
            .ok_or_else(|| Error::new_execution(&format!("Class with id {id} not found")))
    }

    pub fn get(&self, fully_qualified_class_name: &str) -> Result<Arc<JavaClass>> {
        self.get_with_id(fully_qualified_class_name)
            .map(|(_, klass)| klass)
    }

    /// Get loaded class by its fully qualified name.
    /// If the class is not loaded, loads and returns it.
    /// Multistage loading will be here including using class loaders
    pub fn get_with_id(&self, fully_qualified_class_name: &str) -> Result<(i32, Arc<JavaClass>)> {
        let fully_qualified_class_name = undecorate(fully_qualified_class_name);
        if let Some(id) = self.index_by_name.get(fully_qualified_class_name) {
            let id = *id.value();
            let klass = self.get_by_id(id)?;
            return Ok((id, klass));
        }

        if fully_qualified_class_name.starts_with('[') {
            let arc = MethodArea::generate_synthetic_array_class(fully_qualified_class_name);
            let id = self.insert_auto(Arc::clone(&arc));
            return Ok((id, arc));
        }

        let klass = with_method_area(|a| a.load_class_file(fully_qualified_class_name))?;

        let id = self.insert_auto(Arc::clone(&klass));

        Ok((id, klass))
    }

    pub fn is_loaded(&self, fully_qualified_class_name: &str) -> bool {
        let fully_qualified_class_name = undecorate(fully_qualified_class_name);
        self.index_by_name.contains_key(fully_qualified_class_name)
    }

    pub fn insert_auto(&self, klass: Arc<JavaClass>) -> i32 {
        let this_class_name = klass.this_class_name().to_string();

        if self.index_by_name.contains_key(&this_class_name) {
            unreachable!("The class with name {} is already loaded", this_class_name);
        }

        let id = self.loaded_classes.insert_auto(klass);
        self.index_by_name.insert(this_class_name, id);
        id
    }
}
