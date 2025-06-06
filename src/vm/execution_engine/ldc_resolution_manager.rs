use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::reflection_class_loader::ReflectionClassLoader;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::helper::{i64_to_vec, vec_to_i64};
use crate::vm::method_area::cpool_helper::CPoolHelperTrait;
use crate::vm::method_area::method_area::with_method_area;
use std::collections::HashMap;
use std::sync::RwLock;

type CPoolIndex = u16;
type Value = Vec<i32>;

#[derive(Debug, Default)]
pub struct LdcResolutionManager {
    reflection_class_loader: ReflectionClassLoader,
    cache: RwLock<HashMap<String, HashMap<CPoolIndex, Value>>>,
}

impl LdcResolutionManager {
    pub fn resolve_ldc(&self, current_class_name: &str, cpoolindex: u16) -> Result<i32> {
        if let Some(Some(value)) = self
            .cache
            .read()?
            .get(current_class_name)
            .map(|map| map.get(&cpoolindex))
        {
            return Ok(value[0]);
        }

        let java_class = with_method_area(|method_area| method_area.get(current_class_name))?;
        let cpool_helper = java_class.cpool_helper();

        let result = if let Some(value) = cpool_helper.get_integer(cpoolindex) {
            value
        } else if let Some(value) = cpool_helper.get_float(cpoolindex) {
            Self::float_to_int(value)
        } else if let Some(value) = cpool_helper.get_string(cpoolindex) {
            StringPoolHelper::get_string(value)?
        } else if let Some(class_name) = cpool_helper.get_class(cpoolindex) {
            self.load_reflection_class(&class_name)?
        } else {
            return Err(Error::new_constant_pool(&format!(
                "Error resolving ldc: {}",
                cpoolindex
            )));
        };

        self.cache
            .write()?
            .entry(current_class_name.to_string())
            .or_insert_with(HashMap::new)
            .insert(cpoolindex, vec![result]);

        Ok(result)
    }

    pub fn load_reflection_class(&self, class_name: &str) -> Result<i32> {
        self.reflection_class_loader.load(&class_name)
    }

    pub fn resolve_ldc2_w(&self, current_class_name: &str, cpoolindex: u16) -> Result<i64> {
        if let Some(Some(value)) = self
            .cache
            .read()?
            .get(current_class_name)
            .map(|map| map.get(&cpoolindex))
        {
            return Ok(vec_to_i64(value));
        }

        let java_class = with_method_area(|method_area| method_area.get(current_class_name))?;
        let cpool_helper = java_class.cpool_helper();

        let result = if let Some(value) = cpool_helper.get_long(cpoolindex) {
            value
        } else if let Some(value) = cpool_helper.get_double(cpoolindex) {
            Self::double_to_int(value)
        } else {
            return Err(Error::new_constant_pool(&format!(
                "Error resolving ldc: {}",
                cpoolindex
            )));
        };

        self.cache
            .write()?
            .entry(current_class_name.to_string())
            .or_insert_with(HashMap::new)
            .insert(cpoolindex, i64_to_vec(result));

        Ok(result)
    }

    fn float_to_int(value: f32) -> i32 {
        value.to_bits() as i32
    }

    fn double_to_int(value: f64) -> i64 {
        value.to_bits() as i64
    }
}
