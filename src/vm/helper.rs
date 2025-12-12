use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::method_area::primitives_helper::PRIMITIVE_TYPE_BY_CODE;
use crate::vm::stack::stack_value::StackValue;
use jdescriptor::TypeDescriptor;
use std::sync::Arc;

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

pub fn clazz_ref(class_name: &str) -> Result<i32> {
    CLASSES.get(class_name)?.mirror_clazz_ref()
}

pub fn klass(clazz_ref: i32) -> Result<Arc<JavaClass>> {
    HEAP.get_mirror_klass_id(clazz_ref)
        .map(|klass_id| CLASSES.get_by_id(klass_id))?
}

pub fn get_length(type_descriptor: &TypeDescriptor) -> Result<i32> {
    match type_descriptor {
        TypeDescriptor::Byte
        | TypeDescriptor::Char
        | TypeDescriptor::Integer
        | TypeDescriptor::Short
        | TypeDescriptor::Boolean
        | TypeDescriptor::Float
        | TypeDescriptor::Array(_, _)
        | TypeDescriptor::Object(_) => Ok(1),
        TypeDescriptor::Long | TypeDescriptor::Double => Ok(2),
        TypeDescriptor::Void => Err(Error::new_execution("Void type doesn't have a length")),
    }
}

pub fn default_value(type_descriptor: &TypeDescriptor) -> Result<Vec<i32>> {
    match type_descriptor {
        TypeDescriptor::Byte
        | TypeDescriptor::Char
        | TypeDescriptor::Integer
        | TypeDescriptor::Short
        | TypeDescriptor::Boolean => Ok(vec![0]),
        TypeDescriptor::Float => Ok(0.0f32.to_vec()),
        TypeDescriptor::Long => Ok(vec![0, 0]),
        TypeDescriptor::Double => Ok(0.0f64.to_vec()),
        TypeDescriptor::Array(_, _) => Ok(vec![0]),
        TypeDescriptor::Object(_) => Ok(vec![0]),
        TypeDescriptor::Void => Err(Error::new_execution("Void type doesn't have a value")),
    }
}

pub fn argument_length(args: &[TypeDescriptor]) -> Result<i32> {
    args.iter().map(get_length).sum()
}

pub fn strip_nest_host(class_name: &str) -> Option<&str> {
    class_name.find('$').map(|index| &class_name[..index])
}

pub fn create_array_of_strings(props: &[String]) -> Result<i32> {
    let class_of_array = "java/lang/String";
    let class_of_array = format!("[L{class_of_array};");
    let length = props.len() as i32;
    let array_ref = HEAP.create_array(&class_of_array, length);

    for (index, prop) in props.iter().enumerate() {
        let string_ref = StringPoolHelper::get_string(prop)?;
        HEAP.set_array_value(array_ref, index as i32, vec![string_ref])?
    }

    Ok(array_ref)
}

#[cfg(unix)]
pub fn get_handle(fd_ref: i32) -> Result<i32> {
    let raw = HEAP.get_object_field_value(fd_ref, "java/io/FileDescriptor", "fd")?;
    Ok(raw[0])
}
#[cfg(windows)]
pub fn get_handle(fd_ref: i32) -> Result<i64> {
    let raw = HEAP.get_object_field_value(fd_ref, "java/io/FileDescriptor", "handle")?;
    Ok(vec_to_i64(&raw))
}

pub fn decorate(type_name: String) -> String {
    if PRIMITIVE_TYPE_BY_CODE.contains_key(type_name.as_str()) // primitive type B, C, D, F, I, J, S, Z, V
        || (type_name.starts_with('[') && PRIMITIVE_TYPE_BY_CODE.contains_key(&type_name[1..])) // array of primitive types [B, [C, [D, [F, [I, [J, [S, [Z, [V
        || ((type_name.starts_with('L') || type_name.starts_with('[')) && type_name.ends_with(';'))
    // already decorated type Ljava/lang/String; or [Ljava/lang/String;
    {
        type_name
    } else {
        format!("L{};", type_name)
    }
}

pub fn undecorate(type_name: &str) -> &str {
    if type_name.starts_with('L') && type_name.ends_with(';') {
        &type_name[1..type_name.len() - 1]
    } else {
        type_name
    }
}
