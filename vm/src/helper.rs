use crate::error::Error;
use crate::method_area::method_area::with_method_area;
use crate::stack::sack_value::StackValue;
use jdescriptor::TypeDescriptor;

pub fn i32toi64(high: i32, low: i32) -> i64 {
    let high_converted = (high as i64) << 32;
    let low_converted = low as u32/*to prevent sign extension*/ as i64;

    high_converted | low_converted
}

pub fn i64_to_vec(value: i64) -> Vec<i32> {
    let low = value as i32;
    let high = (value >> 32) as i32;

    vec![high, low]
}

pub fn vec_to_i64(value: &[i32]) -> i64 {
    match value.len() {
        1 => value[0] as i64,
        2 => i32toi64(value[0], value[1]),
        _ => panic!("Invalid value length: {}", value.len()),
    }
}

pub fn clazz_ref(class_name: &str) -> crate::error::Result<i32> {
    with_method_area(|area| {
        let clazz_ref = area.load_reflection_class(class_name)?;
        Ok::<i32, Error>(clazz_ref)
    })
}

pub fn get_length(type_descriptor: &TypeDescriptor) -> crate::error::Result<i32> {
    match type_descriptor {
        TypeDescriptor::Byte
        | TypeDescriptor::Char
        | TypeDescriptor::Int
        | TypeDescriptor::Short
        | TypeDescriptor::Boolean
        | TypeDescriptor::Float => Ok(1),
        TypeDescriptor::Long | TypeDescriptor::Double => Ok(2),
        TypeDescriptor::Void => Err(Error::new_execution("Void type doesn't have a length")),
        TypeDescriptor::Array(_, _) => Ok(1),
        TypeDescriptor::Object(_) => Ok(1),
    }
}

pub fn default_value(type_descriptor: &TypeDescriptor) -> Vec<i32> {
    match type_descriptor {
        TypeDescriptor::Byte
        | TypeDescriptor::Char
        | TypeDescriptor::Int
        | TypeDescriptor::Short
        | TypeDescriptor::Boolean => vec![0],
        TypeDescriptor::Float => 0.0f32.to_vec(),
        TypeDescriptor::Long => vec![0, 0],
        TypeDescriptor::Double => 0.0f64.to_vec(),
        TypeDescriptor::Array(_, _) => vec![0],
        TypeDescriptor::Object(_) => vec![0],
        TypeDescriptor::Void => panic!("Void type doesn't have a value"),
    }
}
