use fs_extra::dir::{copy, CopyOptions};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=tests/test_data");
    let dest_dir = PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap_or(String::from("target")))
        .join("java_classes_for_tests");
    fs::create_dir_all(&dest_dir)?;

    copt_generated_files(&dest_dir)?;

    compile(&dest_dir)
}

fn copt_generated_files(dest_dir: &Path) -> anyhow::Result<()> {
    let src_dir = PathBuf::from("tests/test_data/generated");
    let options = CopyOptions::new().content_only(true).overwrite(true);
    copy(src_dir, dest_dir, &options)?;

    Ok(())
}

fn compile(dest_dir: &Path) -> anyhow::Result<()> {
    let java_home = env::var("JAVA_HOME").map_err(|_| anyhow::anyhow!("JAVA_HOME is not set"))?;
    let mut javac = PathBuf::from(&java_home).join("bin").join("javac");
    if cfg!(windows) {
        javac.set_extension("exe");
    }
    if !javac.exists() {
        anyhow::bail!("javac not found at {}", javac.display());
    }

    let output = Command::new(&javac)
        .arg("-version")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()?;
    let version_str = String::from_utf8_lossy(&output.stdout);
    if !version_str.contains("23") {
        anyhow::bail!("javac version is not 23: {}", version_str.trim());
    }
    println!("javac version OK: {}", version_str.trim());

    let src_dir = PathBuf::from("tests/test_data");
    let excludes = [
        "StringConcatInline.java",
        "BootstrapMethodInvokerExample.java",
        "ConstantPoolExample.java",
        "StringConcatHelperExample.java",
        "ReflectionAreNestMatesExample.java",
        "ReflectionGetCallerClassExample.java",
        "UnsafeGetLongUnalignedExample.java",
        "UnsafeUsage.java",
        "UnsafeObjectFieldOffset.java",
        "UnsafePutReferenceVolatileExample.java",
    ];

    let mut normal_files = Vec::new();
    for entry in fs::read_dir(&src_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |e| e == "java") {
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            if !excludes.contains(&filename.as_str()) {
                normal_files.push(path);
            }
        }
    }

    if !normal_files.is_empty() {
        let mut args = vec!["-d", dest_dir.to_str().unwrap()];
        args.extend(normal_files.iter().map(|p| p.to_str().unwrap()));
        run(&javac, &args)?;
    }

    let special_cmds: &[(&[&str], &str)] = &[
        (&["-XDstringConcat=inline", "-d"], "StringConcatInline.java"),
        (
            &["--patch-module", "java.base=.", "-d"],
            "BootstrapMethodInvokerExample.java",
        ),
        (
            &[
                "--add-exports",
                "java.base/jdk.internal.reflect=ALL-UNNAMED",
                "--patch-module",
                "java.base=.",
                "-d",
            ],
            "ConstantPoolExample.java",
        ),
        (
            &["--patch-module", "java.base=.", "-d"],
            "StringConcatHelperExample.java",
        ),
        (
            &[
                "--add-exports",
                "java.base/jdk.internal.reflect=ALL-UNNAMED",
                "-d",
            ],
            "ReflectionAreNestMatesExample.java",
        ),
        (
            &[
                "--add-exports",
                "java.base/jdk.internal.reflect=ALL-UNNAMED",
                "-d",
            ],
            "ReflectionGetCallerClassExample.java",
        ),
        (
            &[
                "--add-exports",
                "java.base/jdk.internal.misc=ALL-UNNAMED",
                "-d",
            ],
            "UnsafeGetLongUnalignedExample.java",
        ),
        (
            &[
                "--add-exports",
                "java.base/jdk.internal.misc=ALL-UNNAMED",
                "-d",
            ],
            "UnsafeUsage.java",
        ),
        (
            &[
                "--add-exports",
                "java.base/jdk.internal.misc=ALL-UNNAMED",
                "-d",
            ],
            "UnsafeObjectFieldOffset.java",
        ),
        (
            &[
                "--add-exports",
                "java.base/jdk.internal.misc=ALL-UNNAMED",
                "-d",
            ],
            "UnsafePutReferenceVolatileExample.java",
        ),
    ];

    for (args_prefix, file) in special_cmds {
        let mut args: Vec<String> = args_prefix.iter().map(|s| s.to_string()).collect();
        args.push(dest_dir.to_string_lossy().into_owned());
        args.push(src_dir.join(file).to_string_lossy().into_owned());
        let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        run(&javac, &args_ref)?;
    }

    Ok(())
}

fn run(javac: &PathBuf, args: &[&str]) -> anyhow::Result<()> {
    println!("Running: {} {}", javac.display(), args.join(" "));
    let status = Command::new(javac)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    if !status.success() {
        anyhow::bail!("javac failed with args: {:?}", args);
    }
    Ok(())
}
