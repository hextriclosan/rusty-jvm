use crate::vm::error::Result;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::clazz_ref;
use crate::vm::system_native::method_handle_natives::member_name::MemberName;
use crate::vm::system_native::method_handle_natives::offsets::{
    get_field_offset, get_static_field_offset,
};
use crate::vm::system_native::method_handle_natives::resolution::member_name_init;

pub(crate) mod invocation;
mod member_name;
mod method_type;
mod native_accessor;
mod offsets;
mod resolution;
mod resolved_method_name;
pub(crate) mod types;
mod var_handle;
pub(crate) mod wrappers;

/// `java.lang.invoke.MethodHandleNatives.registerNatives()V`
pub(crate) fn register_natives() -> Result<()> {
    Ok(()) // todo: implement me
}

/// `java.lang.invoke.MethodHandleNatives.init(Ljava/lang/invoke/MemberName;Ljava/lang/Object;)V`
pub(crate) fn init(member_name_ref: i32, obj_ref: i32) -> Result<()> {
    member_name_init(member_name_ref, obj_ref)
}

/// `java.lang.invoke.MethodHandleNatives.resolve(Ljava/lang/invoke/MemberName;Ljava/lang/Class;IZ)Ljava/lang/invoke/MemberName;`
pub(crate) fn resolve(
    member_name_ref: i32,
    caller_class_ref: i32,
    lookup_mode: i32,
    speculative_resolve: bool,
) -> Result<i32> {
    resolution::resolve(
        member_name_ref,
        caller_class_ref,
        lookup_mode,
        speculative_resolve,
    )
}

/// `java.lang.invoke.MethodHandleNatives.objectFieldOffset(Ljava/lang/invoke/MemberName;)J`
pub(crate) fn object_field_offset(member_name_ref: i32) -> Result<i64> {
    get_field_offset(member_name_ref)
}

/// `java.lang.invoke.MethodHandleNatives.staticFieldOffset(Ljava/lang/invoke/MemberName;)J`
pub(crate) fn static_field_offset(member_name_ref: i32) -> Result<i64> {
    get_static_field_offset(member_name_ref)
}

/// `java.lang.invoke.MethodHandleNatives.staticFieldBase(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;`
pub(crate) fn static_field_base(member_name_ref: i32) -> Result<i32> {
    let member_name = MemberName::new(member_name_ref)?;
    let class_name = member_name.class_name();
    clazz_ref(class_name)
}

/// `java.lang.invoke.MethodHandleNatives.getNamedCon(I[Ljava/lang/Object;)I`
// By not setting a value for `box` we break the check loop on the Java side so, just bypassing the check
pub(crate) fn get_named_con(_what: i32, _box_ref: i32) -> Result<i32> {
    Ok(0) // todo: implement me
}

/// `java.lang.invoke.MethodHandleNatives.getMemberVMInfo(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;`
pub(crate) fn get_member_vm_info(member_name_ref: i32) -> Result<i32> {
    // returns {vmindex,vmtarget}
    let member_name = MemberName::new(member_name_ref)?;
    member_name.get_member_vm_info()
}

/// `java.lang.invoke.MethodHandleNatives.setCallSiteTargetNormal(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V`
pub(crate) fn set_call_site_target_normal(
    call_site_ref: i32,
    method_handle_ref: i32,
) -> Result<()> {
    let call_site_name = HEAP.get_instance_name(call_site_ref)?;
    HEAP.set_object_field_value(
        call_site_ref,
        &call_site_name,
        "target",
        vec![method_handle_ref],
    )
}
