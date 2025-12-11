use crate::vm::error::{Error, Result};
use crate::vm::heap::heap::HEAP;
use crate::vm::heap::java_instance::{JavaInstance, JavaInstanceBase, JavaInstanceClass};
use crate::vm::helper::undecorate;
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::method_area::{with_method_area, MethodArea};
use crate::vm::method_area::primitives_helper::PRIMITIVE_TYPE_BY_CODE;
use indexmap::IndexMap;
use jdescriptor::TypeDescriptor;
use parking_lot::lock_api::RwLockWriteGuard;
use parking_lot::{RawRwLock, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock};
use tracing::trace;

pub(crate) static CLASSES: LazyLock<LoadedClasses> = LazyLock::new(LoadedClasses::default);

#[derive(Debug, Default)]
pub(crate) struct LoadedClasses {
    loaded_classes: RwLock<IndexMap<String, Arc<JavaClass>>>,
    post_construct_invoked: AtomicBool,
}

impl LoadedClasses {
    /// Checks if class is already loaded
    ///
    /// Used by java/lang/ClassLoader:findLoadedClass0
    pub fn is_loaded(&self, fully_qualified_class_name: &str) -> bool {
        let fully_qualified_class_name = undecorate(fully_qualified_class_name);
        self.loaded_classes
            .read()
            .contains_key(fully_qualified_class_name)
    }

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
            // todo loading class can be called concurrently from multiple threads - need to handle that
        };
        match fully_qualified_class_name {
            "java/lang/Object" | "java/lang/Class" => {
                // we don't create Class instances for Object and Class here to avoid infinite recursion
                self.insert_klass_impl(klass, false)
            }
            _ => self.insert_klass(klass),
        }
    }

    /// Inserts class into loaded classes, returns existing if already present
    ///
    /// Used by:
    /// - MethodArea::new() to insert synthetic classes for primitive types
    /// - MethodArea::create_metaclass() to create class dynamically from bytecode byte-array
    pub fn insert_klass(&self, klass: Arc<JavaClass>) -> Result<(usize, String, Arc<JavaClass>)> {
        self.insert_klass_impl(klass, true)
    }

    fn insert_klass_impl(
        &self,
        klass: Arc<JavaClass>,
        create_clazz_instance: bool,
    ) -> Result<(usize, String, Arc<JavaClass>)> {
        let mut writer = self.loaded_classes.write();
        let fully_qualified_class_name = klass.this_class_name();
        // Double check locking, maybe another thread created it while we waited for the lock
        if let Some((id, key, klass)) = writer.get_full(fully_qualified_class_name) {
            return Ok((id, key.to_string(), Arc::clone(klass)));
        }

        if !fully_qualified_class_name.starts_with('[') {
            // this is not an array class - insert directly
            Self::perform_insertion(&klass, create_clazz_instance, &mut writer, None)
        } else {
            Self::perform_array_insertion(&mut writer, &fully_qualified_class_name)
        }
    }

    fn perform_insertion(
        klass: &Arc<JavaClass>,
        create_clazz_instance: bool,
        writer: &mut RwLockWriteGuard<RawRwLock, IndexMap<String, Arc<JavaClass>>>,
        component_type_ref: Option<i32>,
    ) -> Result<(usize, String, Arc<JavaClass>)> {
        let fully_qualified_class_name = klass.this_class_name();
        if let Some((id, key, klass)) = writer.get_full(fully_qualified_class_name) {
            return Ok((id, key.to_string(), Arc::clone(klass)));
        }

        let (klass_id, _value) =
            writer.insert_full(fully_qualified_class_name.to_string(), Arc::clone(&klass));
        trace!("<CLASS LOADED> -> {}", fully_qualified_class_name);

        if create_clazz_instance {
            let clazz_name = "java/lang/Class";
            let (class_klass_id, _name, class_klass) =
                writer.get_full(clazz_name).ok_or_else(|| {
                    Error::new_execution(&format!(
                        "{clazz_name} class not found in loaded classes"
                    ))
                })?;

            Self::create_clazz_instance(
                &klass,
                klass_id,
                class_klass,
                class_klass_id,
                component_type_ref,
            )?;
        }

        Ok((
            klass_id,
            fully_qualified_class_name.to_string(),
            Arc::clone(&klass),
        ))
    }

    fn perform_array_insertion(
        mut writer: &mut RwLockWriteGuard<RawRwLock, IndexMap<String, Arc<JavaClass>>>,
        fully_qualified_class_name: &str,
    ) -> Result<(usize, String, Arc<JavaClass>)> {
        if let Ok(TypeDescriptor::Array(value, dimension)) =
            fully_qualified_class_name.parse::<TypeDescriptor>()
        {
            // Create component class first
            let component_name = value.to_string();
            let component_name = undecorate(&component_name);
            let component_klass =
                if let Some((_id, _name, klass)) = writer.get_full(component_name) {
                    Ok(Arc::clone(klass))
                } else {
                    with_method_area(|a| a.load_class_file(component_name))
                }?;

            let _ = Self::perform_insertion(&component_klass, true, &mut writer, None)?;

            // Create array classes from component up to full array
            let mut component_type_ref = component_klass.mirror_clazz_ref()?;
            for padding in (0..dimension as usize).rev() {
                let name = &fully_qualified_class_name[padding..];
                let (klass_id, name, klass) = if let Some((id, key, klass)) = writer.get_full(name)
                {
                    Ok((id, key.to_string(), Arc::clone(klass)))
                } else {
                    let array_klass = MethodArea::generate_synthetic_array_class(name);

                    Self::perform_insertion(
                        &array_klass,
                        true,
                        &mut writer,
                        Some(component_type_ref),
                    )
                }?;

                if padding == 0 {
                    return Ok((klass_id, name, klass));
                }

                component_type_ref = klass.mirror_clazz_ref()?;
            }
            Err(Error::new_execution(
                "Array insertion loop terminated unexpectedly",
            ))
        } else {
            Err(Error::new_execution(&format!(
                "Unexpected descriptor {fully_qualified_class_name}"
            )))
        }
    }

    fn create_clazz_instance(
        klass: &Arc<JavaClass>,
        klass_id: usize,
        class_klass: &Arc<JavaClass>,
        class_klass_id: usize,
        component_type_ref: Option<i32>,
    ) -> Result<()> {
        let mut class_instance = JavaInstance::Class(JavaInstanceClass::new(
            JavaInstanceBase::new(
                class_klass_id,
                class_klass.instance_fields_hierarchy()?.clone(),
            ),
            klass_id,
        ));
        class_instance.set_field_value(
            "java/lang/Class",
            "componentType",
            vec![component_type_ref.unwrap_or(0)],
        )?;

        class_instance.set_field_value(
            "java/lang/Class",
            "primitive",
            vec![
                if PRIMITIVE_TYPE_BY_CODE.contains_key(klass.this_class_name().as_str()) {
                    1
                } else {
                    0
                },
            ],
        )?;
        let class_modifiers = klass.class_modifiers().bits();
        class_instance.set_field_value(
            "java/lang/Class",
            "modifiers",
            vec![class_modifiers as i32],
        )?;

        let class_instance_id = HEAP.create_instance(class_instance.clone());

        klass.inject_mirror_clazz_ref(class_instance_id)
    }

    /// Post construction
    pub fn post_construct(&self) -> Result<()> {
        if !self.post_construct_invoked.swap(true, Ordering::SeqCst) {
            let reader = self.loaded_classes.read();

            // Create Class<Class> instance
            let (class_klass_id, _name, class_klass) =
                reader.get_full("java/lang/Class").ok_or_else(|| {
                    Error::new_execution("java/lang/Class class not found in loaded classes")
                })?;
            Self::create_clazz_instance(
                &class_klass,
                class_klass_id,
                class_klass,
                class_klass_id,
                None,
            )?;

            // Create Class<Object> instance
            let (object_klass_id, _name, object_klass) =
                reader.get_full("java/lang/Object").ok_or_else(|| {
                    Error::new_execution("java/lang/Object class not found in loaded classes")
                })?;
            Self::create_clazz_instance(
                &object_klass,
                object_klass_id,
                class_klass,
                class_klass_id,
                None,
            )?;

            Ok(())
        } else {
            Err(Error::new_execution(
                "LoadedClasses post_construct has already been called",
            ))
        }
    }
}
