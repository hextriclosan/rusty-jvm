mod utils;

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Once;

fn assert_native_matches_reference(entry: &str) {
    static BUILD: Once = Once::new();
    BUILD.call_once(|| {
        let status = Command::new("cargo")
            .args(["build", "--locked", "-p", "jni_test_lib"])
            .status()
            .expect("failed to build JNI test library");
        assert!(status.success());
    });
    let library = format!("-Djava.library.path={}", env!("JNI_TEST_LIB_PATH"));
    let reference =
        utils::run_reference_jvm(&[&library, "--enable-native-access=ALL-UNNAMED"], entry);
    assert_eq!(reference.exit_code, 0, "{}", reference.stderr);
    let output = Command::new(env!("CARGO_BIN_EXE_rusty-jvm"))
        .current_dir(utils::TEST_PATH.as_path())
        .args([&library, entry])
        .stdin(Stdio::null())
        .output()
        .expect("failed to run rusty-jvm");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout)
            .unwrap()
            .replace("\r\n", "\n"),
        reference.stdout
    );
}

#[test]
fn jni_variadic_calls_match_reference_for_all_return_types() {
    assert_native_matches_reference("samples.jni.JniVariadicDemo");
}

#[test]
fn jni_reflected_fields_preserve_ids_and_declaring_classes() {
    assert_native_matches_reference("samples.jni.JniReflectedFieldsDemo");
}

#[test]
fn reflected_field_generics_and_annotations_match_reference() {
    utils::assert_same_as_reference_jvm(
        &[],
        "samples.reflection.fields.FieldMetadataDemo",
        utils::ExecutionResult::Success,
    );
}

#[test]
fn bootstrap_initializes_reference_access_with_utf8_encodings() {
    utils::assert_with_all_args(
        &["-Dnative.encoding=UTF-8", "-Dsun.jnu.encoding=UTF-8"],
        "samples.properties.EncodingPropertiesDemo",
        &[],
        "native.encoding=UTF-8\nstdin.encoding=UTF-8\nstdout.encoding=UTF-8\nstderr.encoding=UTF-8\nencoding-properties-ok\n",
        "",
        utils::ExecutionResult::Success,
        0,
        Default::default(),
    );
}

fn assert_encoding_properties(locale: Option<&str>, piped_input: bool) {
    let java_home = std::env::var_os("JAVA_HOME").expect("JAVA_HOME is required");
    let java =
        Path::new(&java_home)
            .join("bin")
            .join(if cfg!(windows) { "java.exe" } else { "java" });
    let outputs = [java, PathBuf::from(env!("CARGO_BIN_EXE_rusty-jvm"))].map(|binary| {
        let mut command = Command::new(binary);
        command
            .current_dir(utils::TEST_PATH.as_path())
            .arg("samples.properties.EncodingPropertiesDemo")
            .env_remove("JAVA_TOOL_OPTIONS")
            .env_remove("_JAVA_OPTIONS")
            .env_remove("JDK_JAVA_OPTIONS")
            .stdin(if piped_input {
                Stdio::piped()
            } else {
                Stdio::null()
            });
        if let Some(locale) = locale {
            command.env("LC_ALL", locale);
        }
        let output = command.output().expect("failed to run encoding fixture");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        output
    });
    assert_eq!(outputs[0].stdout, outputs[1].stdout);
    assert_eq!(outputs[0].stderr, outputs[1].stderr);
}

#[test]
fn encoding_properties_match_reference_with_redirected_streams() {
    assert_encoding_properties(None, false);
}

#[test]
fn encoding_properties_match_reference_with_piped_input() {
    assert_encoding_properties(None, true);
}

#[cfg(unix)]
#[test]
fn encoding_properties_match_reference_with_c_locale() {
    assert_encoding_properties(Some("C"), false);
}
