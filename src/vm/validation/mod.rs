use crate::vm::error::{Error, Result};

/// Basic (and naïve) validation for the standard library directory path.
pub fn validate_std_dir(dir_path: &str) -> Result<()> {
    if dir_path.is_empty() {
        return Err(Error::new_execution(
            "Standard library directory path cannot be empty",
        ));
    }

    let path = std::path::Path::new(dir_path);
    if !path.exists() {
        return Err(Error::new_execution(&format!(
            "Standard library directory does not exist: {}",
            dir_path
        )));
    }

    if !path.is_dir() {
        return Err(Error::new_execution(&format!(
            "Path is not a directory: {}",
            dir_path
        )));
    }

    let is_empty = std::fs::read_dir(path)
        .map_err(|e| Error::new_execution(&format!("Failed to read directory: {}", e)))?
        .next()
        .is_none();

    if is_empty {
        return Err(Error::new_execution(&format!(
            "Standard library directory is empty: {}",
            dir_path
        )));
    }

    Ok(())
}

pub fn validate_class_name(class_name: &str) -> Result<()> {
    if class_name.is_empty() {
        return Err(Error::new_execution("Class name cannot be empty"));
    }

    Ok(())
}
