use crate::vm::error::Result;
use crate::vm::system_native::method_handle_natives::member_name::MemberName;
use crate::vm::system_native::object_offset::offset_utils::{
    object_field_offset_by_names, static_field_offset_by_names,
};

pub(crate) fn get_field_offset(member_name_ref: i32) -> Result<i64> {
    let member_name = MemberName::new(member_name_ref)?;
    let class_name = member_name.class_name();
    let name = member_name.name();
    object_field_offset_by_names(class_name, name)
}

pub(crate) fn get_static_field_offset(member_name_ref: i32) -> Result<i64> {
    let member_name = MemberName::new(member_name_ref)?;
    let class_name = member_name.class_name();
    let name = member_name.name();
    static_field_offset_by_names(class_name, name)
}
