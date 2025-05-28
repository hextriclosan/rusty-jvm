use crate::cli::utils::{current_version_data_dir, data_dir, user_confirmed, NAME, VERSION};
use reqwest;
use std::fs::File;
use std::io::Cursor;
use std::path::{Path, PathBuf};

pub fn do_install(yes: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("About to install standard libraries for {NAME}-{VERSION}");

    let lib_dir = current_version_data_dir()?;

    println!("Libraries will be installed to: {}", lib_dir.display());

    if !(yes || user_confirmed()?) {
        println!("Installation cancelled by user.");
        return Ok(());
    }

    let raw = download()?;
    extract(&raw, &lib_dir)?;

    println!("Installed successfully!");
    Ok(())
}

pub fn do_purge(yes: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("About to purge all versions of standard libraries");

    let data_dir = data_dir()?;

    println!("Directory {} will be deleted", data_dir.display());

    if !(yes || user_confirmed()?) {
        println!("Purging cancelled by user.");
        return Ok(());
    }

    delete_dir(&data_dir)?;

    println!("Purged successfully!");
    Ok(())
}

fn download() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let target_url = format!(
        "https://github.com/hextriclosan/rusty-jvm/archive/refs/tags/rusty_jvm-v{VERSION}.zip"
    );
    println!("Downloading {target_url} to memory...");

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(300)) // 5 minutes timeout (for slow docker containers)
        .build()?;

    let response = client.get(target_url).send()?;
    println!("Download finished with status: {}", response.status());
    Ok(response.bytes()?.to_vec())
}

fn extract(raw: &[u8], lib_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("Extracting standard libraries to {} ...", lib_dir.display());
    let mut archive = zip::ZipArchive::new(Cursor::new(raw))?;

    let extract_prefix = format!("rusty-jvm-rusty_jvm-v{}/lib/", VERSION);
    let out_dir = Path::new(&lib_dir);
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let name = entry.name();

        if name.starts_with(&extract_prefix) && name.ends_with(".class") {
            let relative_path = &name[extract_prefix.len()..];
            let out_path = out_dir.join(relative_path);

            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let mut out_file = File::create(out_path.as_path())?;
            std::io::copy(&mut entry, &mut out_file)?;
        }
    }

    Ok(())
}

fn delete_dir(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if path.exists() {
        if path.is_dir() {
            std::fs::remove_dir_all(path)?;
        }
    }
    Ok(())
}
