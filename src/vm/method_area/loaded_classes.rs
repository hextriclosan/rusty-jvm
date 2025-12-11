use crate::vm::error::{Error, Result};
use crate::vm::heap::heap::HEAP;
use crate::vm::heap::java_instance::{JavaInstance, JavaInstanceBase, JavaInstanceClass};
use crate::vm::helper::undecorate;
use crate::vm::method_area::attributes_helper::AttributesHelper;
use crate::vm::method_area::class_modifiers::ClassModifier;
use crate::vm::method_area::cpool_helper::CPoolHelper;
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::method_area::primitives_helper::PRIMITIVE_TYPE_BY_CODE;
use crate::vm::UNNAMED_MODULE_REF;
use indexmap::{IndexMap, IndexSet};
use jdescriptor::TypeDescriptor;
use parking_lot::lock_api::RwLockWriteGuard;
use parking_lot::{RawRwLock, RwLock};
use std::ops::DerefMut;
use std::sync::atomic::{AtomicI8, Ordering};
use std::sync::{Arc, LazyLock};
use tracing::trace;

pub(crate) static CLASSES: LazyLock<LoadedClasses> = LazyLock::new(LoadedClasses::default);

#[derive(Debug, Default)]
pub(crate) struct LoadedClasses {
    loaded_classes: RwLock<IndexMap<String, Arc<JavaClass>>>,
    construct_stage: AtomicI8,
}

const OBJECT: &'static str = "java/lang/Object";
const CLASS: &'static str = "java/lang/Class";

impl LoadedClasses {
    /// Checks if class is already loaded
    pub fn is_loaded(&self, fully_qualified_class_name: &str) -> bool {
        let fully_qualified_class_name = undecorate(fully_qualified_class_name);
        self.loaded_classes
            .read()
            .contains_key(fully_qualified_class_name)
    }

    /// Gets class by its internal id
    pub fn get_by_id(&self, id: usize) -> Result<Arc<JavaClass>> {
        self.loaded_classes
            .read()
            .get_index(id)
            .map(|(_key, klass)| Arc::clone(klass))
            .ok_or_else(|| Error::new_execution(&format!("Class with id {id} not found")))
    }

    /// Gets class by name, loading it if necessary
    pub fn get(&self, fully_qualified_class_name: &str) -> Result<Arc<JavaClass>> {
        self.get_full(fully_qualified_class_name)
            .map(|(_id, _key, klass)| klass)
    }

    /// Gets class by name, loading it if necessary
    /// todo: Multistage loading will be here including using class loaders
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

        // todo loading class can be called concurrently from multiple threads - not critical but it's better to handle that
        let klass = if fully_qualified_class_name.starts_with('[') {
            Self::generate_synthetic_array_class(fully_qualified_class_name)
        } else {
            with_method_area(|a| a.load_class_file(fully_qualified_class_name))?
        };

