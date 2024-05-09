use loader::loader::load;

fn main() {
    let class_file = load("test_data/Trivial.class");

    match class_file {
        Ok(file) => println!("{:#?}", file),
        Err(err) => eprintln!("Error loading file: {}", err)
    }
}
