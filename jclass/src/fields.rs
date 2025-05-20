use crate::attributes::{get_attributes, Attribute};
use crate::constant_pool::ConstantPool;
use crate::error::Result;
use crate::extractors::{get_bitfield, get_int};
use bitflags::bitflags;
use derive_new::new;
use getset::{CopyGetters, Getters};

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

#[derive(Debug, PartialEq, Getters, CopyGetters, new)]
pub struct FieldInfo {
    #[get = "pub"]
    access_flags: FieldFlags,
    #[get_copy = "pub"]
    name_index: u16,
    #[get_copy = "pub"]
    descriptor_index: u16,
    #[get = "pub"]
    attributes: Vec<Attribute>,
}

pub(crate) fn get_fields(
    data: &&[u8],
    mut start_from: &mut usize,
    constant_pool_vec: &Vec<ConstantPool>,
) -> Result<Vec<FieldInfo>> {
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
