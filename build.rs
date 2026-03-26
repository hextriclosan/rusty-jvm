use fs_extra::dir::{copy, CopyOptions};
use std::fs::{create_dir_all, File};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{env, fs, io};

fn main() -> anyhow::Result<()> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".into());
    let target = env::var("TARGET")?;
    let profile = env::var("PROFILE")?;

    let base = PathBuf::from(manifest_dir).join(target_dir);

    let with_target = base.join(&target).join(&profile);
    let without_target = base.join(&profile);

    let path = if with_target.exists() {
        with_target
    } else {
        without_target
    };
    println!("cargo:rustc-env=JNI_TEST_LIB_PATH={}", path.display());

    println!("cargo:rerun-if-changed=tests/test_data");
    let dest_dir = PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap_or(String::from("target")))
        .join("java_classes_for_tests");
    create_dir_all(&dest_dir)?;

    copy_generated_files(&dest_dir)?;

    compile(&dest_dir)
}

fn copy_generated_files(dest_dir: &Path) -> anyhow::Result<()> {
    let src_dir = PathBuf::from("tests").join("test_data").join("generated");
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
    if !version_str.contains("25") {
        anyhow::bail!("javac version is not 25: {}", version_str.trim());
    }
    println!("javac version OK: {}", version_str.trim());

    let src_dir = PathBuf::from("tests").join("test_data");
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
        "UserPerfCounterExample.java",
        "ClasspathDemo.java",
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

    let (jar_path_long, jar_path_short) = download_jar_to_temp(dest_dir)?;
    println!("cargo:rustc-env=TEST_JAR_PATH={}", jar_path_short);
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
        (
            &[
                "--add-exports",
                "java.base/jdk.internal.perf=ALL-UNNAMED",
                "--add-exports",
                "java.management/sun.management.counter=ALL-UNNAMED",
                "-d",
            ],
            "UserPerfCounterExample.java",
        ),
        (&["-cp", jar_path_long.as_str(), "-d"], "ClasspathDemo.java"),
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

fn download_jar_to_temp(path: &Path) -> anyhow::Result<(String, String)> {
    let short_path = PathBuf::from("lib_jar").join("algorithm.jar");
    let file_path = path.join(&short_path);
    if file_path.exists() {
        return Ok((
            file_path.display().to_string(),
            short_path.display().to_string(),
        ));
    }

    create_dir_all(file_path.parent().unwrap())?;
    let url = "https://repo1.maven.org/maven2/io/github/hextriclosan/algorithm/0.0.5/algorithm-0.0.5.jar";
    let response = ureq::get(url).call()?;
    let mut reader = response.into_body();
    let mut file = File::create(&file_path)?;
    io::copy(&mut reader.as_reader(), &mut file)?;

    Ok((
        file_path.display().to_string(),
        short_path.display().to_string(),
    ))
}
