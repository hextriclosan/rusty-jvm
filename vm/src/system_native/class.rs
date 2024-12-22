use crate::error::Error;
use crate::execution_engine::string_pool_helper::StringPoolHelper;
use crate::heap::heap::with_heap_write_lock;
use crate::method_area::method_area::with_method_area;
use crate::method_area::primitives_helper::{PRIMITIVE_CODE_BY_TYPE, PRIMITIVE_TYPE_BY_CODE};
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
const ANNOTATION: u16 = 0x00002000;
const ENUM: u16 = 0x00004000;

const MODIFIERS: u16 = PUBLIC
    | PROTECTED
    | PRIVATE
    | ABSTRACT
    | STATIC
    | FINAL
    | STRICT
    | INTERFACE
    | ENUM
    | ANNOTATION;

pub(crate) fn get_modifiers_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let modifiers = get_modifiers(args[0])?;

    Ok(vec![modifiers])
}
fn get_modifiers(reference: i32) -> crate::error::Result<i32> {
    let modifiers = with_method_area(|method_area| {
        let name = method_area.get_from_reflection_table(reference)?;
        let rc = method_area.get(&name)?;
        let access_flags = rc.access_flags();

        Ok::<u16, Error>(access_flags & MODIFIERS)
    })? as i32;

    Ok(modifiers)
}

pub(crate) fn get_superclass_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let current_clazz_ref = args[0];
    let super_clazz_ref = get_superclass(current_clazz_ref)?;

    Ok(vec![super_clazz_ref])
}
fn get_superclass(clazz_ref: i32) -> crate::error::Result<i32> {
    let current_clazz_ref = with_method_area(|method_area| {
        let name = method_area.get_from_reflection_table(clazz_ref)?;
        let rc = method_area.get(&name)?;
        let parent = if !rc.is_interface() {
            rc.parent().clone()
        } else {
            None
        };

        let current_clazz_ref =
            parent.map_or(Ok(0), |parent| method_area.load_reflection_class(&parent));
        current_clazz_ref
    })?;

    Ok(current_clazz_ref)
}

pub(crate) fn get_primitive_class_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let string_ref = args[0];
    let class_ref = get_primitive_class(string_ref)?;
    Ok(vec![class_ref])
}

fn get_primitive_class(string_ref: i32) -> crate::error::Result<i32> {
    let string_content = get_utf8_string_by_ref(string_ref)?;
    let mapped_class_name = map_primitive_class(&string_content)?;

    let reflection_ref =
        with_method_area(|method_area| method_area.load_reflection_class(mapped_class_name))?;

    Ok(reflection_ref)
}
fn map_primitive_class(primitive_type: &str) -> crate::error::Result<&str> {
    let matched = PRIMITIVE_CODE_BY_TYPE.get(primitive_type).ok_or_else(|| {
        Error::new_execution(&format!("Unsupported primitive type: {primitive_type}"))
    })?;

    Ok(matched)
}

pub(crate) fn is_primitive_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let primitive = is_primitive(args[0])?;

    Ok(vec![primitive as i32])
}
fn is_primitive(reference: i32) -> crate::error::Result<bool> {
    with_method_area(|method_area| {
        let name = method_area.get_from_reflection_table(reference)?;
        Ok(PRIMITIVE_TYPE_BY_CODE.contains_key(&name.as_str()))
    })
}

pub(crate) fn is_array_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let array = is_array(args[0])?;

    Ok(vec![array as i32])
}
fn is_array(reference: i32) -> crate::error::Result<bool> {
    with_method_area(|method_area| {
        let name = method_area.get_from_reflection_table(reference)?;
        Ok(name.starts_with('['))
    })
}

pub(crate) fn is_interface_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let interface = is_interface(args[0])?;

    Ok(vec![interface as i32])
}
fn is_interface(reference: i32) -> crate::error::Result<bool> {
    Ok((get_modifiers(reference)? as u16 & INTERFACE) != 0)
}

pub(crate) fn class_init_class_name_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let class_ref = args[0];
    let string_ref = init_class_name(class_ref)?;

    Ok(vec![string_ref])
}
fn init_class_name(class_ref: i32) -> crate::error::Result<i32> {
    let class_name = with_method_area(|method_area| {
        let class_name = method_area.get_from_reflection_table(class_ref)?;
        let result = method_area.get(&class_name)?;
        let string = result.external_name().to_string();
        Ok::<String, Error>(string)
    })?;

    let string_ref = StringPoolHelper::get_string(class_name)?;
    Ok(string_ref)
}

pub(crate) fn for_name0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let name_ref = args[0];
    let initialize = args[1] != 0;
    let loader_ref = args[2];
    let caller_ref = args[3];

    let class_ref = for_name0(name_ref, initialize, loader_ref, caller_ref)?;
    Ok(vec![class_ref])
}
fn for_name0(
    name_ref: i32,
    _initialize: bool,
    _loader_ref: i32,
    _caller_ref: i32,
) -> crate::error::Result<i32> {
    let name = get_utf8_string_by_ref(name_ref)?;
    let internal_name = name.replace('.', "/");
    let reflection_ref =
        with_method_area(|method_area| method_area.load_reflection_class(&internal_name))?;

    Ok(reflection_ref)
}
pub(crate) fn get_interfaces0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let class_ref = args[0];
    let interfaces_ref = get_interfaces0(class_ref)?;
    Ok(vec![interfaces_ref])
}
fn get_interfaces0(class_ref: i32) -> crate::error::Result<i32> {
    let interface_refs = with_method_area(|method_area| {
        let class_name = method_area.get_from_reflection_table(class_ref)?;
        let jc = method_area.get(&class_name)?;
        let interfaces = jc.interfaces();

        let interface_refs = interfaces
            .iter()
            .map(|interface| method_area.load_reflection_class(interface))
            .collect::<crate::error::Result<Vec<i32>>>();
        interface_refs
    })?;

    let result_ref = with_heap_write_lock(|heap| {
        heap.create_array_with_values("[Ljava/lang/Class;", &interface_refs)
    });
    Ok(result_ref)
}
