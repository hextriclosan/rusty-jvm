use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::engine::Engine;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::method_area::instance_checker::InstanceChecker;
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::stack::stack_value::StackValueKind;

pub struct StaticInit {}

impl StaticInit {
    pub fn initialize(java_class_name: &str) -> Result<()> {
        let java_class = CLASSES.get(java_class_name)?;
        Self::initialize_java_class(&java_class)
    }

    pub fn initialize_java_class(java_class: &JavaClass) -> Result<()> {
        let state = java_class.static_fields_init_state();
        match state.begin() {
            Ok(false) => return Ok(()),
            Ok(true) => {}
            Err(cause_root) => {
                if cause_root == 0 {
                    return Err(Error::new_execution(
                        "class initialization previously failed",
                    ));
                }
                let message = StringPoolHelper::get_string(java_class.this_class_name())?;
                let failure = Executor::invoke_args_constructor(
                    "java/lang/NoClassDefFoundError",
                    "<init>:(Ljava/lang/String;)V",
                    &[StackValueKind::Ref(message)],
                    None,
                )?;
                Executor::invoke_non_static_method(
                    "java/lang/Throwable",
                    "initCause:(Ljava/lang/Throwable;)Ljava/lang/Throwable;",
                    failure[0].value(),
                    &[StackValueKind::Ref(cause_root)],
                )?;
                return Err(Error::uncaught_exception(failure[0]));
            }
        }

        // No initialization mutex is held here, including during parent initialization.
        let result = (|| -> Result<()> {
            if let Some(parent_name) = java_class.parent() {
                Self::initialize(parent_name)?;
            }
            if let Some(method) = java_class.try_get_method("<clinit>:()V") {
                Engine::execute(method.new_stack_frame()?, "class initialization")?;
            }
            Ok(())
        })();
        match result {
            Ok(()) => {
                state.finish(None);
                Ok(())
            }
            Err(error) => {
                // Publish the persistent cause before releasing the Error's transfer root.
                state.finish(Some(error.throwable_ref().unwrap_or(0)));
                let Some(cause) = error.throwable_ref() else {
                    return Err(error);
                };
                let name = HEAP.get_instance_name(cause)?;
                if InstanceChecker::checkcast(&name, "java/lang/Error")? {
                    return Err(error);
                }
                let wrapped = Executor::invoke_args_constructor(
                    "java/lang/ExceptionInInitializerError",
                    "<init>:(Ljava/lang/Throwable;)V",
                    &[StackValueKind::Ref(cause)],
                    None,
                )?;
                Err(Error::uncaught_exception(wrapped[0]))
            }
        }
    }
}
