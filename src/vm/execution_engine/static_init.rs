use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::engine::Engine;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::method_area::instance_checker::InstanceChecker;
use crate::vm::method_area::java_class::{InnerState, JavaClass};
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::method_area::lookup;
use crate::vm::stack::stack_value::StackValueKind;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use std::sync::atomic::AtomicU32;
use tracing::trace;

pub struct StaticInit {}

static COUNTER: AtomicU32 = AtomicU32::new(0);

impl StaticInit {
    const STATIC_INIT_METHOD: &'static str = "<clinit>:()V";
    pub fn initialize(java_class_name: &str) -> Result<()> {
        let java_class = CLASSES.get(java_class_name)?;
        Self::initialize_java_class(&java_class)
    }

    pub fn initialize_java_class(java_class: &JavaClass) -> Result<()> {
        Self::initialization_impl(java_class)
    }

    fn initialization_impl(java_class: &JavaClass) -> Result<()> {
        // The lock is held while the state is read and changed and while the superclass and
        // `<clinit>` run, so that other threads wait for the outcome (JVMS §5.5, step 2). It is not
        // held while a failure is described or its errors are built: that runs Java code of the
        // failed initializer's own exception, which may well use the class from another thread,
        // and that thread must then find the class erroneous rather than wait for this one.
        let failure = {
            let guard = java_class.static_fields_init_state().lock();
            match guard.get_inner_state() {
                InnerState::Initialized => return Ok(()), // already initialized, nothing to do
                InnerState::Initializing => {
                    // initialization is in progress, doing nothing, just log
                    trace!("{}: recursively initializing", java_class.this_class_name());
                    return Ok(());
                }
                InnerState::Erroneous => {
                    // JVMS §5.5, step 5: a class whose initialization failed before fails every
                    // attempt since with a NoClassDefFoundError, caused by what it died of then
                    let failure = guard.failure();
                    drop(guard);
                    let error_ref = Self::could_not_initialize(java_class, failure)?;
                    return Err(Error::uncaught_exception(error_ref));
                }
                InnerState::NotInitialized => {
                    guard.set_inner_state(InnerState::Initializing);
                    match Self::initialize_superclass_and_run_initializer(java_class) {
                        Ok(()) => {
                            guard.set_inner_state(InnerState::Initialized);
                            return Ok(());
                        }
                        Err(error) => {
                            guard.set_inner_state(InnerState::Erroneous);
                            error
                        }
                    }
                }
            }
        };

        // The class is erroneous and the lock released: what it died of is what its later uses
        // are told, whether its own initializer threw or its superclass's did (JVMS §5.5, step 7)
        if let Some(throwable_ref) = failure.throwable_ref() {
            let description = Self::describe_failure(throwable_ref);
            java_class
                .static_fields_init_state()
                .lock()
                .set_failure(description);
        }
        Err(Self::initialization_failure(java_class, failure)?)
    }

    /// JVMS §5.5, steps 7 and 9: the superclass first, whose failure is this class's failure with
    /// the very same exception, then this class's own `<clinit>`.
    fn initialize_superclass_and_run_initializer(java_class: &JavaClass) -> Result<()> {
        if let Some(parent_name) = java_class.parent() {
            let klass = CLASSES.get(parent_name)?;
            Self::initialization_impl(&klass)?;
        }

        let curr_counter = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        trace!(
            ">>> {curr_counter} initializing {}",
            java_class.this_class_name()
        );

        if let Some(static_init_method) = java_class.try_get_method(Self::STATIC_INIT_METHOD) {
            Engine::execute(
                static_init_method.new_stack_frame()?,
                &format!("static initialization {}", java_class.this_class_name()),
            )?;
        }

        trace!(
            "<<< {curr_counter} initialised {}",
            java_class.this_class_name()
        );
        Ok(())
    }

    /// JVMS §5.5, steps 10 and 11: an exception out of `<clinit>` is passed on as it is when it is
    /// an `Error`, and wrapped in an `ExceptionInInitializerError` otherwise. A failure of the VM
    /// itself is passed on untouched.
    fn initialization_failure(java_class: &JavaClass, error: Error) -> Result<Error> {
        let Some(throwable_ref) = error.throwable_ref() else {
            return Ok(error);
        };
        let throwable_class = HEAP.get_instance_name(throwable_ref)?;
        if InstanceChecker::checkcast(&throwable_class, "java/lang/Error")? {
            return Ok(error);
        }
        let wrapped_ref = Executor::invoke_args_constructor(
            "java/lang/ExceptionInInitializerError",
            "<init>:(Ljava/lang/Throwable;)V",
            &[throwable_ref.into()],
            Some(&format!(
                "wrapping the initialization failure of {}",
                java_class.this_class_name()
            )),
        )?;
        Ok(Error::uncaught_exception(wrapped_ref))
    }

