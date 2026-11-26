//! Names (JVMS §4.2) and descriptors (JVMS §4.3), as far as format checking (JVMS §4.8) looks at
//! them: these functions tell whether a string is well formed, and resolve nothing.

/// The most dimensions an array type may have (JVMS §4.3.2).
const MAX_DIMENSIONS: usize = 255;
/// The longest method parameters may be, `long` and `double` counting twice (JVMS §4.3.3).
const MAX_PARAMETERS_LENGTH: usize = 255;

/// An unqualified name (JVMS §4.2.2), as fields, local variables and formal parameters have.
pub(crate) fn is_unqualified_name(name: &str) -> bool {
    !name.is_empty() && !name.chars().any(|c| matches!(c, '.' | ';' | '[' | '/'))
}

/// A method name (JVMS §4.2.2): an unqualified name without `<` and `>`, or one of the special
/// method names `<init>` and `<clinit>`.
pub(crate) fn is_method_name(name: &str) -> bool {
    name == "<init>"
        || name == "<clinit>"
        || is_unqualified_name(name) && !name.chars().any(|c| matches!(c, '<' | '>'))
}

/// A binary class or interface name encoded in internal form (JVMS §4.2.1): identifiers
/// separated by `/`. A package name encoded in internal form (JVMS §4.2.3) has the same shape.
pub(crate) fn is_binary_name(name: &str) -> bool {
    name.split('/').all(is_unqualified_name)
}

/// What a `CONSTANT_Class_info` structure may name (JVMS §4.4.1): a class or an interface by its
/// binary name, or an array type by its descriptor.
pub(crate) fn is_class_info_name(name: &str) -> bool {
    if name.starts_with('[') {
        is_field_descriptor(name)
    } else {
        is_binary_name(name)
    }
}

/// A module name (JVMS §4.2.3).
pub(crate) fn is_module_name(name: &str) -> bool {
    let mut chars = name.chars();

    while let Some(c) = chars.next() {
        let is_permitted = match c {
            '\u{0000}'..='\u{001F}' | ':' | '@' => false,
            // A backslash escapes a backslash, a colon or an at-sign, and nothing else.
            '\\' => matches!(chars.next(), Some('\\' | ':' | '@')),
            _ => true,
        };
        if !is_permitted {
            return false;
        }
    }

    true
}

/// A field descriptor (JVMS §4.3.2).
pub(crate) fn is_field_descriptor(descriptor: &str) -> bool {
    strip_field_type(descriptor) == Some("")
}

/// A method descriptor (JVMS §4.3.3).
///
/// The parameters are measured without the contribution for `this`: a descriptor alone does not
/// tell whether the method is static.
pub(crate) fn is_method_descriptor(descriptor: &str) -> bool {
    let Some(mut rest) = descriptor.strip_prefix('(') else {
        return false;
    };

    let mut parameters_length = 0;
    loop {
        if let Some(return_descriptor) = rest.strip_prefix(')') {
            return return_descriptor == "V" || is_field_descriptor(return_descriptor);
        }

        parameters_length += match rest.as_bytes().first() {
            Some(b'J' | b'D') => 2,
            _ => 1,
        };
        if parameters_length > MAX_PARAMETERS_LENGTH {
            return false;
        }

        match strip_field_type(rest) {
            Some(after_parameter) => rest = after_parameter,
            None => return false,
        }
    }
}

