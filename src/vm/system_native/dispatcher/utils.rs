use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::helper::vec_to_i64;
use jdescriptor::TypeDescriptor;
use libffi::middle::Type;

pub(super) fn to_ffitype(type_descriptor: &TypeDescriptor) -> Type {
    match type_descriptor {
        TypeDescriptor::Byte => Type::i8(),
        TypeDescriptor::Char => Type::u16(),
        TypeDescriptor::Double => Type::f64(),
        TypeDescriptor::Float => Type::f32(),
        TypeDescriptor::Integer => Type::i32(),
        TypeDescriptor::Long => Type::i64(),
        TypeDescriptor::Short => Type::i16(),
        TypeDescriptor::Boolean => Type::u8(),
        TypeDescriptor::Void => Type::void(),
        TypeDescriptor::Array(_, _) => Type::pointer(),
        TypeDescriptor::Object(_) => Type::pointer(),
    }
}

pub(super) fn get_symbol_address(name: &String, native_libraries_ref: i32) -> Result<Option<i64>> {
    let short_name_ref = StringPoolHelper::get_string(&name)?;
    let symbol_address = Executor::invoke_non_static_method(
        "jdk/internal/loader/NativeLibraries",
        "find:(Ljava/lang/String;)J",
        native_libraries_ref,
        &[short_name_ref.into()],
    )?;
    let symbol_address = vec_to_i64(&symbol_address);
    Ok(if symbol_address != 0 {
        Some(symbol_address)
    } else {
        None
    })
}