        self.insert_klass(klass)
    }

    /// Inserts class into loaded classes, returns existing if already present
    /// Returns (klass_id, fully_qualified_class_name, klass)
    pub fn insert_klass(&self, klass: Arc<JavaClass>) -> Result<(usize, String, Arc<JavaClass>)> {
        let mut writer = self.loaded_classes.write();
        let fully_qualified_class_name = klass.this_class_name();
        // Double check locking, maybe another thread created it while we waited for the lock
        if let Some((id, key, klass)) = writer.get_full(fully_qualified_class_name) {
            return Ok((id, key.to_string(), Arc::clone(klass)));
        }

        if !fully_qualified_class_name.starts_with('[') {
            // this is not an array class - insert directly
            Self::perform_insertion(&klass, &mut writer, None)
        } else {
            Self::perform_array_insertion(&klass, &mut writer)
        }
    }

    fn perform_insertion(
        klass: &Arc<JavaClass>,
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

        let (class_klass_id, _name, class_klass) = writer.get_full(CLASS).ok_or_else(|| {
            Error::new_execution(&format!("{CLASS} class not found in loaded classes"))
        })?;

        Self::create_clazz_instance(
            &klass,
            klass_id,
            class_klass,
            class_klass_id,
            component_type_ref,
        )?;

        Ok((
            klass_id,
            fully_qualified_class_name.to_string(),
            Arc::clone(&klass),
        ))
    }

    fn perform_array_insertion(
        array_klass: &Arc<JavaClass>,
        writer: &mut RwLockWriteGuard<RawRwLock, IndexMap<String, Arc<JavaClass>>>,
    ) -> Result<(usize, String, Arc<JavaClass>)> {
        let fully_qualified_class_name = array_klass.this_class_name();
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

            let (_, _, component_klass) = Self::perform_insertion(&component_klass, writer, None)?;

            // Create array classes from component up to the full array class (except the last one which is created outside)
            let mut component_type_ref = component_klass.mirror_clazz_ref()?;
            for padding in (1..dimension as usize).rev() {
                let name = &fully_qualified_class_name[padding..];
                let (_klass_id, _name, klass) =
                    if let Some((id, key, klass)) = writer.get_full(name) {
                        // maybe sub-array class is already created, then just reuse it
                        Ok((id, key.to_string(), Arc::clone(klass)))
                    } else {
                        // create sub-array class
                        let sub_array_klass = Self::generate_synthetic_array_class(name);

                        Self::perform_insertion(&sub_array_klass, writer, Some(component_type_ref))
                    }?;

                component_type_ref = klass.mirror_clazz_ref()?;
            }

            // Finally create the full array class
            Self::perform_insertion(&array_klass, writer, Some(component_type_ref))
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
            CLASS,
            "componentType",
            vec![component_type_ref.unwrap_or(0)],
        )?;

        class_instance.set_field_value(
            CLASS,
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
        class_instance.set_field_value(CLASS, "modifiers", vec![class_modifiers as i32])?;

        let (module_ref, patch) = with_method_area(|a| {
            let file_name = format!("{}.class", klass.this_class_name());
            if let Some(package) = a.modules_mapping().get(&file_name) {
                let modules = a.modules();
                let registry = modules.registry();
                let module_ref = registry.get(package).map(|got| *got.value()).unwrap_or(0);
                let patch = package == "java.base" && module_ref == 0;
                Ok::<_, Error>((module_ref, patch))
            } else {
                // Set unnamed module
                let module_ref = UNNAMED_MODULE_REF.get().copied().unwrap_or(0); // todo: use unnamed module per classloader
                Ok::<_, Error>((module_ref, false))
            }
        })?;
        class_instance.set_field_value(CLASS, "module", vec![module_ref])?;

        let class_instance_id = HEAP.create_instance(class_instance);

        with_method_area(|method_area| {
            if patch {
                let modules = method_area.modules();
                let base_classes_to_patch = modules.base_classes_to_patch();
                let mut guard = base_classes_to_patch.lock();
                if let Some(to_patch) = guard.deref_mut() {
                    to_patch.insert(class_instance_id);
                } else {
                    return Err(Error::new_execution("Patching has already been performed"));
                }
            }
            Ok::<_, Error>(())
        })?;

        klass.inject_mirror_clazz_ref(class_instance_id)
    }

    /// Pre-construction initialization
    /// Used to load Object and Class classes before any other class loading
    pub fn pre_construct(&self) -> Result<()> {
        let current_stage = self.construct_stage.fetch_add(1i8, Ordering::SeqCst);
        if current_stage == 0 {
            let mut writer = self.loaded_classes.write();

            let object_klass = with_method_area(|a| a.load_class_file(OBJECT))?;
            writer.insert_full(OBJECT.to_string(), Arc::clone(&object_klass));
            trace!("<CLASS LOADED> -> {OBJECT}");

            let class_klass = with_method_area(|a| a.load_class_file(CLASS))?;
            writer.insert_full(CLASS.to_string(), Arc::clone(&class_klass));
            trace!("<CLASS LOADED> -> {CLASS}");

            Ok(())
        } else {
            Err(Error::new_execution(&format!(
                "pre_construct: wrong construction stage: {current_stage}"
            )))
        }
    }

    /// Post-construction initialization
    /// Used to create Class<Class> and Class<Object> instances to avoid infinite recursion during class loading
    pub fn post_construct(&self) -> Result<()> {
        let current_stage = self.construct_stage.fetch_add(1i8, Ordering::SeqCst);
        if current_stage == 1 {
            let reader = self.loaded_classes.read();

            // Create Class<Class> instance
            let (class_klass_id, _name, class_klass) =
                reader.get_full(CLASS).ok_or_else(|| {
                    Error::new_execution(&format!("{CLASS} class not found in loaded classes"))
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
                reader.get_full(OBJECT).ok_or_else(|| {
                    Error::new_execution(&format!("{OBJECT} class not found in loaded classes"))
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
            Err(Error::new_execution(&format!(
                "post_construct: wrong construction stage: {current_stage}"
            )))
        }
    }

    fn generate_synthetic_array_class(array_class_name_internal: &str) -> Arc<JavaClass> {
        let array_class_name_external = array_class_name_internal.replace('/', ".");
        Arc::new(JavaClass::new(
            IndexMap::new(),
            IndexMap::new(),
            IndexMap::new(),
            IndexMap::new(),
            CPoolHelper::new(&Vec::new()),
            AttributesHelper::new(&Vec::new()),
            array_class_name_internal.to_string(),
            array_class_name_external,
            Some(OBJECT.to_string()),
            IndexSet::from([
                "java/lang/Cloneable".to_string(),
                "java/io/Serializable".to_string(),
            ]),
            ClassModifier::Public | ClassModifier::Final | ClassModifier::Abstract,
            None,
            None,
            None,
            None,
        ))
    }
}
