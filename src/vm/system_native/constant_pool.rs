use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::helper::klass;
use crate::vm::method_area::cpool_helper::CPoolHelperTrait;
use crate::vm::method_area::java_class::JavaClass;
use jclassfile::constant_pool::ConstantPool;
use std::sync::Arc;

/// `jdk.internal.reflect.ConstantPool.getUTF8At0(Ljava/lang/Object;I)Ljava/lang/String;`
pub(crate) fn get_utf8_at0(_this: i32, oop_ref: i32, index: i32) -> Result<i32> {
    let klass = extract_java_class(oop_ref)?;
    let cpool_helper = klass.cpool_helper();
    let utf8 = cpool_helper.get_utf8(index as u16).ok_or_else(|| {
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
    Ok(klass.cpool_helper().raw_cpool().len() as i32)
}

/// `jdk.internal.reflect.ConstantPool.getTagAt0(Ljava/lang/Object;I)B`
pub(crate) fn get_tag_at0(_this: i32, oop_ref: i32, index: i32) -> Result<i8> {
    let klass = extract_java_class(oop_ref)?;
    let raw_pool = klass.cpool_helper().raw_cpool();

    let constant = raw_pool.get(index as usize).ok_or_else(|| {
        Error::new_constant_pool(&format!(
            "error getting tag by cpool index={index} in {}",
            klass.this_class_name()
        ))
    })?;

    let tag_code = match constant {
        ConstantPool::Empty => 0,
        ConstantPool::Utf8 { .. } => 1,
        ConstantPool::Integer { .. } => 3,
        ConstantPool::Float { .. } => 4,
        ConstantPool::Long { .. } => 5,
        ConstantPool::Double { .. } => 6,
        ConstantPool::Class { .. } => 7,
        ConstantPool::String { .. } => 8,
        ConstantPool::Fieldref { .. } => 9,
        ConstantPool::Methodref { .. } => 10,
        ConstantPool::InterfaceMethodref { .. } => 11,
        ConstantPool::NameAndType { .. } => 12,
        ConstantPool::MethodHandle { .. } => 15,
        ConstantPool::MethodType { .. } => 16,
        ConstantPool::Dynamic { .. } => 17,
        ConstantPool::InvokeDynamic { .. } => 18,
        ConstantPool::Module { .. } => 19,
        ConstantPool::Package { .. } => 20,
    };

    Ok(tag_code)
}

fn extract_java_class(constant_pool_oop_ref: i32) -> Result<Arc<JavaClass>> {
    let clazz_ref = constant_pool_oop_ref; // oop_ref is actually clazz_ref (so far)
    klass(clazz_ref)
}
