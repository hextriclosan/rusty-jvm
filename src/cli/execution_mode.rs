use indexmap::IndexMap;
use rusty_jvm::Arguments;

#[derive(PartialEq, Debug)]
/// Represents the mode in which the Java program is running.
pub(crate) enum ExecutionMode {
    /// Normal mode, used for running the Java program.
    Normal(Arguments),
    /// Install mode, used for installing the Java program.
    Install(bool),
    /// Purge mode, used for uninstalling the Java program.
    Purge(bool),
}

/// Converts a vector of raw command-line arguments into an `ExecutionMode`.
impl From<Vec<String>> for ExecutionMode {
    fn from(raw_args: Vec<String>) -> Self {
        let mut java_standard_options = Vec::new();
        let mut java_launcher_options = Vec::new();
        let mut system_properties = IndexMap::new();
        let mut jvm_options = Vec::new();
        let mut advanced_jvm_options = Vec::new();
        let mut entry_point = String::new();
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
                entry_point = arg;
                break;
            }
        }

        program_args.extend(iter);

        if is_install_mode(&java_launcher_options) {
            ExecutionMode::Install(is_yes(&java_launcher_options))
        } else if is_purge_mode(&java_launcher_options) {
            ExecutionMode::Purge(is_yes(&java_launcher_options))
        } else {
            ExecutionMode::Normal(Arguments::new(
                entry_point,
                program_args,
                java_standard_options,
                java_launcher_options,
                system_properties,
                jvm_options,
                advanced_jvm_options,
            ))
        }
    }
}

/// Checks if it is running in install mode.
fn is_install_mode(options: &[String]) -> bool {
    java_launcher_option_is_set(options, "--install")
}

/// Checks if it is running in purge mode.
fn is_purge_mode(options: &[String]) -> bool {
    java_launcher_option_is_set(options, "--purge")
}

/// For install/purge modes checks if the `--yes` option is set, which indicates automatic confirmation for prompts.
fn is_yes(options: &[String]) -> bool {
    java_launcher_option_is_set(options, "--yes")
}

/// Checks if a specific Java launcher option is set in the provided options.
fn java_launcher_option_is_set(options: &[String], option: &str) -> bool {
    options.iter().any(|v| v == option)
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

        let parsed: ExecutionMode = raw_args.into();
        let expected = ExecutionMode::Normal(Arguments::new(
            "MainClass".to_string(),
            vec!["arg1".to_string(), "arg2".to_string()],
            vec!["-version".to_string()],
            vec!["--launcher-option".to_string()],
            IndexMap::from([
                ("property1".to_string(), "value1".to_string()),
                ("property2".to_string(), "value2".to_string()),
            ]),
            vec!["-Xmx1024m".to_string()],
            vec!["-XX:+UseG1GC".to_string()],
        ));

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

        let parsed: ExecutionMode = raw_args.into();
        let expected = ExecutionMode::Normal(Arguments::new(
            "".to_string(),
            vec![],
            vec!["-version".to_string()],
            vec!["--launcher-option".to_string()],
            IndexMap::from([("property".to_string(), "value".to_string())]),
            vec!["-Xmx1024m".to_string()],
            vec!["-XX:+UseG1GC".to_string()],
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_empty_args() {
        let parsed: ExecutionMode = vec![].into();
        let expected = ExecutionMode::Normal(Arguments::new(
            "".to_string(),
            vec![],
            vec![],
            vec![],
            IndexMap::new(),
            vec![],
            vec![],
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_install_mode() {
        let raw_args = vec!["--install".to_string(), "--yes".to_string()];
        let parsed: ExecutionMode = raw_args.into();
        let expected = ExecutionMode::Install(true);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_purge_mode() {
        let raw_args = vec!["--purge".to_string(), "--yes".to_string()];
        let parsed: ExecutionMode = raw_args.into();
        let expected = ExecutionMode::Purge(true);
        assert_eq!(parsed, expected);
    }
}
