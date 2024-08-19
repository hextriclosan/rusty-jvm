use crate::attributes::{get_attributes, Attribute};
use crate::constant_pool::ConstantPool;
use crate::error::Result;
use crate::extractors::{get_bitfield, get_int};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, PartialEq)]
    pub struct MethodFlags: u16 {
        const ACC_PUBLIC = 0x0001;      // Declared public; may be accessed from outside its package.
        const ACC_PRIVATE = 0x0002;     // Declared private; accessible only within the defining class and other classes belonging to the same nest (§5.4.4).
        const ACC_PROTECTED = 0x0004;   // Declared protected; may be accessed within subclasses.
        const ACC_STATIC = 0x0008;      // Declared static.
        const ACC_FINAL = 0x0010;       // Declared final; must not be overridden (§5.4.5).
        const ACC_SYNCHRONIZED = 0x0020;// Declared synchronized; invocation is wrapped by a monitor use.
        const ACC_BRIDGE = 0x0040;      // A bridge method, generated by the compiler.
        const ACC_VARARGS = 0x0080;     // Declared with variable number of arguments.
        const ACC_NATIVE = 0x0100;      // Declared native; implemented in a language other than the Java programming language.
        const ACC_ABSTRACT = 0x0400;    // Declared abstract; no implementation is provided.
        const ACC_STRICT = 0x0800;      // In a class file whose major version number is at least 46 and at most 60: Declared strictfp.
        const ACC_SYNTHETIC = 0x1000;   // Declared synthetic; not present in the source code.
    }
}

#[derive(Debug, PartialEq)]
pub struct MethodInfo {
    access_flags: MethodFlags,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<Attribute>,
}

impl MethodInfo {
    pub fn new(
        access_flags: MethodFlags,
        name_index: u16,
        descriptor_index: u16,
        attributes: Vec<Attribute>,
    ) -> Self {
        Self {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        }
    }
    pub fn access_flags(&self) -> &MethodFlags {
        &self.access_flags
    }
    pub fn name_index(&self) -> u16 {
        self.name_index
    }
    pub fn descriptor_index(&self) -> u16 {
        self.descriptor_index
    }
    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }
}

pub(crate) fn get_methods(
    data: &[u8],
    mut start_from: &mut usize,
    constant_pool_vec: &Vec<ConstantPool>,
) -> Result<Vec<MethodInfo>> {
    let methods_count: u16 = get_int(&data, &mut start_from)?;
    let mut methods = Vec::with_capacity(methods_count as usize);
    for _ in 0..methods_count {
        methods.push(MethodInfo::new(
            get_bitfield(&data, &mut start_from)?,
            get_int(&data, &mut start_from)?,
            get_int(&data, &mut start_from)?,
            get_attributes(&data, &mut start_from, &constant_pool_vec)?,
        ))
    }

    Ok(methods)
}
