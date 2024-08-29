use std::env;
use std::process;
use vm::vm::VM;

fn print_usage() -> Result<(), String> {
    let current_exe = env::current_exe()
        .map_err(|err| format!("Failed to get the current executable path: {}", err))?;
    let file_name = current_exe
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "Failed to determine the executable name".to_string())?;
    println!("Usage: {} <class file>", file_name);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        if let Err(err) = print_usage() {
            eprintln!("{}", err);
            process::exit(1);
        }
        process::exit(1);
    }

    let filename = &args[1];

    let vm = match VM::new(filename, "std") {
        Ok(vm) => vm,
        Err(err) => {
            eprintln!("Failed to create VM: {}", err);
            process::exit(1);
        }
    };

    let result = match vm.run() {
        Ok(output) => output,
        Err(err) => {
            eprintln!("VM execution failed: {}", err);
            process::exit(1);
        }
    };

    println!(
        "result={}",
        result.map_or_else(|| "<empty>".to_string(), |v| v.to_string())
    );
}
