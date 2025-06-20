use clap::{Args, Parser, Subcommand};
use jimage_rs::jimage::JImage;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// A fictional tool to inspect and extract from jimage files.
#[derive(Parser, Debug)]
#[command(name = "jimage-rs", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Extract a specific resource from a jimage file
    Extract(ExtractArgs),
}

#[derive(Args, Debug)]
struct ExtractArgs {
    /// The full path of the resource to extract from the jimage
    /// (e.g., "/java.base/java/lang/String.class")
    #[arg(long, short = 'f', value_name = "RESOURCE_PATH")]
    file: String,

    /// Path to the jimage file (typically `<JDK_HOME>/lib/modules`)
    #[arg(value_name = "JIMAGE_PATH", required = true)]
    jimage_path: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Extract(args) => {
            extract_resource(&args.jimage_path, &args.file).unwrap_or_else(|err| {
                eprintln!("Error extracting resource: {}", err);
                std::process::exit(1);
            });
        }
    }
}

fn extract_resource(jimage_path: &PathBuf, resource_name: &str) -> Result<(), Box<dyn Error>> {
    let image = JImage::new(&jimage_path)?;
    let resource_data = image
        .find_resource(resource_name)?
        .ok_or_else(|| format!("Resource '{}' not found in the jimage file", resource_name))?;

    let output_path = PathBuf::from(resource_name.strip_prefix('/').unwrap_or(resource_name));
    save_resource_to_file(&resource_data, &output_path)?;

    Ok(())
}

fn save_resource_to_file(resource: &[u8], output_path: &Path) -> std::io::Result<()> {
    if let Some(parent_dir) = output_path.parent() {
        std::fs::create_dir_all(parent_dir)?;
    }

    let mut file = File::create(output_path)?;
    file.write_all(resource)?;
    Ok(())
}
