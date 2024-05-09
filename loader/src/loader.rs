
use std::{fs, io};
use model::class_file::ClassFile;
use crate::class_file_parser::Parser;


pub fn load(file_name: &str) -> Result<ClassFile, io::Error> {
    let data = fs::read(file_name)?;

    let mut parser = Parser::new();
    let class_file = parser.parse(data.as_slice());

    class_file
}
