use crate::vm::error::Result;
use crate::vm::system_native::properties_provider::properties::path_separator;
use std::fs;
use std::path::{Path, PathBuf};

pub(super) fn wildcards_to_classpath(classpath: &str) -> Result<String> {
    // fast path (same as OpenJDK)
    if !classpath.contains('*') {
        return Ok(classpath.to_string());
    }

    let mut result = Vec::new();

    for entry in classpath.split(path_separator()) {
        if is_wildcard(entry) {
            match expand_wildcard(entry)? {
                Some(mut jars) => result.append(&mut jars),
                None => result.push(entry.to_string()), // fallback (same behavior)
            }
        } else {
            result.push(entry.to_string());
        }
    }

    Ok(result.join(path_separator()))
}

fn is_wildcard(entry: &str) -> bool {
    if !entry.ends_with('*') {
        return false;
    }

    // must be "*" or "dir/*"
    if entry.len() > 1 {
        let bytes = entry.as_bytes();
        let prev = bytes[bytes.len() - 2];
        if prev != b'/' && prev != b'\\' {
            return false;
        }
    }

    // OpenJDK: wildcard must NOT exist as file
    !Path::new(entry).exists()
}

fn expand_wildcard(entry: &str) -> Result<Option<Vec<String>>> {
    // strip trailing '*'
    let dir = if entry == "*" {
        PathBuf::from(".")
    } else {
        PathBuf::from(&entry[..entry.len() - 1])
    };

    let read_dir = match fs::read_dir(&dir) {
        Ok(rd) => rd,
        Err(_) => return Ok(None), // same as C version
    };

    let mut jars = Vec::new();

    for entry in read_dir {
        let entry = entry?;
        let path = entry.path();

        if is_jar(&path) {
            jars.push(path_to_string(&path));
        }
    }

    if jars.is_empty() {
        Ok(None)
    } else {
        Ok(Some(jars))
    }
}

fn is_jar(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => ext.eq_ignore_ascii_case("jar"),
        None => false,
    }
}

fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::path::Path;

    fn join(parts: &[&str]) -> String {
        parts.join(path_separator())
    }

    fn touch(path: &Path) {
        File::create(path).unwrap();
    }

    fn mkdir(path: &Path) {
        fs::create_dir_all(path).unwrap();
    }

    fn tmp() -> tempfile::TempDir {
        tempfile::tempdir().unwrap()
    }

    // -----------------------------
    // BASIC BEHAVIOR
    // -----------------------------

    #[test]
    fn no_wildcards_returns_same() {
        let cp = join(&["a.jar", "b.jar"]);
        let out = wildcards_to_classpath(&cp).unwrap();
        assert_eq!(out, cp);
    }

    #[test]
    fn single_wildcard_expands() {
        let dir = tmp();
        let lib = dir.path().join("lib");
        mkdir(&lib);

        touch(&lib.join("a.jar"));
        touch(&lib.join("b.jar"));

        let cp = format!("{}{}*", lib.display(), std::path::MAIN_SEPARATOR);
        let out = wildcards_to_classpath(&cp).unwrap();

        assert!(out.contains("a.jar"));
        assert!(out.contains("b.jar"));
    }

    #[test]
    fn ignores_non_jar_files() {
        let dir = tmp();
        let lib = dir.path().join("lib");
        mkdir(&lib);

        touch(&lib.join("a.jar"));
        touch(&lib.join("b.txt"));
        touch(&lib.join("c.zip"));

        let cp = format!("{}{}*", lib.display(), std::path::MAIN_SEPARATOR);
        let out = wildcards_to_classpath(&cp).unwrap();

        assert!(out.contains("a.jar"));
        assert!(!out.contains("b.txt"));
        assert!(!out.contains("c.zip"));
    }

    #[test]
    fn case_insensitive_jar() {
        let dir = tmp();
        let lib = dir.path().join("lib");
        mkdir(&lib);

        touch(&lib.join("A.JAR"));

        let cp = format!("{}{}*", lib.display(), std::path::MAIN_SEPARATOR);
        let out = wildcards_to_classpath(&cp).unwrap();

        assert!(out.contains("A.JAR"));
    }

    // -----------------------------
    // MULTIPLE ENTRIES
    // -----------------------------

    #[test]
    fn mixed_entries() {
        let dir = tmp();
        let lib = dir.path().join("lib");
        mkdir(&lib);

        touch(&lib.join("a.jar"));

        let cp = join(&[
            "app.jar",
            &format!("{}{}*", lib.display(), std::path::MAIN_SEPARATOR),
        ]);

        let out = wildcards_to_classpath(&cp).unwrap();

        assert!(out.contains("app.jar"));
        assert!(out.contains("a.jar"));
    }

    #[test]
    fn multiple_wildcards() {
        let dir = tmp();

        let lib1 = dir.path().join("lib1");
        let lib2 = dir.path().join("lib2");

        mkdir(&lib1);
        mkdir(&lib2);

        touch(&lib1.join("a.jar"));
        touch(&lib2.join("b.jar"));

        let cp = join(&[
            &format!("{}{}*", lib1.display(), std::path::MAIN_SEPARATOR),
            &format!("{}{}*", lib2.display(), std::path::MAIN_SEPARATOR),
        ]);

        let out = wildcards_to_classpath(&cp).unwrap();

        assert!(out.contains("a.jar"));
        assert!(out.contains("b.jar"));
    }

    // -----------------------------
    // EDGE CASES (OpenJDK behavior)
    // -----------------------------

    #[test]
    fn wildcard_directory_not_exists() {
        let dir = tmp();
        let missing = dir.path().join("missing");

        let cp = format!("{}{}*", missing.display(), std::path::MAIN_SEPARATOR);
        let out = wildcards_to_classpath(&cp).unwrap();

        // unchanged
        assert_eq!(out, cp);
    }

    #[test]
    fn wildcard_no_jars() {
        let dir = tmp();
        let lib = dir.path().join("lib");
        mkdir(&lib);

        touch(&lib.join("a.txt"));

        let cp = format!("{}{}*", lib.display(), std::path::MAIN_SEPARATOR);
        let out = wildcards_to_classpath(&cp).unwrap();

        // unchanged (important OpenJDK behavior)
        assert_eq!(out, cp);
    }

    #[test]
    fn star_only_current_dir() {
        let dir = tmp();
        let old = std::env::current_dir().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();

        touch(&dir.path().join("a.jar"));

        let out = wildcards_to_classpath("*").unwrap();

        assert!(out.contains("a.jar"));

        std::env::set_current_dir(old).unwrap();
    }

    #[test]
    fn wildcard_not_at_end_is_ignored() {
        let cp = "lib/*/foo.jar";
        let out = wildcards_to_classpath(cp).unwrap();

        // should NOT expand
        assert_eq!(out, cp);
    }

    #[test]
    fn wildcard_without_separator_is_ignored() {
        let cp = "lib*";
        let out = wildcards_to_classpath(cp).unwrap();

        assert_eq!(out, cp);
    }

    #[test]
    #[cfg(not(windows))] // Windows does not support `*` in file names
    fn wildcard_in_filename_is_ignored() {}
    fn existing_path_with_star_is_not_expanded() {
        let dir = tmp();
        let path = dir.path().join("real*file");

        touch(&path);

        let cp = path.to_string_lossy();
        let out = wildcards_to_classpath(&cp).unwrap();

        assert_eq!(out, cp);
    }

    // -----------------------------
    // ORDER + STRUCTURE
    // -----------------------------

    #[test]
    fn preserves_non_wildcard_order() {
        let dir = tmp();
        let lib = dir.path().join("lib");
        mkdir(&lib);

        touch(&lib.join("a.jar"));

        let cp = join(&[
            "x.jar",
            &format!("{}{}*", lib.display(), std::path::MAIN_SEPARATOR),
            "y.jar",
        ]);

        let out = wildcards_to_classpath(&cp).unwrap();

        assert!(out.starts_with("x.jar"));
        assert!(out.ends_with("y.jar"));
    }

    #[test]
    fn empty_classpath() {
        let out = wildcards_to_classpath("").unwrap();
        assert_eq!(out, "");
    }

    #[test]
    fn only_separator() {
        let cp = path_separator();
        let out = wildcards_to_classpath(&cp).unwrap();

        assert_eq!(out, cp);
    }

    // -----------------------------
    // STRESS / ROBUSTNESS
    // -----------------------------

    #[test]
    fn many_jars() {
        let dir = tmp();
        let lib = dir.path().join("lib");
        mkdir(&lib);

        for i in 0..100 {
            touch(&lib.join(format!("{}.jar", i)));
        }

        let cp = format!("{}{}*", lib.display(), std::path::MAIN_SEPARATOR);
        let out = wildcards_to_classpath(&cp).unwrap();

        for i in 0..100 {
            assert!(out.contains(&format!("{}.jar", i)));
        }
    }

    #[test]
    fn unicode_paths() {
        let dir = tmp();
        let lib = dir.path().join("ліб"); // unicode
        mkdir(&lib);

        touch(&lib.join("тест.jar"));

        let cp = format!("{}{}*", lib.display(), std::path::MAIN_SEPARATOR);
        let out = wildcards_to_classpath(&cp).unwrap();

        assert!(out.contains("тест.jar"));
    }
}
