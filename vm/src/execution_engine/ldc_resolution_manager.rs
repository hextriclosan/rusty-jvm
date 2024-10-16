use crate::error::Error;
use crate::execution_engine::string_pool_helper::StringPoolHelper;
use crate::helper::{i32toi64, i64_to_vec};
use crate::method_area::method_area::with_method_area;
use std::collections::HashMap;
use std::sync::RwLock;

type CPoolIndex = u16;
type Value = Vec<i32>;

pub struct LdcResolutionManager {
    cache: RwLock<HashMap<String, HashMap<CPoolIndex, Value>>>,
}

impl LdcResolutionManager {
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub fn resolve_ldc(
        &self,
        current_class_name: &str,
        cpoolindex: u16,
    ) -> crate::error::Result<i32> {
        if let Some(Some(value)) = self
            .cache
            .read()
            .expect("error getting cache lock")
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
            let class = with_method_area(|method_area| method_area.get(&class_name))?;

            class.reflection_ref()
        } else {
            return Err(Error::new_constant_pool(&format!(
                "Error resolving ldc: {}",
                cpoolindex
            )));
        };

        self.cache
            .write()
            .expect("error getting cache write lock")
            .entry(current_class_name.to_string())
            .or_insert_with(HashMap::new)
            .insert(cpoolindex, vec![result]);

        Ok(result)
    }

    pub fn resolve_ldc2_w(
        &self,
        current_class_name: &str,
        cpoolindex: u16,
    ) -> crate::error::Result<i64> {
        if let Some(Some(value)) = self
            .cache
            .read()
            .expect("error getting cache lock")
            .get(current_class_name)
            .map(|map| map.get(&cpoolindex))
        {
            return Ok(i32toi64(value[0], value[1]));
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
            .write()
            .expect("error getting cache write lock")
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
