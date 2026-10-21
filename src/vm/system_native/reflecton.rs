use crate::vm::error::Result;
use crate::vm::exception::pending_helpers::set_pending_internal_error_with_message;
use crate::vm::helper::{clazz_ref, klass, strip_nest_host};
use crate::vm::stack::stack_frames_util::{CallerClass, StackFramesUtil};

/// `jdk.internal.reflect.Reflection.getCallerClass()Ljava/lang/Class;`
pub(crate) fn get_caller_class() -> Result<i32> {
    match StackFramesUtil::get_caller_class_name()? {
        CallerClass::Found(caller_name) => clazz_ref(&caller_name),
        CallerClass::NotFound => Ok(0),
        CallerClass::Misuse(message) => {
            set_pending_internal_error_with_message(&message)?;
            Ok(0)
        }
    }
}

/// `jdk.internal.reflect.Reflection.getClassAccessFlags(Ljava/lang/Class;)I`
pub(crate) fn get_class_access_flags(class_ref: i32) -> Result<i32> {
    let klass = klass(class_ref)?;
    let flags = klass.class_modifiers().bits() as i32;
    Ok(flags)
}

/// `jdk.internal.reflect.Reflection.areNestMates(Ljava/lang/Class;Ljava/lang/Class;)Z`
pub(crate) fn are_nest_mates(current_class_ref: i32, member_class_ref: i32) -> Result<bool> {
    let current_klass = klass(current_class_ref)?;
    let member_klass = klass(member_class_ref)?;
    let current_class_name = current_klass.this_class_name();
    let member_class_name = member_klass.this_class_name();

    let current_class_nest_host =
        strip_nest_host(current_class_name).unwrap_or(current_class_name.as_str());

    let member_class_nest_host =
        strip_nest_host(member_class_name).unwrap_or(member_class_name.as_str());

    Ok(current_class_nest_host == member_class_nest_host)
}
