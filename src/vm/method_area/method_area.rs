use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::ldc_resolution_manager::LdcResolutionManager;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::java_instance::{JavaInstance, JavaInstanceBase};
use crate::vm::helper::klass;
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::method_area::module_helper::Modules;
use crate::vm::method_area::primitives_helper::PRIMITIVE_TYPE_BY_CODE;
use crate::vm::system_native::class_loader::SYNTH_CLASS_DELIM;
use crate::vm::{JAVA_HOME, SYSTEM_CLASSLOADER_REF};
use indexmap::IndexSet;
use jclassmodel::{parse, ClassModel};
use jimage_rs::jimage::JImage;
use jimage_rs::raw_jimage::RawJImage;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::trace;

static METHOD_AREA: OnceCell<MethodArea> = OnceCell::new();

pub(crate) fn with_method_area<F, R>(f: F) -> R
where
    F: FnOnce(&MethodArea) -> R,
{
    let method_area = METHOD_AREA.get().expect("error getting method area");

    f(method_area)
}

#[derive(Debug)]
pub(crate) struct MethodArea {
    jimage: JImage,
    modules_mapping: HashMap<String, String>,
    modules: Arc<Modules>,
    ldc_resolution_manager: LdcResolutionManager,
    system_thread_id: OnceCell<i32>, // main thread, spawned by VM
    system_thread_group_id: OnceCell<i32>, // root "system" thread group, created by VM
    main_thread_group_id: OnceCell<i32>, // "main" thread group (child of system), holds the main thread
}

impl MethodArea {
    pub(crate) fn init() -> Result<()> {
        METHOD_AREA
            .set(MethodArea::new()?)
            .map_err(|_| Error::new_execution("MethodArea already initialized"))
    }

    fn new() -> Result<Self> {
        let java_home = JAVA_HOME.get().ok_or_else(|| {
            Error::new_execution("JAVA_HOME not set, cannot initialize MethodArea")
        })?;
        let modules = java_home.join("lib").join("modules");
        let jimage = JImage::open(modules)?;
        let modules_mapping = jimage
            .resource_names_iter()
            .map(|result| result.map_err(Error::from))
            .map(|result| result.map(|r| r.get_full_name()))
            .map(|result| result.map(|(module, name)| (name, module)))
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(Self {
            jimage,
            modules_mapping,
            modules: Arc::new(Modules::new()),
            ldc_resolution_manager: LdcResolutionManager::default(),
            system_thread_id: OnceCell::new(),
            system_thread_group_id: OnceCell::new(),
            main_thread_group_id: OnceCell::new(),
        })
    }

    pub(crate) fn create_metaclass(
        &self,
        fully_qualified_class_name: &str,
        bytecode: &[u8],
        class_loader_ref: i32,
    ) -> Result<(String, String)> {
        let (internal, external) = derive_internal_and_external_names(fully_qualified_class_name);

        if CLASSES.is_loaded(&internal) {
            return Ok((internal, external));
        }

        let model = parse(bytecode)?.into_model(internal.clone(), external.clone())?;
        let (_, java_class) = self.to_java_class(model)?;
        CLASSES.insert_klass(Arc::clone(&java_class), Some(class_loader_ref))?;
        trace!("<META CLASS LOADED> -> {}", java_class.this_class_name());

        Ok((internal, external))
    }

    pub(crate) fn load_class_file(
        &self,
        fully_qualified_class_name: &str,
    ) -> Result<Arc<JavaClass>> {
        let class_file_path = format!("{fully_qualified_class_name}.class");
        if let Some(module) = self.modules_mapping.get(&class_file_path) {
            let resource_path = format!("/{module}/{class_file_path}");
            if let Some(res) = self
                .jimage
                .find_resource(&resource_path)
                .map_err(|jimage_error| Error::new_execution(&jimage_error.to_string()))?
            {
                match self.try_parse(&res) {
                    Ok(Some(java_class)) => return Ok(java_class),
                    Ok(None) => {}
                    Err(e) => return Err(e),
                };
            }
        }

        if class_file_path.starts_with("java/") {
            self.try_open_and_parse(&PathBuf::from(&class_file_path))?
                .ok_or_else(|| {
                    Error::new_execution(&format!("error opening class file {class_file_path}"))
                })
        } else {
            let external_name = fully_qualified_class_name.replace('/', ".");
            let name_ref = StringPoolHelper::get_string(&external_name)?;
            let clazz_ref = Executor::invoke_static_method(
                "java/lang/Class",
                "forName:(Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;",
                &[
                    name_ref.into(),
                    0.into(),
                    SYSTEM_CLASSLOADER_REF.get().copied().unwrap_or(0).into(),
                ],
            )?[0];
            klass(clazz_ref)
        }
    }

