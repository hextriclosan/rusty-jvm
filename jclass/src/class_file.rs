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
use bitflags::bitflags;

use crate::fields::FieldInfo;

bitflags! {
    #[derive(Debug, PartialEq)]
    pub struct ClassFlags: u16 {
        const ACC_PUBLIC = 0x0001;      // Declared public; may be accessed from outside its package.
        const ACC_FINAL = 0x0010;       // Declared final; no subclasses allowed.
        const ACC_SUPER = 0x0020;       // Treat superclass methods specially when invoked by the invokespecial instruction.
        const ACC_INTERFACE = 0x0200;   // Is an interface, not a class.
        const ACC_ABSTRACT = 0x0400;    // Declared abstract; must not be instantiated.
        const ACC_SYNTHETIC = 0x1000;   // Declared synthetic; not present in the source code.
        const ACC_ANNOTATION = 0x2000;  // Declared as an annotation interface.
        const ACC_ENUM = 0x4000;        // Declared as an enum class.
        const ACC_MODULE = 0x8000;      // Is a module, not a class or interface.
    }
}

#[derive(Debug, PartialEq)]
pub struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<ConstantPool>,
    access_flags: ClassFlags,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
    attributes: Vec<Attribute>,
}

impl ClassFile {
    pub fn new(
        magic: u32,
        minor_version: u16,
        major_version: u16,
        constant_pool: Vec<ConstantPool>,
        access_flags: ClassFlags,
        this_class: u16,
        super_class: u16,
        interfaces: Vec<u16>,
        fields: Vec<FieldInfo>,
        methods: Vec<MethodInfo>,
        attributes: Vec<Attribute>,
    ) -> Self {
        Self {
            magic,
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        }
    }
    pub fn magic(&self) -> u32 {
        self.magic
    }
    pub fn minor_version(&self) -> u16 {
        self.minor_version
    }
    pub fn major_version(&self) -> u16 {
        self.major_version
    }
    pub fn constant_pool(&self) -> &Vec<ConstantPool> {
        &self.constant_pool
    }
    pub fn access_flags(&self) -> &ClassFlags {
        &self.access_flags
    }
    pub fn this_class(&self) -> u16 {
        self.this_class
    }
    pub fn super_class(&self) -> u16 {
        self.super_class
    }
    pub fn interfaces(&self) -> &Vec<u16> {
        &self.interfaces
    }
    pub fn fields(&self) -> &Vec<FieldInfo> {
        &self.fields
    }
    pub fn methods(&self) -> &Vec<MethodInfo> {
        &self.methods
    }
    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }
}

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
