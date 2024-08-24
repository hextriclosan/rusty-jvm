use vm::vm::VM;

fn main() {
    match VM::new("tests/test_data/Sub.class") {
        Ok(vm) => {
            println!("{vm:#?}");
            vm.run().expect("error running vm");
        }
        Err(err) => {
            eprintln!("Error: {err}");
        }
    }

}
