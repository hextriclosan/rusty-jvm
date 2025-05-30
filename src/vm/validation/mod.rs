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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn validate_std_dir_with_valid_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path().to_str().unwrap().to_string();
        fs::create_dir_all(&temp_dir_path).unwrap();
        fs::write(Path::new(&temp_dir_path).join("file.txt"), "content").unwrap();

        assert!(validate_std_dir(&temp_dir_path).is_ok());
    }

    #[test]
    fn validate_std_dir_with_empty_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path().to_str().unwrap().to_string();

        let result = validate_std_dir(&temp_dir_path);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Execution Error: Standard library directory is empty: {}",
                temp_dir_path
            )
        );
    }

    #[test]
    fn validate_std_dir_with_nonexistent_directory() {
        let result = validate_std_dir("nonexistent_dir");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Execution Error: Standard library directory does not exist: nonexistent_dir"
        );
    }

    #[test]
    fn validate_std_dir_with_file_instead_of_directory() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap().to_string();

        let result = validate_std_dir(&file_path);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!("Execution Error: Path is not a directory: {}", file_path)
        );
    }

    #[test]
    fn validate_std_dir_with_empty_path() {
        let result = validate_std_dir("");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Execution Error: Standard library directory path cannot be empty"
        );
    }

    #[test]
    fn validate_class_name_with_valid_name() {
        assert!(validate_class_name("ValidClassName").is_ok());
    }

    #[test]
    fn validate_class_name_with_empty_name() {
        let result = validate_class_name("");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Execution Error: Class name cannot be empty"
        );
    }
}
