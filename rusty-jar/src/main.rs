use clap::{Args, Parser, Subcommand};
use rusty_jar::jarfile::JarFile;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// A tool to inspect and extract from JAR files.
#[derive(Parser, Debug)]
#[command(name = "rusty-jar", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Extract a specific resource from a JAR file
    Extract(ExtractArgs),
    /// List all resources in a JAR file
    List(ListArgs),
}

#[derive(Args, Debug)]
struct ExtractArgs {
    /// The full path of the resource to extract from the JAR
    /// (e.g., "META-INF/MANIFEST.MF")
    #[arg(long, short = 'r', value_name = "RESOURCE_PATH")]
    resource_path: String,

    /// Path to the JAR file
    #[arg(value_name = "JAR_FILE_PATH", required = true)]
    jar_file_path: PathBuf,
}

#[derive(Args, Debug)]
struct ListArgs {
    /// Path to the JAR file
    #[arg(value_name = "JAR_FILE_PATH", required = true)]
    jar_file_path: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Extract(args) => {
            extract_resource(&args.jar_file_path, &args.resource_path).unwrap_or_else(|err| {
                eprintln!("Error extracting resource: {}", err);
                std::process::exit(1);
            });
        }
        Commands::List(args) => {
            list_resources(&args.jar_file_path).unwrap_or_else(|err| {
                eprintln!("Error getting list of resources: {}", err);
                std::process::exit(1);
            });
        }
    }
}

fn extract_resource(jar_file_path: &PathBuf, resource_path: &str) -> Result<(), Box<dyn Error>> {
    let mut jar_file = JarFile::open(&jar_file_path)?;
    let resource_data = jar_file.content_by_name(resource_path)?;

    let path = Path::new(resource_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    File::create(path)?.write_all(&resource_data)?;

    Ok(())
}

fn list_resources(jar_file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let jar_file = JarFile::open(&jar_file_path)?;
    for file_name in jar_file.file_names() {
        println!("{}", file_name);
    }

    Ok(())
}
