use crate::vm::error::{Error, Result};
use crate::vm::properties::wildcard::wildcards_to_classpath;
use crate::Arguments;
use once_cell::sync::OnceCell;
use std::env;

static CLASSPATH: OnceCell<String> = OnceCell::new();

pub(crate) fn resolve_classpath(arguments: &Arguments) -> Result<()> {
    let mut classpath = String::from(".");

    if let Ok(cp) = env::var("CLASSPATH") {
        classpath = cp;
    }

    if let Some(cp) = arguments.classpath() {
        classpath = cp.clone();
    }

    if *arguments.jar_mode() {
        classpath = arguments.entry_point().clone();
    }

    classpath = wildcards_to_classpath(&classpath)?;

    CLASSPATH.set(classpath).map_err(|existing_value| {
        Error::new_execution(&format!(
            "CLASSPATH already initialized: {existing_value:?}"
        ))
    })?;
    Ok(())
}

pub(crate) fn classpath() -> &'static str {
    let cp = CLASSPATH.get()
        .expect("CLASSPATH is not initialized. This should never happen as resolve_classpath should be called during JVM initialization.");

    cp.as_str()
}
