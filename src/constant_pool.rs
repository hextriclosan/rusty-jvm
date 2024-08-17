use crate::constant_pool::ConstantPool::*;
use crate::extractors::{get_float, get_int, get_string};
use std::io::ErrorKind::InvalidInput;
use crate::error::{Error, Result};

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum ConstantPool {
    Empty = 0,
    Utf8 {
        value: std::string::String,
    } = 1,
    Integer {
        value: i32,
    } = 3,
    Float {
        value: f32,
    } = 4,
    Long {
        value: i64,
    } = 5,
    Double {
        value: f64,
    } = 6,
    Class {
        name_index: u16,
    } = 7,
    String {
        string_index: u16,
    } = 8,
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    } = 9,
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    } = 10,
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    } = 11,
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    } = 12,
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    } = 15,
    MethodType {
        descriptor_index: u16,
    } = 16,
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    } = 17,
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    } = 18,
    Module {
        name_index: u16,
    } = 19,
    Package {
        name_index: u16,
    } = 20,
}

pub(crate) fn get_constant_pool(
    data: &[u8],
    mut start_from: &mut usize,
) -> Result<Vec<ConstantPool>> {
    let constant_pool_count: u16 = get_int(&data, &mut start_from)?;
    let mut constant_pool_vec = Vec::with_capacity(constant_pool_count as usize);
    for _ in 0..constant_pool_count {
        match constant_pool_vec.last() {
            Some(val) => match val {
                Double { .. } | Long { .. } => {
                    constant_pool_vec.push(Empty);
                    continue;
                }
                _ => {}
            },
            None => {
                constant_pool_vec.push(Empty);
                continue;
            }
        }

        let tag: u8 = get_int(&data, &mut start_from)?;

        let constant_pool_entry = match tag {
            1 => Utf8 {
                value: get_string(&data, &mut start_from)?,
            },
            3 => Integer {
                value: get_int(&data, &mut start_from)?,
            },
            4 => Float {
                value: get_float(&data, &mut start_from)?,
            },
            5 => Long {
                value: get_int(&data, &mut start_from)?,
            },
            6 => Double {
                value: get_float(&data, &mut start_from)?,
            },
            7 => Class {
                name_index: get_int(&data, &mut start_from)?,
            },
            8 => String {
                string_index: get_int(&data, &mut start_from)?,
            },
            9 => Fieldref {
                class_index: get_int(&data, &mut start_from)?,
                name_and_type_index: get_int(&data, &mut start_from)?,
            },
            10 => Methodref {
                class_index: get_int(&data, &mut start_from)?,
                name_and_type_index: get_int(&data, &mut start_from)?,
            },
            11 => InterfaceMethodref {
                class_index: get_int(&data, &mut start_from)?,
                name_and_type_index: get_int(&data, &mut start_from)?,
            },
            12 => NameAndType {
                name_index: get_int(&data, &mut start_from)?,
                descriptor_index: get_int(&data, &mut start_from)?,
            },
            15 => MethodHandle {
                reference_kind: get_int(&data, &mut start_from)?,
                reference_index: get_int(&data, &mut start_from)?,
            },
            16 => MethodType {
                descriptor_index: get_int(&data, &mut start_from)?,
            },
            17 => Dynamic {
                bootstrap_method_attr_index: get_int(&data, &mut start_from)?,
                name_and_type_index: get_int(&data, &mut start_from)?,
            },
            18 => InvokeDynamic {
                bootstrap_method_attr_index: get_int(&data, &mut start_from)?,
                name_and_type_index: get_int(&data, &mut start_from)?,
            },
            19 => Module {
                name_index: get_int(&data, &mut start_from)?,
            },
            20 => Package {
                name_index: get_int(&data, &mut start_from)?,
            },
            _ => {
                return Err(Error::new_io(
                    InvalidInput,
                    format!("unmatched tag: {}", tag).as_str()));
            }
        };

        constant_pool_vec.push(constant_pool_entry);
    }

    Ok(constant_pool_vec)
}
