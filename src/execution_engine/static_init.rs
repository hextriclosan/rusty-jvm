use crate::execution_engine::engine::Engine;
use crate::method_area::java_class::{InnerState, JavaClass};
use crate::method_area::method_area::with_method_area;
use std::sync::atomic::AtomicU32;
use tracing::trace;

pub struct StaticInit {}

static COUNTER: AtomicU32 = AtomicU32::new(0);

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
        let guard = java_class.static_fields_init_state().lock();

        match guard.get_inner_state() {
            InnerState::Initialized => {
                // already initialized doing nothing
            }
            InnerState::Initializing => {
                // initialization is in progress, doing nothing, just log
                trace!("{}: recursively initializing", java_class.this_class_name());
            }
            InnerState::NotInitialized => {
                guard.set_inner_state(InnerState::Initializing);

                if let Some(parent_name) = java_class.parent() {
                    let jc = with_method_area(|area| area.get(parent_name))?;
                    Self::initialization_impl(&jc)?;
                }

                let curr_counter = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                trace!(
                    ">>> {curr_counter} initializing {}",
                    java_class.this_class_name()
                );

                if let Some(static_init_method) =
                    java_class.try_get_method(Self::STATIC_INIT_METHOD)
                {
                    Engine::execute(
                        static_init_method.new_stack_frame()?,
                        &format!("static initialization {}", java_class.this_class_name()),
                    )?;
                }

                guard.set_inner_state(InnerState::Initialized);

                trace!(
                    "<<< {curr_counter} initialised {}",
                    java_class.this_class_name()
                );
            }
        }

        Ok(())
    }
}
