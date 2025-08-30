use crate::vm::error::{Error, Result};
use crate::vm::heap::heap::with_heap_write_lock;
use crate::vm::helper::undecorate;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::method_area::primitives_helper::PRIMITIVE_TYPE_BY_CODE;
use jdescriptor::TypeDescriptor;
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug, Default)]
pub struct ReflectionClassLoader {
    class_type_instance_by_name: RwLock<HashMap<String, i32>>,
}

impl ReflectionClassLoader {
    pub fn load(&self, for_class: &str) -> Result<i32> {
        let class_modifiers = with_method_area(|area| {
            Ok::<u16, Error>(area.get(for_class)?.class_modifiers().bits())
        })?;

        let component_type_ref_empty = 0;
        if !for_class.starts_with('[') && !for_class.ends_with(';') {
            return self.get_or_create(for_class, component_type_ref_empty, class_modifiers);
        }

        let descriptor: TypeDescriptor = for_class.parse()?;
        match &descriptor {
            TypeDescriptor::Array(value, dimension) => {
                let component_ref_name = value.as_ref().to_string();
                let component_name = undecorate(&component_ref_name);
                let component_flags = with_method_area(|area| {
                    Ok::<u16, Error>(area.get(&component_name)?.class_modifiers().bits())
                })?;

                let mut array_ref = self.get_or_create(
                    &component_ref_name,
                    component_type_ref_empty,
                    component_flags,
                )?;

                for padding in (0..*dimension as usize).rev() {
                    let name = &for_class[padding..];
                    array_ref = self.get_or_create(name, array_ref, class_modifiers)?;
                }

                Ok(array_ref)
            }
            _ => self.get_or_create(for_class, component_type_ref_empty, class_modifiers),
        }
    }

    fn get_or_create(
        &self,
        class_name: &str,
        component_type_ref: i32,
        modifiers: u16,
    ) -> Result<i32> {
        let mut class_type_map = self.class_type_instance_by_name.write()?;
        if let Some(&reflection_ref) = class_type_map.get(class_name) {
            return Ok(reflection_ref);
        }

        let reflection_ref =
            Self::create_reflection_instance(class_name, component_type_ref, modifiers)?;
        class_type_map.insert(class_name.to_string(), reflection_ref);
        Ok(reflection_ref)
    }

    fn create_reflection_instance(
        this_class_name: &str,
        component_type_ref: i32,
        modifiers: u16,
    ) -> Result<i32> {
        let mut reflection_instance = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields("java/lang/Class")
        })?;
        reflection_instance.set_field_value(
            "java/lang/Class",
            "componentType",
            vec![component_type_ref],
        )?;
        reflection_instance.set_field_value(
            "java/lang/Class",
            "primitive",
            vec![if PRIMITIVE_TYPE_BY_CODE.contains_key(this_class_name) {
                1
            } else {
                0
            }],
        )?;
        reflection_instance.set_field_value(
            "java/lang/Class",
            "modifiers",
            vec![modifiers as i32],
        )?;

        let reflection_reference =
            with_heap_write_lock(|heap| heap.create_instance(reflection_instance));

        with_method_area(|method_area| {
            method_area.put_to_reflection_table(reflection_reference, this_class_name)
        })?;

        Ok(reflection_reference)
    }
}
