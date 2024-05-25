use std::{env, fs, io};
use jclass::class_file::ClassFile;
use jclass::class_file_parser::parse;

fn main() {
    let print_usage = || -> Result<(), String> {
        let current_exe = env::current_exe().map_err(|err| format!("Error: {}", err))?;
        let file_name = current_exe
            .as_path()
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or("Error: unable to determine exe name")?;
        println!("Usage: {} <class file>", file_name);

        Ok(())
    };

    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(filename) => {
            let class_file = load(filename);

            match class_file {
                Ok(file) => println!("{:#?}", file),
                Err(err) => {
                    eprintln!("Error loading file: {}", err);
                    std::process::exit(1);
                }
            }
        }
        None => {
            if let Err(err) = print_usage() {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }
    }
}

fn load(file_name: &str) -> Result<ClassFile, io::Error> {
    let data = fs::read(file_name)?;

    let class_file = parse(data.as_slice());

    class_file
}
