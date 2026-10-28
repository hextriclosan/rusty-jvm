//! Java assertion status, set by the `-ea`/`-da`/`-esa`/`-dsa` launcher options and resolved
//! for each class the way HotSpot's `JavaAssertions` does.

use crate::vm::error::{Error, Result};
use std::sync::OnceLock;

static JAVA_ASSERTIONS: OnceLock<JavaAssertions> = OnceLock::new();

/// Options taking an optional `:<package>...` or `:<class>` argument.
const USER_ASSERTION_OPTIONS: [&str; 4] =
    ["-da", "-ea", "-disableassertions", "-enableassertions"];
/// Options for classes loaded by the bootstrap class loader, which take no argument.
const SYSTEM_ASSERTION_OPTIONS: [&str; 4] = [
    "-dsa",
    "-esa",
    "-disablesystemassertions",
    "-enablesystemassertions",
];

/// A class or package named by an assertion option, in internal form (`java/lang/Thread`).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AssertionOption {
    pub(crate) name: String,
    pub(crate) enabled: bool,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct JavaAssertions {
    /// Default for classes not loaded by the bootstrap class loader (`-ea`/`-da` alone).
    user_default: bool,
    /// Default for classes loaded by the bootstrap class loader (`-esa`/`-dsa`).
    system_default: bool,
    /// Class options in command-line order; later ones take precedence.
    classes: Vec<AssertionOption>,
    /// Package options (`-ea:<package>...`) in command-line order; later ones take precedence.
    /// The unnamed package is the empty name (`-ea:...`).
    packages: Vec<AssertionOption>,
}

/// Records the assertion options among the VM's `options`, once per VM.
pub(crate) fn init_java_assertions(options: &[String]) -> Result<()> {
    JAVA_ASSERTIONS
        .set(JavaAssertions::from_options(options))
        .map_err(|e| Error::new_execution(&format!("Java assertions already set: {e:?}")))
}

pub(crate) fn java_assertions() -> &'static JavaAssertions {
    JAVA_ASSERTIONS.get_or_init(JavaAssertions::default)
}

impl JavaAssertions {
    /// Applies the assertion options among `options` in order, ignoring all other options.
    pub(crate) fn from_options(options: &[String]) -> Self {
        let mut assertions = Self::default();
        for option in options {
            // the char after '-' tells enabling from disabling
            let enable = option.as_bytes().get(1) == Some(&b'e');
            if let Some(tail) = match_option(option, &USER_ASSERTION_OPTIONS, true) {
                match tail.strip_prefix(':') {
                    Some(name) => assertions.add_option(name, enable),
                    None => assertions.user_default = enable,
                }
            } else if match_option(option, &SYSTEM_ASSERTION_OPTIONS, false).is_some() {
                assertions.system_default = enable;
            }
        }
        assertions
    }

    fn add_option(&mut self, name: &str, enabled: bool) {
        let (list, name) = match name.strip_suffix("...") {
            Some(package) => (&mut self.packages, package),
            None => (&mut self.classes, name),
        };
        list.push(AssertionOption {
            name: name.replace('.', "/"),
            enabled,
        });
    }

    /// Whether assertions are enabled for `class_name` (internal form), loaded by the bootstrap
    /// class loader if `system_class`.
    pub(crate) fn enabled(&self, class_name: &str, system_class: bool) -> bool {
        if let Some(option) = self.match_class(class_name) {
            return option.enabled;
        }
        if let Some(option) = self.match_package(class_name) {
            return option.enabled;
        }
        if system_class {
            self.system_default
        } else {
            self.user_default
        }
    }

    fn match_class(&self, class_name: &str) -> Option<&AssertionOption> {
        self.classes
            .iter()
            .rev()
            .find(|option| option.name == class_name)
    }

    /// Checks each package of `class_name`, from the most specific to the least. As in HotSpot,
    /// the unnamed package only applies to classes that are in it.
    fn match_package(&self, class_name: &str) -> Option<&AssertionOption> {
        let mut package = class_name
            .rsplit_once('/')
            .map_or("", |(package, _)| package);
        loop {
            let found = self
                .packages
                .iter()
                .rev()
                .find(|option| option.name == package);
            if found.is_some() {
                return found;
            }
            match package.rsplit_once('/') {
                Some((parent, _)) => package = parent,
                None => return None,
            }
        }
    }

    pub(crate) fn user_default(&self) -> bool {
        self.user_default
    }

    pub(crate) fn classes(&self) -> &[AssertionOption] {
        &self.classes
    }

    pub(crate) fn packages(&self) -> &[AssertionOption] {
        &self.packages
    }
}

