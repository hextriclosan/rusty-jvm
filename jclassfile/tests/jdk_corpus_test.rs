//! Parses every class file of a directory tree, to catch a format check that rejects what
//! compilers actually emit.
//!
//! Ignored unless asked for, as it needs class files that are not part of the repository, e.g.
//! the classes of a JDK:
//!
//! ```text
//! jimage extract --dir /tmp/jdk-classes $JAVA_HOME/lib/modules
//! JCLASSFILE_CORPUS_DIR=/tmp/jdk-classes cargo test -p jclassfile --test jdk_corpus_test -- --ignored --nocapture
//! ```

use jclassfile::class_file::parse;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn collect_class_files(dir: &Path, class_files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_class_files(&path, class_files);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "class")
        {
            class_files.push(path);
        }
    }
}

#[test]
#[ignore = "needs JCLASSFILE_CORPUS_DIR to point to a directory of class files"]
fn should_not_fail_format_checking_of_corpus() {
    let corpus_dir = env::var("JCLASSFILE_CORPUS_DIR").expect("JCLASSFILE_CORPUS_DIR is not set");
    let mut class_files = Vec::new();
    collect_class_files(Path::new(&corpus_dir), &mut class_files);
    assert!(!class_files.is_empty(), "No class files in {corpus_dir}");

    let mut failed_format_checking = Vec::new();
    // A class file with an attribute the parser does not know is not parsed either, but that is
    // not a format check: the `module-info.class` files of a JDK carry `ModuleTarget`, for one.
    let mut not_parsed_otherwise = Vec::new();
    for path in &class_files {
        if let Err(error) = parse(&fs::read(path).unwrap()) {
            let failure = format!("{}: {error}", path.display());
            if error.to_string().starts_with("Format Error: ") {
                failed_format_checking.push(failure);
            } else {
                not_parsed_otherwise.push(failure);
            }
        }
    }

    println!("{}", not_parsed_otherwise.join("\n"));
    println!(
        "{} class file(s): {} failed format checking, {} not parsed for another reason",
        class_files.len(),
        failed_format_checking.len(),
        not_parsed_otherwise.len()
    );
    assert!(
        failed_format_checking.is_empty(),
        "{}",
        failed_format_checking.join("\n")
    );
}
