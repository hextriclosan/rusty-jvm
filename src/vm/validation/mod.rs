use crate::vm::error::{Error, Result};

pub fn validate_class_name(class_name: &str) -> Result<()> {
    if class_name.is_empty() {
        return Err(Error::new_execution("Class name cannot be empty"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
