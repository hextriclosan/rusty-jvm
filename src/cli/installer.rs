use crate::cli::utils::{current_version_data_dir, data_dir, user_confirmed, NAME, VERSION};
use reqwest;
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
        "https://github.com/hextriclosan/rusty-jvm/releases/download/rusty_jvm-v{VERSION}/rusty_jvm-v{VERSION}-lib.zip"
    );
    println!("Downloading {target_url} to memory...");

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(300)) // 5 minutes timeout (for slow docker containers)
        .build()?;

    let response = client.get(target_url).send()?;
    let status_code = response.status();
    println!("Download finished with status: {status_code}");
    if !status_code.is_success() {
        return Err(format!("Failed to download libraries: {}", status_code).into());
    }
    Ok(response.bytes()?.to_vec())
}

fn extract(raw: &[u8], lib_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("Extracting standard libraries to {} ...", lib_dir.display());
    let mut archive = zip::ZipArchive::new(Cursor::new(raw))?;
    let out_dir = Path::new(&lib_dir);
    archive.extract(Path::new(out_dir))?;
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
