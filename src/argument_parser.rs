use derive_new::new;
use getset::Getters;
use std::collections::HashMap;
use crate::argument_parser::ParserError::NoEntryPointProvided;

#[derive(Debug, PartialEq)]
pub(crate) enum ParserError {
    NoEntryPointProvided,
}

pub(crate) fn group_args(raw_args: Vec<String>) -> Result<ParsedArguments, ParserError> {
    let mut java_standard_options = Vec::new();
    let mut java_launcher_options = Vec::new();
    let mut system_properties = HashMap::new();
    let mut jvm_options = Vec::new();
    let mut advanced_jvm_options = Vec::new();
    let mut entry_point = None;
    let mut program_args = Vec::new();

    let mut iter = raw_args.into_iter().peekable();
    while let Some(arg) = iter.next() {
        if arg.starts_with('-') {
            if arg.starts_with("-XX:") {
                advanced_jvm_options.push(arg);
            } else if arg.starts_with("-X") {
                jvm_options.push(arg);
            } else if arg.starts_with("-D") {
                let trimmed = &arg[2..];
                let (key, value) = match trimmed.split_once('=') {
                    Some((key, value)) => (key, value),
                    None => (trimmed, ""),
                };
                system_properties.insert(key.to_string(), value.to_string());
            } else if arg.starts_with("--") {
                java_launcher_options.push(arg);
            } else {
                java_standard_options.push(arg);
            }
        } else {
            entry_point = Some(arg);
            break;
        }
    }

    program_args.extend(iter);

    let entry_point = entry_point.ok_or(NoEntryPointProvided)?;

    Ok(ParsedArguments::new(
        entry_point,
        program_args,
        java_standard_options,
        java_launcher_options,
        system_properties,
        jvm_options,
        advanced_jvm_options,
    ))
}

#[derive(PartialEq, Debug, new, Getters)]
#[get = "pub"]
pub struct ParsedArguments {
    entry_point: String,
    program_args: Vec<String>,
    java_standard_options: Vec<String>,
    java_launcher_options: Vec<String>,
    system_properties: HashMap<String, String>,
    jvm_options: Vec<String>,
    advanced_jvm_options: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_args() {
        let raw_args = vec![
            "-version".to_string(),
            "--launcher-option".to_string(),
            "-Dproperty1=value1".to_string(),
            "-Dproperty2=value2".to_string(),
            "-Xmx1024m".to_string(),
            "-XX:+UseG1GC".to_string(),
            "MainClass".to_string(),
            "arg1".to_string(),
            "arg2".to_string(),
        ];

        let parsed = group_args(raw_args).unwrap();
        let expected = ParsedArguments::new(
            "MainClass".to_string(),
            vec!["arg1".to_string(), "arg2".to_string()],
            vec!["-version".to_string()],
            vec!["--launcher-option".to_string()],
            HashMap::from([
                ("property1".to_string(), "value1".to_string()),
                ("property2".to_string(), "value2".to_string()),
            ]),
            vec!["-Xmx1024m".to_string()],
            vec!["-XX:+UseG1GC".to_string()],
        );

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_no_entry_point() {
        let raw_args = vec![
            "-version".to_string(),
            "--launcher-option".to_string(),
            "-Dproperty=value".to_string(),
            "-Xmx1024m".to_string(),
            "-XX:+UseG1GC".to_string(),
        ];

        let result = group_args(raw_args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NoEntryPointProvided);
    }

    #[test]
    fn test_empty_args() {
        let result = group_args(vec![]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NoEntryPointProvided);
    }
}
