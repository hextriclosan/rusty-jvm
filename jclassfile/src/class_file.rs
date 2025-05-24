use crate::attributes::*;
use crate::constant_pool::{get_constant_pool, ConstantPool};
use crate::error::{Error, Result};
use crate::extractors::{get_bitfield, get_int};
use crate::fields::get_fields;
use crate::methods::{get_methods, MethodInfo};
use std::io;

const MAGIC: u32 = 0xCAFEBABE;

use crate::attributes::Attribute;
use crate::error::ErrorKind::InvalidInput;
use crate::fields::FieldInfo;
use bitflags::bitflags;
use derive_new::new;
use getset::{CopyGetters, Getters};

bitflags! {
    #[derive(Debug, PartialEq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    /// Class access and property modifiers
    pub struct ClassFlags: u16 {
        /// Declared public; may be accessed from outside its package.
        const ACC_PUBLIC = 0x0001;
        /// Declared final; no subclasses allowed.
        const ACC_FINAL = 0x0010;
        /// Treat superclass methods specially when invoked by the invokespecial instruction.
        const ACC_SUPER = 0x0020;
        /// Is an interface, not a class.
        const ACC_INTERFACE = 0x0200;
        /// Declared abstract; must not be instantiated.
        const ACC_ABSTRACT = 0x0400;
        /// Declared synthetic; not present in the source code.
        const ACC_SYNTHETIC = 0x1000;
        /// Declared as an annotation interface.
        const ACC_ANNOTATION = 0x2000;
        /// Declared as an enum class.
        const ACC_ENUM = 0x4000;
        /// Is a module, not a class or interface.
        const ACC_MODULE = 0x8000;
    }
}

#[derive(Debug, PartialEq, Getters, CopyGetters, new)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `ClassFile` structure (JVMS §4.1).
pub struct ClassFile {
    #[get_copy = "pub"]
    /// The magic item supplies the magic number identifying the class file format; it has the value 0xCAFEBABE.
    magic: u32,
    #[get_copy = "pub"]
    /// The minor version number of the class file format.
    minor_version: u16,
    #[get_copy = "pub"]
    /// The major version number of the class file format; 45 means java 1.0.2, ... 68 means java 24.
    major_version: u16,
    #[get = "pub"]
    /// The constant pool is a table of structures representing various kinds of constants used in the class file.
    constant_pool: Vec<ConstantPool>,
    #[get = "pub"]
    /// Access and property flags for the class or interface.
    access_flags: ClassFlags,
    #[get_copy = "pub"]
    /// Index of `constant_pool` table of class or interface that is defined by this class file.
    this_class: u16,
    #[get_copy = "pub"]
    /// Index of `constant_pool` table of class or interface that is the superclass of this class.
    super_class: u16,
    #[get = "pub"]
    /// The interfaces implemented by this class or interface.
    interfaces: Vec<u16>,
    #[get = "pub"]
    /// The fields of this class or interface.
    fields: Vec<FieldInfo>,
    #[get = "pub"]
    /// The methods of this class or interface.
    methods: Vec<MethodInfo>,
    #[get = "pub"]
    /// The attributes of this class or interface.
    attributes: Vec<Attribute>,
}

/// Parses a class file from a byte slice.
pub fn parse(data: &[u8]) -> Result<ClassFile> {
    let mut start_from = 0;
    let magic = get_int(&data, &mut start_from)?;

    if magic != MAGIC {
        return Err(Error::new(InvalidInput(String::from(
            "Not a valid class-file",
        ))));
    }

    let minor_version = get_int(&data, &mut start_from)?;
    let major_version = get_int(&data, &mut start_from)?;
    let constant_pool_vec = get_constant_pool(&data, &mut start_from)?;
    let access_flags = get_bitfield(&data, &mut start_from)?;
    let this_class = get_int(&data, &mut start_from)?;
    let super_class = get_int(&data, &mut start_from)?;
    let interfaces = get_interfaces(&data, &mut start_from)?;
    let fields = get_fields(&data, &mut start_from, &constant_pool_vec)?;
    let methods = get_methods(&data, &mut start_from, &constant_pool_vec)?;
    let attributes = get_attributes(&data, &mut start_from, &constant_pool_vec)?;

    if data.len() != start_from {
        return Err(Error::new_io(
            io::ErrorKind::InvalidInput,
            format!(
                "Not all was read : data.len() is {}, start_from is {}",
                data.len(),
                start_from
            )
            .as_str(),
        ));
    }

    Ok(ClassFile::new(
        magic,
        minor_version,
        major_version,
        constant_pool_vec,
        access_flags,
        this_class,
        super_class,
        interfaces,
        fields,
        methods,
        attributes,
    ))
}

fn get_interfaces(data: &&[u8], mut start_from: &mut usize) -> Result<Vec<u16>> {
    let interfaces_count: u16 = get_int(&data, &mut start_from)?;

    let mut interfaces = Vec::with_capacity(interfaces_count as usize);
    for _ in 0..interfaces_count {
        interfaces.push(get_int(&data, &mut start_from)?);
    }

    Ok(interfaces)
}