    /// HotSpot's record of what an initialization died of, `Exception <throwable> [in thread
    /// "<name>"]`, which becomes the message of the cause of every later `NoClassDefFoundError`.
    ///
    /// The throwable is rendered the way `Throwable.toString` renders it by default, from its
    /// class name and its detail message, without calling `toString` itself: that may be
    /// overridden, and HotSpot runs nothing of the exception's own here either. Best effort
    /// throughout, as nothing about describing the failure may replace the failure itself.
    fn describe_failure(throwable_ref: i32) -> String {
        let class_name = HEAP
            .get_instance_name(throwable_ref)
            .unwrap_or_else(|_| "?".to_string());
        let description = match Self::string_field(throwable_ref, &class_name, "detailMessage") {
            Some(message) => format!("{}: {message}", class_name.replace('/', ".")),
            None => class_name.replace('/', "."),
        };
        let thread_name = JavaThread::current_thread()
            .and_then(|thread_ref| {
                let thread_class = HEAP.get_instance_name(thread_ref).ok()?;
                Self::string_field(thread_ref, &thread_class, "name")
            })
            .unwrap_or_else(|| "main".to_string());
        format!("Exception {description} [in thread \"{thread_name}\"]")
    }

    /// The value of a `String` field of `object_ref`, or `None` for `null` or any failure.
    fn string_field(object_ref: i32, class_name: &str, field_name: &str) -> Option<String> {
        let string_ref = *HEAP
            .get_object_field_value(object_ref, class_name, field_name)
            .ok()?
            .first()?;
        if string_ref == 0 {
            return None;
        }
        get_utf8_string_by_ref(string_ref).ok()
    }

    /// The `NoClassDefFoundError` for a class that failed to initialize before, worded and caused
    /// as HotSpot's: `Could not initialize class <name>`, caused by an
    /// `ExceptionInInitializerError` that carries the description of the original failure.
    fn could_not_initialize(java_class: &JavaClass, failure: Option<String>) -> Result<i32> {
        let message = format!(
            "Could not initialize class {}",
            java_class.this_class_name().replace('/', ".")
        );
        let message_ref = StringPoolHelper::create_string(&message)?;
        let error_ref = Executor::invoke_args_constructor(
            "java/lang/NoClassDefFoundError",
            "<init>:(Ljava/lang/String;)V",
            &[message_ref.into()],
            Some("NoClassDefFoundError for a class that failed to initialize"),
        )?;
        if let Some(failure) = failure {
            let failure_ref = StringPoolHelper::create_string(&failure)?;
            let cause_ref = Executor::invoke_args_constructor(
                "java/lang/ExceptionInInitializerError",
                "<init>:(Ljava/lang/String;)V",
                &[failure_ref.into()],
                Some("cause of the NoClassDefFoundError"),
            )?;
            Self::invoke_virtual(
                error_ref,
                "initCause:(Ljava/lang/Throwable;)Ljava/lang/Throwable;",
                &[cause_ref.into()],
            )?;
        }
        Ok(error_ref)
    }

    /// Calls `name_signature` on `receiver_ref` as `invokevirtual` would, resolving it in the
    /// receiver's class hierarchy first: the executor expects a resolved method, and the ones
    /// used here, `toString` and `initCause`, live in `Throwable`, not in the exception's class.
    fn invoke_virtual(
        receiver_ref: i32,
        name_signature: &str,
        args: &[StackValueKind],
    ) -> Result<i32> {
        let receiver_class = HEAP.get_instance_name(receiver_ref)?;
        let method =
            lookup::lookup_instance_method(&receiver_class, name_signature)?.ok_or_else(|| {
                Error::new_execution(&format!(
                    "{name_signature} not found in the hierarchy of {receiver_class}"
                ))
            })?;
        Ok(Executor::invoke_non_static_method(
            method.class_name(),
            name_signature,
            receiver_ref,
            args,
        )?[0])
    }
}
