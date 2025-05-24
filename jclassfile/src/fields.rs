use crate::attributes::{get_attributes, Attribute};
use crate::constant_pool::ConstantPool;
use crate::error::Result;
use crate::extractors::{get_bitfield, get_int};
use bitflags::bitflags;
use derive_new::new;
use getset::{CopyGetters, Getters};

bitflags! {
    #[derive(Debug, PartialEq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    /// Field access and property modifiers
    pub struct FieldFlags: u16 {
        /// Declared public; may be accessed from outside its package.
        const ACC_PUBLIC = 0x0001;
        /// Declared private; accessible only within the defining class and other classes belonging to the same nest (JVMS §5.4.4).
        const ACC_PRIVATE = 0x0002;
        /// Declared protected; may be accessed within subclasses.
        const ACC_PROTECTED = 0x0004;
        /// Declared static.
        const ACC_STATIC = 0x0008;
        /// Declared final; never directly assigned to after object construction (JLS §17.5).
        const ACC_FINAL = 0x0010;
        /// Declared volatile; cannot be cached.
        const ACC_VOLATILE = 0x0040;
        /// Declared transient; not written or read by a persistent object manager.
        const ACC_TRANSIENT = 0x0080;
        /// Declared synthetic; not present in the source code.
        const ACC_SYNTHETIC = 0x1000;
        /// Declared as an element of an enum class.
        const ACC_ENUM = 0x4000;
    }
}

#[derive(Debug, PartialEq, Getters, CopyGetters, new)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `field_info` structure (JVMS §4.5).
pub struct FieldInfo {
    #[get = "pub"]
    /// Access and property flags for the field
    access_flags: FieldFlags,
    #[get_copy = "pub"]
    /// Name index in the constant pool
    name_index: u16,
    #[get_copy = "pub"]
    /// Descriptor index in the constant pool
    descriptor_index: u16,
    #[get = "pub"]
    /// Attributes associated with the field
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
