use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::helper::klass;
use crate::vm::method_area::java_class::JavaClass;
use jclassmodel::constant_pool::ConstantPoolLookup;
use std::sync::Arc;

/// `jdk.internal.reflect.ConstantPool.getUTF8At0(Ljava/lang/Object;I)Ljava/lang/String;`
pub(crate) fn get_utf8_at0(_this: i32, oop_ref: i32, index: i32) -> Result<i32> {
    let klass = extract_java_class(oop_ref)?;
    let constant_pool = klass.constant_pool();
    let utf8 = constant_pool.get_utf8(index as u16).ok_or_else(|| {
        Error::new_constant_pool(&format!(
            "error getting utf8 by cpool index={index} in {}",
            klass.this_class_name()
        ))
    })?;
    let string_ref = StringPoolHelper::get_string(&utf8)?;

    Ok(string_ref)
}

/// `jdk.internal.reflect.ConstantPool.getSize0(Ljava/lang/Object;)I`
pub(crate) fn get_size0(_this: i32, oop_ref: i32) -> Result<i32> {
    let klass = extract_java_class(oop_ref)?;
    Ok(klass.constant_pool().len() as i32)
}

/// `jdk.internal.reflect.ConstantPool.getTagAt0(Ljava/lang/Object;I)B`
pub(crate) fn get_tag_at0(_this: i32, oop_ref: i32, index: i32) -> Result<i8> {
    let klass = extract_java_class(oop_ref)?;

    let tag = klass
        .constant_pool()
        .tag_at(index as usize)
        .ok_or_else(|| {
            Error::new_constant_pool(&format!(
                "error getting tag by cpool index={index} in {}",
                klass.this_class_name()
            ))
        })?;

    Ok(tag.as_u8() as i8)
}

fn extract_java_class(constant_pool_oop_ref: i32) -> Result<Arc<JavaClass>> {
    let clazz_ref = constant_pool_oop_ref; // oop_ref is actually clazz_ref (so far)
    klass(clazz_ref)
}