/// Returns the rest of `option` after the first of `names` it starts with, if that rest is empty
/// or, when `tail_allowed`, starts with ':'.
fn match_option<'a>(option: &'a str, names: &[&str], tail_allowed: bool) -> Option<&'a str> {
    names.iter().find_map(|name| {
        let tail = option.strip_prefix(name)?;
        (tail.is_empty() || (tail_allowed && tail.starts_with(':'))).then_some(tail)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assertions(options: &[&str]) -> JavaAssertions {
        JavaAssertions::from_options(&options.iter().map(|o| o.to_string()).collect::<Vec<_>>())
    }

    const USER_CLASS: &str = "com/example/app/Main";
    const SYSTEM_CLASS: &str = "java/lang/String";

    #[test]
    fn assertions_are_disabled_by_default() {
        let a = assertions(&[]);
        assert!(!a.enabled(USER_CLASS, false));
        assert!(!a.enabled(SYSTEM_CLASS, true));
    }

    #[test]
    fn ea_enables_user_classes_only() {
        for option in ["-ea", "-enableassertions"] {
            let a = assertions(&[option]);
            assert!(a.enabled(USER_CLASS, false), "{option}");
            assert!(!a.enabled(SYSTEM_CLASS, true), "{option}");
        }
    }

    #[test]
    fn esa_enables_system_classes_only() {
        for option in ["-esa", "-enablesystemassertions"] {
            let a = assertions(&[option]);
            assert!(!a.enabled(USER_CLASS, false), "{option}");
            assert!(a.enabled(SYSTEM_CLASS, true), "{option}");
        }
    }

    #[test]
    fn later_defaults_override_earlier_ones() {
        assert!(!assertions(&["-ea", "-da"]).enabled(USER_CLASS, false));
        assert!(!assertions(&["-ea", "-disableassertions"]).enabled(USER_CLASS, false));
        assert!(!assertions(&["-esa", "-dsa"]).enabled(SYSTEM_CLASS, true));
        assert!(!assertions(&["-esa", "-disablesystemassertions"]).enabled(SYSTEM_CLASS, true));
    }

    #[test]
    fn package_option_covers_its_subpackages() {
        let a = assertions(&["-ea:com.example..."]);
        assert!(a.enabled("com/example/Main", false));
        assert!(a.enabled(USER_CLASS, false));
        assert!(!a.enabled("com/other/Main", false));
        assert!(!a.enabled("com/examples/Main", false));
    }

    #[test]
    fn package_option_applies_to_system_classes_too() {
        let a = assertions(&["-ea:java.lang..."]);
        assert!(a.enabled(SYSTEM_CLASS, true));
        assert!(!a.enabled("java/util/List", true));
    }

    #[test]
    fn class_option_applies_to_that_class_only() {
        let a = assertions(&["-ea:com.example.app.Main"]);
        assert!(a.enabled(USER_CLASS, false));
        assert!(!a.enabled("com/example/app/Other", false));
        assert!(!a.enabled("com/example/app/Main$Inner", false));
    }

    #[test]
    fn name_without_dots_is_a_class_not_a_package() {
        let a = assertions(&["-ea:com.example"]);
        assert!(a.enabled("com/example", false));
        assert!(!a.enabled("com/example/Main", false));
        assert!(!a.enabled(USER_CLASS, false));
    }

    #[test]
    fn class_option_applies_to_system_classes_too() {
        let a = assertions(&["-ea:java.lang.String"]);
        assert!(a.enabled(SYSTEM_CLASS, true));
        assert!(!a.enabled("java/lang/Integer", true));
    }

    #[test]
    fn nested_class_is_a_class_of_its_own() {
        let a = assertions(&["-ea:com.example.app.Main$Inner"]);
        assert!(a.enabled("com/example/app/Main$Inner", false));
        assert!(!a.enabled(USER_CLASS, false));
    }

    #[test]
    fn empty_name_matches_no_class() {
        let a = assertions(&["-ea:"]);
        assert!(!a.enabled("Main", false));
        assert!(!a.enabled(USER_CLASS, false));
        assert!(!a.enabled(SYSTEM_CLASS, true));
    }

    #[test]
    fn most_specific_option_wins() {
        let a = assertions(&[
            "-ea:com.example.app.Main",
            "-da:com.example...",
            "-ea:com...",
        ]);
        assert!(a.enabled(USER_CLASS, false));
        assert!(!a.enabled("com/example/app/Other", false));
        assert!(a.enabled("com/other/Main", false));

        let a = assertions(&["-ea", "-da:com.example.app..."]);
        assert!(!a.enabled(USER_CLASS, false));
        assert!(a.enabled("com/example/Main", false));
    }

    #[test]
    fn later_option_for_the_same_name_wins() {
        let a = assertions(&["-ea:com.example...", "-da:com.example..."]);
        assert!(!a.enabled(USER_CLASS, false));
        let a = assertions(&["-da:com.example.app.Main", "-ea:com.example.app.Main"]);
        assert!(a.enabled(USER_CLASS, false));
    }

    #[test]
    fn unnamed_package_option_applies_only_to_classes_in_it() {
        let a = assertions(&["-ea:..."]);
        assert!(a.enabled("Main", false));
        assert!(!a.enabled(USER_CLASS, false));
    }

    #[test]
    fn long_forms_take_arguments_like_short_ones() {
        let a = assertions(&[
            "-enableassertions:com.example...",
            "-disableassertions:com.example.app.Main",
        ]);
        assert!(a.enabled("com/example/Main", false));
        assert!(!a.enabled(USER_CLASS, false));
    }

    #[test]
    fn options_are_matched_exactly() {
        // not assertion options: arguments to system options, or other options sharing a prefix
        for option in [
            "-esa:java.lang...",
            "-eax",
            "-ea=com.example",
            "-dsax",
            "-verbose",
        ] {
            assert_eq!(assertions(&[option]), JavaAssertions::default(), "{option}");
        }
    }

    #[test]
    fn options_keep_command_line_order() {
        let a = assertions(&["-ea:a.B", "-da:c...", "-da:a.B", "-ea:d.e..."]);
        let names = |options: &[AssertionOption]| {
            options
                .iter()
                .map(|o| (o.name.clone(), o.enabled))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            names(a.classes()),
            [("a/B".to_string(), true), ("a/B".to_string(), false)]
        );
        assert_eq!(
            names(a.packages()),
            [("c".to_string(), false), ("d/e".to_string(), true)]
        );
    }
}
