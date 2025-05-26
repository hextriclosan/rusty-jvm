use crate::error::Error;
use crate::execution_engine::string_pool_helper::StringPoolHelper;
use crate::method_area::cpool_helper::CPoolHelperTrait;
use crate::method_area::java_class::JavaClass;
use crate::method_area::method_area::with_method_area;
use jclassfile::constant_pool::ConstantPool;
use std::sync::Arc;

pub(crate) fn constant_pool_get_utf8_at0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let _constant_pool_ref = args[0];
    let constant_pool_oop_ref = args[1];
    let constant_pool_index = args[2];

    let string_ref = get_utf8(constant_pool_oop_ref, constant_pool_index)?;
    Ok(vec![string_ref])
}
fn get_utf8(oop_ref: i32, index: i32) -> crate::error::Result<i32> {
    let jc = extract_java_class(oop_ref)?;
    let cpool_helper = jc.cpool_helper();
    let utf8 = cpool_helper.get_utf8(index as u16).ok_or_else(|| {
        Error::new_constant_pool(&format!(
            "error getting utf8 by cpool index={index} in {}",
            jc.this_class_name()
        ))
    })?;
    let string_ref = StringPoolHelper::get_string(utf8)?;

    Ok(string_ref)
}

pub(crate) fn constant_pool_get_size0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let _constant_pool_ref = args[0];
    let constant_pool_oop_ref = args[1];

    let size = get_size0(constant_pool_oop_ref)?;
    Ok(vec![size])
}
fn get_size0(oop_ref: i32) -> crate::error::Result<i32> {
    let jc = extract_java_class(oop_ref)?;
    Ok(jc.cpool_helper().raw_cpool().len() as i32)
}

pub(crate) fn constant_pool_get_tag_at0_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let _constant_pool_ref = args[0];
    let constant_pool_oop_ref = args[1];
    let constant_pool_index = args[2];

    let tag_byte_value = get_tag_at0(constant_pool_oop_ref, constant_pool_index)?;
    Ok(vec![tag_byte_value as i32])
}
fn get_tag_at0(oop_ref: i32, index: i32) -> crate::error::Result<u8> {
    let jc = extract_java_class(oop_ref)?;
    let raw_pool = jc.cpool_helper().raw_cpool();

    let constant = raw_pool.get(index as usize).ok_or_else(|| {
        Error::new_constant_pool(&format!(
            "error getting tag by cpool index={index} in {}",
            jc.this_class_name()
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

fn extract_java_class(constant_pool_oop_ref: i32) -> crate::error::Result<Arc<JavaClass>> {
    let clazz_ref = constant_pool_oop_ref; // oop_ref is actually clazz_ref (so far)
    let jc = with_method_area(|method_area| {
        let class_name = method_area.get_from_reflection_table(clazz_ref)?;
        method_area.get(&class_name)
    })?;
    Ok(jc)
}
