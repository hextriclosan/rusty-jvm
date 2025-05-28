use directories_next::ProjectDirs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};

const LIB_DIR_ENV: &'static str = "RUSTY_LIB_DIR";
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub fn resolve_std_dir() -> Result<Option<String>, Box<dyn std::error::Error>> {
    if let Ok(dir) = std::env::var(LIB_DIR_ENV) {
        Ok(Some(dir))
    } else {
        resolve_data_dir()
    }
}

fn resolve_data_dir() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let lib_dir = current_version_data_dir()?;
    dir_non_empty(&lib_dir).and_then(|non_empty| {
        if non_empty {
            Ok(Some(lib_dir.display().to_string()))
        } else {
            Ok(None)
        }
    })
}

pub fn current_version_data_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let data_dir = data_dir()?;
    let lib_dir = data_dir.join(format!("v{}", VERSION)).join("lib");
    Ok(lib_dir)
}

pub fn data_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let project_dirs = ProjectDirs::from("com.github.hextriclosan", "", NAME)
        .ok_or_else(|| "Failed to determine application data directory")?;

    Ok(project_dirs.data_dir().to_path_buf())
}

fn dir_non_empty<P: AsRef<Path>>(path: P) -> Result<bool, Box<dyn std::error::Error>> {
    let non_empty = match fs::read_dir(path) {
        Ok(mut entries) => entries.next().is_some(), // Check if there is at least one entry
        Err(e) if matches!(e.kind(), io::ErrorKind::NotFound) => false,
        Err(e) => return Err(Box::new(e)),
    };
    Ok(non_empty)
}

pub fn user_confirmed() -> io::Result<bool> {
    print!("Continue? [y/N]: ");
    io::stdout().flush()?;

    let mut input = String::new();
    let confirmed = if io::stdin().read_line(&mut input).is_ok() {
        matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
    } else {
        false
    };

    Ok(confirmed)
}