    fn try_open_and_parse(&self, path: &PathBuf) -> Result<Option<Arc<JavaClass>>> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None), // File not found is not considered as an error
            Err(e) => return Err(e.into()),
        };
        let mut buff = Vec::new();
        file.read_to_end(&mut buff)?;

        self.try_parse(&buff)
    }

    fn try_parse(&self, buff: &[u8]) -> Result<Option<Arc<JavaClass>>> {
        let parsed = parse(buff)?;
        let class_name = parsed
            .this_class_name()
            .ok_or_else(|| Error::new_constant_pool("Error getting class_name of parsed class"))?;

        let (internal, external) = derive_internal_and_external_names(&class_name);
        self.to_java_class(parsed.into_model(internal, external)?)
            .map(|(_, java_class)| Ok(Some(java_class)))?
    }

    fn to_java_class(&self, model: ClassModel) -> Result<(String, Arc<JavaClass>)> {
        let name = model.name.clone();

        Ok((name, Arc::new(JavaClass::from_model(model)?)))
    }

    pub fn create_instance_with_default_fields(&self, class_name: &str) -> Result<JavaInstance> {
        let (id, _key, klass) = CLASSES.get_full(class_name)?;
        Ok(JavaInstance::Base(JavaInstanceBase::new(
            id,
            klass.instance_fields_hierarchy()?.clone(),
        )))
    }

    pub(crate) fn generate_synthetic_classes() -> Vec<Arc<JavaClass>> {
        PRIMITIVE_TYPE_BY_CODE
            .keys()
            .map(|class_name| Self::generate_synthetic_class(class_name))
            .collect()
    }

    fn generate_synthetic_class(class_name: &str) -> Arc<JavaClass> {
        let (internal, external) = derive_internal_and_external_names(class_name);
        Arc::new(JavaClass::synthetic(
            internal,
            external,
            None,
            IndexSet::new(),
        ))
    }

    pub(crate) fn resolve_ldc(&self, current_class_name: &str, cpoolindex: u16) -> Result<i32> {
        self.ldc_resolution_manager
            .resolve_ldc(current_class_name, cpoolindex)
    }

    pub(crate) fn resolve_ldc2_w(&self, current_class_name: &str, cpoolindex: u16) -> Result<i64> {
        self.ldc_resolution_manager
            .resolve_ldc2_w(current_class_name, cpoolindex)
    }

    pub fn system_thread_id(&self) -> Result<i32> {
        self.system_thread_id
            .get()
            .copied()
            .ok_or_else(|| Error::new_execution("system_thread_id wasn't set"))
    }

    pub fn set_system_thread_id(&self, thread_id: i32) -> Result<()> {
        self.system_thread_id.set(thread_id).map_err(|_| {
            Error::new_execution("system_thread_id was already set, cannot be set again")
        })
    }

    pub fn set_system_thread_group_id(&self, thread_group_id: i32) -> Result<()> {
        self.system_thread_group_id
            .set(thread_group_id)
            .map_err(|_| {
                Error::new_execution("system_thread_group_id was already set, cannot be set again")
            })
    }

    pub fn set_main_thread_group_id(&self, thread_group_id: i32) -> Result<()> {
        self.main_thread_group_id.set(thread_group_id).map_err(|_| {
            Error::new_execution("main_thread_group_id was already set, cannot be set again")
        })
    }

    pub fn modules(&self) -> Arc<Modules> {
        Arc::clone(&self.modules)
    }

    pub fn jimage_raw(&self) -> RawJImage<'_> {
        self.jimage.raw()
    }

    pub fn modules_mapping(&self) -> &HashMap<String, String> {
        &self.modules_mapping
    }
}

/// Takes a raw name like "my/package/MyClass#0xABCDEF"
/// Returns (internal_name, external_name)
///
/// Examples:
///     "my/package/MyClass#0xABCDEF" -> ("my/package/MyClass/0xABCDEF", "my.package.MyClass/0xABCDEF")
///     "I" -> ("I", "int")
///     "my/package/MyClass" -> ("my/package/MyClass", "my.package.MyClass")
fn derive_internal_and_external_names(raw: &str) -> (String, String) {
    if let Some(external_name) = PRIMITIVE_TYPE_BY_CODE.get(raw) {
        // Check if the raw name is a primitive type
        let internal = raw.to_string();
        let external = external_name.to_string();
        (internal, external)
    } else if let Some(pos) = raw.rfind(SYNTH_CLASS_DELIM) {
        // Check for synthetic class delimiter
        let (base, suffix) = raw.split_at(pos);
        let suffix = &suffix[1..];

        let internal = format!("{}/{}", base, suffix);
        let external = format!("{}/{}", base.replace('/', "."), suffix);

        (internal, external)
    } else {
        // Just ordinary class name
        let internal = raw.to_string();
        let external = raw.replace('/', ".");
        (internal, external)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_internal_and_external_names_with_primitive_type() {
        let result = derive_internal_and_external_names("I");
        assert_eq!(result, ("I".to_string(), "int".to_string()));
    }

    #[test]
    fn derive_internal_and_external_names_with_synthetic_class_delim() {
        let result = derive_internal_and_external_names(
            "java/lang/invoke/LambdaForm$MH#0x0000000000000002",
        );
        assert_eq!(
            result,
            (
                "java/lang/invoke/LambdaForm$MH/0x0000000000000002".to_string(),
                "java.lang.invoke.LambdaForm$MH/0x0000000000000002".to_string()
            )
        );
    }

    #[test]
    fn derive_internal_and_external_names_without_synthetic_class_delim() {
        let result = derive_internal_and_external_names(
            "java/util/concurrent/ConcurrentHashMap$CollectionView",
        );
        assert_eq!(
            result,
            (
                "java/util/concurrent/ConcurrentHashMap$CollectionView".to_string(),
                "java.util.concurrent.ConcurrentHashMap$CollectionView".to_string()
            )
        );
    }

    #[test]
    fn derive_internal_and_external_names_with_arrays() {
        let result = derive_internal_and_external_names("[Ljava/lang/Class;");
        assert_eq!(
            result,
            (
                "[Ljava/lang/Class;".to_string(),
                "[Ljava.lang.Class;".to_string()
            )
        );
    }
}
