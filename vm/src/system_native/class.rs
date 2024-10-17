use crate::error::Error;
use crate::method_area::method_area::with_method_area;
use crate::system_native::string::get_utf8_string_by_ref;
/*
 * Access modifier flag constants from tables 4.1, 4.4, 4.5, and 4.7 of
 * The Java Virtual Machine Specification
 */
const PUBLIC: u16 = 0x00000001;
const PRIVATE: u16 = 0x00000002;
const PROTECTED: u16 = 0x00000004;
const STATIC: u16 = 0x00000008;
const FINAL: u16 = 0x00000010;
const _SYNCHRONIZED: u16 = 0x00000020;
const _VOLATILE: u16 = 0x00000040;
const _TRANSIENT: u16 = 0x00000080;
const _NATIVE: u16 = 0x00000100;
const INTERFACE: u16 = 0x00000200;
const ABSTRACT: u16 = 0x00000400;
const STRICT: u16 = 0x00000800;

const MODIFIERS: u16 =
    PUBLIC | PROTECTED | PRIVATE | ABSTRACT | STATIC | FINAL | STRICT | INTERFACE;

pub(crate) fn get_modifiers_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let modifiers = get_modifiers(args[0]);

    Ok(vec![modifiers])
}
fn get_modifiers(reference: i32) -> i32 {
    with_method_area(|method_area| {
        let rc = method_area
            .get_from_reflection_table(reference)
            .expect("error getting method area");
        let access_flags = rc.access_flags();

        access_flags & MODIFIERS
    }) as i32
}

pub(crate) fn get_primitive_class_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let string_ref = args[0];
    let class_ref = get_primitive_class(string_ref)?;
    Ok(vec![class_ref])
}

fn get_primitive_class(string_ref: i32) -> crate::error::Result<i32> {
    let string_content = get_utf8_string_by_ref(string_ref)?;
    let mapped_class_name = map_primitive_class(&string_content)?;

    let class = with_method_area(|method_area| method_area.get(mapped_class_name))?;

    let reflection_ref = class.reflection_ref();

    Ok(reflection_ref)
}
fn map_primitive_class(primitive_type: &str) -> crate::error::Result<&str> {
    let matched = match primitive_type {
        "boolean" => "Z",
        "byte" => "B",
        "char" => "C",
        "short" => "S",
        "int" => "I",
        "long" => "J",
        "float" => "F",
        "double" => "D",
        "void" => "V",
        _ => {
            return Err(Error::new_execution(&format!(
                "Unsupported primitive type: {primitive_type}"
            )))
        }
    };

    Ok(matched)
}