/// Takes one `FieldType` (JVMS §4.3.2) off the front of a string and returns what follows it.
fn strip_field_type(descriptor: &str) -> Option<&str> {
    let component = descriptor.trim_start_matches('[');
    if descriptor.len() - component.len() > MAX_DIMENSIONS {
        return None;
    }

    let mut chars = component.chars();
    match chars.next()? {
        'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' => Some(chars.as_str()),
        'L' => {
            let (class_name, rest) = chars.as_str().split_once(';')?;
            is_binary_name(class_name).then_some(rest)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unqualified_name() {
        assert!(is_unqualified_name("someText"));
        assert!(is_unqualified_name("lambda$main$0"));
        // Only fields may have angle brackets in their names
        assert!(is_unqualified_name("<init>"));
        assert!(is_unqualified_name("a<b>"));
        assert!(is_unqualified_name("ї"));

        assert!(!is_unqualified_name(""));
        assert!(!is_unqualified_name("a.b"));
        assert!(!is_unqualified_name("a;"));
        assert!(!is_unqualified_name("[a"));
        assert!(!is_unqualified_name("a/b"));
    }

    #[test]
    fn test_method_name() {
        assert!(is_method_name("run"));
        assert!(is_method_name("lambda$main$0"));
        assert!(is_method_name("<init>"));
        assert!(is_method_name("<clinit>"));

        assert!(!is_method_name(""));
        assert!(!is_method_name("<run>"));
        assert!(!is_method_name("a<b"));
        assert!(!is_method_name("a>b"));
        assert!(!is_method_name("a/b"));
    }

    #[test]
    fn test_binary_name() {
        assert!(is_binary_name("Trivial"));
        assert!(is_binary_name("java/lang/Object"));
        assert!(is_binary_name("Trivial$1LocalCls"));
        assert!(is_binary_name("module-info"));

        assert!(!is_binary_name(""));
        assert!(!is_binary_name("java.lang.Object"));
        assert!(!is_binary_name("/java/lang/Object"));
        assert!(!is_binary_name("java/lang/Object/"));
        assert!(!is_binary_name("java//Object"));
        assert!(!is_binary_name("Ljava/lang/Object;"));
    }

    #[test]
    fn test_class_info_name() {
        assert!(is_class_info_name("java/lang/Thread"));
        assert!(is_class_info_name("[Ljava/lang/Thread;"));
        assert!(is_class_info_name("[[I"));
        assert!(is_class_info_name(&format!("{}I", "[".repeat(255))));

        assert!(!is_class_info_name("I["));
        assert!(!is_class_info_name("["));
        assert!(!is_class_info_name("[java/lang/Thread"));
        assert!(!is_class_info_name(&format!("{}I", "[".repeat(256))));
    }

    #[test]
    fn test_module_name() {
        assert!(is_module_name("java.base"));
        assert!(is_module_name("a\\\\b"));
        assert!(is_module_name("a\\:b"));
        assert!(is_module_name("a\\@b"));

        assert!(!is_module_name("a\u{001F}b"));
        assert!(!is_module_name("a:b"));
        assert!(!is_module_name("a@b"));
        assert!(!is_module_name("a\\b"));
        assert!(!is_module_name("a\\"));
    }

    #[test]
    fn test_field_descriptor() {
        for descriptor in ["B", "C", "D", "F", "I", "J", "S", "Z"] {
            assert!(is_field_descriptor(descriptor), "{descriptor}");
        }
        assert!(is_field_descriptor("Ljava/lang/String;"));
        assert!(is_field_descriptor("[[[D"));
        assert!(is_field_descriptor("[Ljava/lang/Object;"));

        assert!(!is_field_descriptor(""));
        assert!(!is_field_descriptor("V"));
        assert!(!is_field_descriptor("II"));
        assert!(!is_field_descriptor("Ljava/lang/String"));
        assert!(!is_field_descriptor("L;"));
        assert!(!is_field_descriptor("Ljava.lang.String;"));
        assert!(!is_field_descriptor("Ljava/lang/String;I"));
        assert!(!is_field_descriptor("()V"));
    }

    #[test]
    fn test_method_descriptor() {
        assert!(is_method_descriptor("()V"));
        assert!(is_method_descriptor("(Ljava/lang/String;)V"));
        assert!(is_method_descriptor(
            "(IDLjava/lang/Thread;)Ljava/lang/Object;"
        ));
        assert!(is_method_descriptor("([[J[Ljava/lang/String;)[I"));

        assert!(!is_method_descriptor(""));
        assert!(!is_method_descriptor("V"));
        assert!(!is_method_descriptor("()"));
        assert!(!is_method_descriptor("(V)V"));
        assert!(!is_method_descriptor("(I"));
        assert!(!is_method_descriptor("(I)VV"));
        assert!(!is_method_descriptor("(Ljava/lang/String)V"));
        assert!(!is_method_descriptor("I()V"));
    }

    #[test]
    fn test_method_descriptor_parameters_length() {
        // `long` and `double` contribute two units each, anything else one
        assert!(is_method_descriptor(&format!("({})V", "I".repeat(255))));
        assert!(!is_method_descriptor(&format!("({})V", "I".repeat(256))));
        assert!(is_method_descriptor(&format!("({}I)V", "J".repeat(127))));
        assert!(!is_method_descriptor(&format!("({})V", "D".repeat(128))));
        // An array of `long` is a reference
        assert!(is_method_descriptor(&format!("({})V", "[J".repeat(255))));
    }
}
