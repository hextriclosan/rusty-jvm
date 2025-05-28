use derive_new::new;
use getset::Getters;
use std::collections::HashMap;

pub(crate) fn group_args(raw_args: Vec<String>) -> ParsedArguments {
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

    ParsedArguments::new(
        entry_point,
        program_args,
        java_standard_options,
        java_launcher_options,
        system_properties,
        jvm_options,
        advanced_jvm_options,
    )
}

#[derive(PartialEq, Debug, new, Getters)]
#[get = "pub"]
/// Represents the parsed command-line arguments for the Java program.
pub struct ParsedArguments {
    /// The entry point for the Java program.  
    /// This may be `None` when running in install or purge mode.
    entry_point: Option<String>,
    /// The arguments passed to the Java program.
    program_args: Vec<String>,
    /// The standard Java options (e.g., `-version`, `-help`).
    java_standard_options: Vec<String>,
    /// The options passed to the Java launcher (e.g., `--install`, `--purge`).
    java_launcher_options: Vec<String>,
    /// System properties set for the Java program (e.g., `-Dproperty=value`).
    system_properties: HashMap<String, String>,
    /// JVM options (e.g., `-Xmx1024m`).
    jvm_options: Vec<String>,
    /// Advanced JVM options (e.g., `-XX:+UseG1GC`).
    advanced_jvm_options: Vec<String>,
}

impl ParsedArguments {
    pub fn is_install_mode(&self) -> bool {
        self.java_launcher_option_is_set("--install")
    }

    pub fn is_purge_mode(&self) -> bool {
        self.java_launcher_option_is_set("--purge")
    }

    pub fn is_yes(&self) -> bool {
        self.java_launcher_option_is_set("--yes")
    }

    fn java_launcher_option_is_set(&self, option: &str) -> bool {
        if self.java_launcher_options().is_empty() {
            return false;
        }

        self.java_launcher_options().iter().any(|v| v == option)
    }
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

        let parsed = group_args(raw_args);
        let expected = ParsedArguments::new(
            Some("MainClass".to_string()),
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

        let parsed = group_args(raw_args);
        let expected = ParsedArguments::new(
            None,
            vec![],
            vec!["-version".to_string()],
            vec!["--launcher-option".to_string()],
            HashMap::from([("property".to_string(), "value".to_string())]),
            vec!["-Xmx1024m".to_string()],
            vec!["-XX:+UseG1GC".to_string()],
        );
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_empty_args() {
        let parsed = group_args(vec![]);
        let expected =
            ParsedArguments::new(None, vec![], vec![], vec![], HashMap::new(), vec![], vec![]);
        assert_eq!(parsed, expected);
    }
}
