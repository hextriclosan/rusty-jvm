use std::io;
use bitflags::bitflags;
use crate::attributes::{Attribute, get_attributes};
use crate::constant_pool::ConstantPool;
use crate::extractors::{get_bitfield, get_int};
bitflags! {
    #[derive(Debug, PartialEq)]
    pub struct FieldFlags: u16 {
        const ACC_PUBLIC = 0x0001;      // Declared public; may be accessed from outside its package.
        const ACC_PRIVATE = 0x0002;     // Declared private; accessible only within the defining class and other classes belonging to the same nest (§5.4.4).
        const ACC_PROTECTED = 0x0004;   // Declared protected; may be accessed within subclasses.
        const ACC_STATIC = 0x0008;      // Declared static.
        const ACC_FINAL = 0x0010;       // Declared final; never directly assigned to after object construction (JLS §17.5).
        const ACC_VOLATILE = 0x0040;    // Declared volatile; cannot be cached.
        const ACC_TRANSIENT = 0x0080;   // Declared transient; not written or read by a persistent object manager.
        const ACC_SYNTHETIC = 0x1000;   // Declared synthetic; not present in the source code.
        const ACC_ENUM = 0x4000;        // Declared as an element of an enum class.
    }
}

#[derive(Debug, PartialEq)]
pub struct FieldInfo {
    access_flags: FieldFlags,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<Attribute>,
}

impl FieldInfo {
    pub fn new(
        access_flags: FieldFlags,
        name_index: u16,
        descriptor_index: u16,
        attributes: Vec<Attribute>) -> Self {
        Self { access_flags, name_index, descriptor_index, attributes }
    }
}

pub(crate) fn get_fields(data: &&[u8], mut start_from: &mut usize, constant_pool_vec: &Vec<ConstantPool>) -> Result<Vec<FieldInfo>, io::Error> {
    let fields_count: u16 = get_int(&data, &mut start_from)?;

    let mut fields = Vec::with_capacity(fields_count as usize);
    for _ in 0..fields_count {
        fields.push(FieldInfo::new(
            get_bitfield(&data, &mut start_from)?,
            get_int(&data, &mut start_from)?,
            get_int(&data, &mut start_from)?,
            get_attributes(&data, &mut start_from, &constant_pool_vec)?,
        ))
    }

    Ok(fields)
}
