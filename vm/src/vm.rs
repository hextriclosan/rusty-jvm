use crate::error::Error;
use crate::execution_engine::executor::Executor;
use crate::execution_engine::string_pool_helper::StringPoolHelper;
use crate::method_area::method_area::MethodArea;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

pub struct VM {}

impl VM {
    const JAVA_HOME_ENV: &'static str = "RUSTY_JAVA_HOME";

    pub fn run(main_class_name: &str) -> crate::error::Result<()> {
        Self::prelude()?;

        let internal_name = &main_class_name.replace('.', "/");
        Executor::invoke_static_method(internal_name, "main:([Ljava/lang/String;)V", &[])
    }

    fn prelude() -> crate::error::Result<()> {
        Self::init_logger()?;

        let std_dir = Self::get_std_dir()?;
        MethodArea::init(&std_dir)?;

        Self::init()?;

        Ok(())
    }

    fn init_logger() -> crate::error::Result<()> {
        let fmt_layer = fmt::layer()
            .with_target(false)
            .without_time()
            .with_ansi(false);
        let filter_layer = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new("info"))
            .map_err(|e| Error::new_execution(&format!("Error creating EnvFilter: {e}")))?;
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .init();

        Ok(())
    }

    fn get_std_dir() -> crate::error::Result<String> {
        if let Ok(rusty_java_home) = std::env::var(Self::JAVA_HOME_ENV) {
            Ok(format!("{}/lib", rusty_java_home))
        } else {
            Err(Error::new_execution(&format!(
                "{} environment variable is not set",
                Self::JAVA_HOME_ENV
            )))
        }
    }

    fn init() -> crate::error::Result<()> {
        // explicit static initialization of java.lang.ref.Reference needed for creating JavaLangRefAccess instance
        Executor::do_static_fields_initialization("java/lang/ref/Reference")?;
        Executor::do_static_fields_initialization("java/lang/ref/Cleaner")?;
        Executor::do_static_fields_initialization("java/lang/reflect/AccessibleObject")?;

        // create primordial ThreadGroup and Thread
        let tg_obj_ref = Executor::invoke_default_constructor("java/lang/ThreadGroup")?;
        let string_obj_ref = StringPoolHelper::get_string("system".to_string())?; // refactor candidate B: introduce and use here common string creator, not string pool one
        let _thread_obj_ref =
            Executor::create_primordial_thread(&vec![tg_obj_ref.into(), string_obj_ref.into()])?;

        Executor::invoke_static_method("java/lang/System", "initPhase1:()V", &[])?;

        Ok(())
    }
}
