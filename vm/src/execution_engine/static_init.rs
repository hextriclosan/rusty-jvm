use crate::execution_engine::engine::Engine;
use crate::method_area::java_class::JavaClass;
use crate::method_area::method_area::with_method_area;
use std::sync::atomic::Ordering;

pub struct StaticInit {}

impl StaticInit {
    const STATIC_INIT_METHOD: &'static str = "<clinit>:()V";
    pub fn initialize(java_class_name: &str) -> crate::error::Result<()> {
        let java_class = with_method_area(|area| area.get(java_class_name))?;
        Self::initialize_java_class(&java_class)
    }

    pub fn initialize_java_class(java_class: &JavaClass) -> crate::error::Result<()> {
        Self::initialization_impl(&java_class)
    }

    fn initialization_impl(java_class: &JavaClass) -> crate::error::Result<()> {
        if java_class
            .static_fields_initialized()
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            let hierarchy = java_class.instance_fields_hierarchy()?;
            for name in hierarchy.keys().take(hierarchy.len() - 1) {
                // skip last one, it will be initialized below
                let jc = with_method_area(|area| area.get(name))?;
                Self::initialization_impl(&jc)?;
            }

            //todo: protect me with recursive mutex
            if let Some(static_init_method) = java_class.try_get_method(Self::STATIC_INIT_METHOD) {
                Engine::execute(
                    static_init_method.new_stack_frame()?,
                    &format!("static initialization {}", java_class.this_class_name()),
                )?;
            }
        }

        Ok(())
    }
}
