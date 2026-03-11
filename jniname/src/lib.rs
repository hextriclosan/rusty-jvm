/*!
# `jniname` - Java method signature to C-style JNI function name converter

`jniname` is a Rust library that provides a simple and efficient way to convert Java method signatures to C-style JNI method names.

*/

use jdescriptor::MethodDescriptor;

/// Represents the possible errors that can occur in the application.
#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInput(String),
}

/// Generates the C-style JNI function name for a given Java class, method and descriptor.
///
/// The `class` parameter must be in the internal JVM form (slash-separated), e.g.
/// `"com/example/Test"`. Dot-separated source form (e.g. `"com.example.Test"`) is not
/// accepted and will produce incorrect JNI names.
///
/// # Returns
/// A tuple containing the simple and overloaded names of the function.
/// The simple name is of the form `Java_<encoded_class>_<encoded_method>`, while the overloaded name is of the form `Java_<encoded_class>_<encoded_method>__<encoded_parameters>`.
///
/// # Errors
/// Returns `Error::InvalidInput` if input is invalid.
///
/// # Example
/// ```rust
/// use jniname::{c_name, Error};
/// let result = c_name("Test", "foo", "(I)I");
/// assert_eq!(result, Ok(("Java_Test_foo".to_string(), "Java_Test_foo__I".to_string())));
/// ```
pub fn c_name(class: &str, method: &str, descriptor: &str) -> Result<(String, String), Error> {
    if class.is_empty() {
        return Err(Error::InvalidInput("Class name is empty".to_string()));
    }
    if method.is_empty() {
        return Err(Error::InvalidInput("Method name is empty".to_string()));
    }
    let parsed: MethodDescriptor = descriptor
        .parse()
        .map_err(|e| Error::InvalidInput(format!("Invalid descriptor: {}", e)))?;

    let params = parsed
        .parameter_types()
        .into_iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("");

    let class_part = encode_jni(class);
    let method_part = encode_jni(method);
    let params_part = encode_jni(&params);
    let short_name = format!("Java_{}_{}", class_part, method_part);
    let long_name = format!("Java_{}_{}__{}", class_part, method_part, params_part);

    Ok((short_name, long_name))
}

fn encode_jni(s: &str) -> String {
    let mut out = String::with_capacity(s.len());

    for c in s.chars() {
        match c {
            '/' => out.push('_'),
            '_' => out.push_str("_1"),
            ';' => out.push_str("_2"),
            '[' => out.push_str("_3"),
            'A'..='Z' | 'a'..='z' | '0'..='9' => out.push(c),
            _ => out.push_str(&format!("_0{:04x}", c as u32)),
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_primitives() {
        assert_eq!(
            c_name("Test", "foo", "()V"),
            ok("Java_Test_foo", "Java_Test_foo__")
        );
        assert_eq!(
            c_name("Test", "foo", "(I)V"),
            ok("Java_Test_foo", "Java_Test_foo__I")
        );
        assert_eq!(
            c_name("Test", "foo", "(IJ)V"),
            ok("Java_Test_foo", "Java_Test_foo__IJ")
        );
        assert_eq!(
            c_name("Test", "foo", "(ZBCSFD)V"),
            ok("Java_Test_foo", "Java_Test_foo__ZBCSFD")
        );
    }

    #[test]
    fn test_object_types() {
        assert_eq!(
            c_name("Test", "bar", "(Ljava/lang/String;)V"),
            ok("Java_Test_bar", "Java_Test_bar__Ljava_lang_String_2")
        );
        assert_eq!(
            c_name(
                "com/example/Test",
                "baz",
                "(Ljava/lang/Object;Ljava/lang/String;)V"
            ),
            ok(
                "Java_com_example_Test_baz",
                "Java_com_example_Test_baz__Ljava_lang_Object_2Ljava_lang_String_2"
            )
        );
    }

    #[test]
    fn test_arrays() {
        assert_eq!(
            c_name("Test", "foo", "([I)V"),
            ok("Java_Test_foo", "Java_Test_foo___3I")
        );
        assert_eq!(
            c_name("Test", "foo", "([[I)V"),
            ok("Java_Test_foo", "Java_Test_foo___3_3I")
        );
        assert_eq!(
            c_name("Test", "foo", "([Ljava/lang/String;)V"),
            ok("Java_Test_foo", "Java_Test_foo___3Ljava_lang_String_2")
        );
        assert_eq!(
            c_name("Test", "foo", "([[Ljava/lang/Object;)V"),
            ok("Java_Test_foo", "Java_Test_foo___3_3Ljava_lang_Object_2")
        );
    }

    #[test]
    fn test_tricky_characters() {
        assert_eq!(
            c_name("my/package_name/Class_", "foo", "(Ljava/lang/Object;[I)V"),
            ok(
                "Java_my_package_1name_Class_1_foo",
                "Java_my_package_1name_Class_1_foo__Ljava_lang_Object_2_3I"
            )
        );
        assert_eq!(
            c_name(
                "my/package/with_underscore_",
                "bar",
                "([[Ljava/lang/String;)V"
            ),
            ok(
                "Java_my_package_with_1underscore_1_bar",
                "Java_my_package_with_1underscore_1_bar___3_3Ljava_lang_String_2"
            )
        );
    }

    #[test]
    fn test_long_mixed_signature() {
        assert_eq!(
            c_name(
                "Test",
                "foo",
                "(I[Ljava/lang/String;[[D[[[Ljava/lang/Object;)V"
            ),
            ok(
                "Java_Test_foo",
                "Java_Test_foo__I_3Ljava_lang_String_2_3_3D_3_3_3Ljava_lang_Object_2"
            )
        );
    }

    #[test]
    fn test_nested_edge_cases() {
        assert_eq!(
            c_name("a/b_c/d_", "m_1", "([[Ljava/lang/String;[I[[[J)V"),
            ok(
                "Java_a_b_1c_d_1_m_11",
                "Java_a_b_1c_d_1_m_11___3_3Ljava_lang_String_2_3I_3_3_3J"
            )
        );
    }

    #[test]
    fn special_characters_in_names() {
        assert_eq!(
            c_name("com/example/Outer$Inner", "method", "(I)V"),
            ok(
                "Java_com_example_Outer_00024Inner_method",
                "Java_com_example_Outer_00024Inner_method__I"
            )
        );

        assert_eq!(
            c_name("Test", "method€", "()V"),
            ok("Java_Test_method_020ac", "Java_Test_method_020ac__")
        );
    }

    #[test]
    fn non_valid_inputs() {
        assert_eq!(c_name("", "foo", "()V"), err("Class name is empty"));
        assert_eq!(c_name("Test", "", "()V"), err("Method name is empty"));
        assert_eq!(
            c_name("Test", "foo", ""),
            err("Invalid descriptor: Invalid descriptor: Method descriptor must start with '('.")
        );
        assert_eq!(
            c_name("Test", "foo", "(III"),
            err("Invalid descriptor: Unexpected end of input.")
        );
        assert_eq!(
            c_name("Test", "foo", "(IM)V"),
            err("Invalid descriptor: Invalid descriptor: Unrecognized type descriptor.")
        );
    }

    fn ok(a: &str, b: &str) -> Result<(String, String), Error> {
        Ok((a.to_string(), b.to_string()))
    }

    fn err(msg: &str) -> Result<(String, String), Error> {
        Err(Error::InvalidInput(msg.to_string()))
    }
}
