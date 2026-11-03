use crate::vm::error::Result;
use crate::vm::exception::common::construct_exception_and_throw;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{clazz_ref, undecorate};
use crate::vm::method_area::primitives_helper::PRIMITIVE_TYPE_BY_CODE;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::stack::stack_value::StackValueKind;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use tracing::warn;

/// Throws the `ClassCastException` for casting an instance of `from` to `to`, both given as
/// internal names (e.g. `java/lang/String` or `[I`), with the same message as HotSpot:
/// `class A cannot be cast to class B (A and B are in module m of loader 'l')`.
pub fn throw_class_cast_exception(
    from: &str,
    to: &str,
    stack_frames: &mut StackFrames,
) -> Result<()> {
    let from_name = from.replace('/', ".");
    let to_name = to.replace('/', ".");
    // The module details are only a description, so failing to look them up must not stop
    // the exception from being thrown
    let message = match class_cast_description(from, &from_name, to, &to_name) {
        Ok(description) => {
            format!("class {from_name} cannot be cast to class {to_name} ({description})")
        }
        Err(e) => {
            warn!("Failed to describe the modules of {from_name} and {to_name}: {e}");
            format!("class {from_name} cannot be cast to class {to_name}")
        }
    };
    let message_ref = StringPoolHelper::create_string(&message)?;
    construct_exception_and_throw(
        "java/lang/ClassCastException",
        "<init>:(Ljava/lang/String;)V",
        &[StackValueKind::from(message_ref)],
        stack_frames,
    )
}

fn class_cast_description(from: &str, from_name: &str, to: &str, to_name: &str) -> Result<String> {
    let from_location = ClassLocation::of(from)?;
    let to_location = ClassLocation::of(to)?;

    Ok(if from_location.is_same_module(&to_location) {
        format!(
            "{from_name} and {to_name} are in {}",
            to_location.describe()
        )
    } else {
        format!(
            "{from_name} is in {}; {to_name} is in {}",
            from_location.describe(),
            to_location.describe()
        )
    })
}

/// The module and class loader of a class. An array has those of its innermost element type,
/// and primitives are in `java.base` with the bootstrap loader.
struct ClassLocation {
    module_ref: i32,
    /// `None` for an unnamed module
    module_name: Option<String>,
    loader_ref: i32,
    /// E.g. `'bootstrap'`, `'app'` or `'MyLoader' @1b6d3586`
    loader_name: String,
}

impl ClassLocation {
    fn of(class_name: &str) -> Result<Self> {
        let element = class_name.trim_start_matches('[');
        let owner = if PRIMITIVE_TYPE_BY_CODE.contains_key(element) {
            "java/lang/Object"
        } else {
            undecorate(element)
        };
        let mirror_ref = clazz_ref(owner)?;
        let module_ref = HEAP.get_object_field_value(mirror_ref, "java/lang/Class", "module")?[0];
        let loader_ref =
            HEAP.get_object_field_value(mirror_ref, "java/lang/Class", "classLoader")?[0];

        let module_name_ref = if module_ref == 0 {
            0
        } else {
            HEAP.get_object_field_value(module_ref, "java/lang/Module", "name")?[0]
        };
        let module_name = if module_name_ref == 0 {
            None
        } else {
            Some(get_utf8_string_by_ref(module_name_ref)?)
        };

        let loader_name = if loader_ref == 0 {
            "'bootstrap'".to_string()
        } else {
            let name_and_id_ref =
                HEAP.get_object_field_value(loader_ref, "java/lang/ClassLoader", "nameAndId")?[0];
            get_utf8_string_by_ref(name_and_id_ref)?
        };

        Ok(Self {
            module_ref,
            module_name,
            loader_ref,
            loader_name,
        })
    }

    /// Each class loader has its own unnamed module in HotSpot, while rusty-jvm shares one
    /// between all loaders, so classes in it are only in the same module with the same loader.
    fn is_same_module(&self, other: &Self) -> bool {
        self.module_ref == other.module_ref
            && (self.module_name.is_some() || self.loader_ref == other.loader_ref)
    }

    /// E.g. `module java.base of loader 'bootstrap'` or `unnamed module of loader 'app'`.
    /// Unlike HotSpot, this leaves out the version of non-JDK named modules.
    fn describe(&self) -> String {
        match &self.module_name {
            Some(name) => format!("module {name} of loader {}", self.loader_name),
            None => format!("unnamed module of loader {}", self.loader_name),
        }
    }
}
