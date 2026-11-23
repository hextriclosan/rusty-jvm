use std::{env, path::PathBuf};

fn main() {
    let java_home = PathBuf::from(env::var("JAVA_HOME").expect("JAVA_HOME is required"));
    let platform = match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "windows" => "win32",
        "macos" => "darwin",
        _ => "linux",
    };
    cc::Build::new()
        .file("src/completion_demo.c")
        .include(java_home.join("include"))
        .include(java_home.join("include").join(platform))
        .compile("jni_completion_tests");
    println!("cargo:rerun-if-changed=src/completion_demo.c");
    println!("cargo:rerun-if-env-changed=JAVA_HOME");
}
