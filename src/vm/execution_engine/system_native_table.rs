use crate::vm::error::Result;
use crate::vm::execution_engine::system_native_table::NativeMethod::{Basic, WithMutStackFrames};
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::dispatcher::invoke::invoke;
use crate::vm::system_native::method_handle_natives::wrappers::{
    method_handle_invoke_basic_wrp, method_handle_invoke_exact_wrp, method_handle_invoke_wrp,
    var_handle_compare_and_set_wrp, var_handle_get_wrp, var_handle_set_wrp,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;

type BasicNativeMethod = fn(&[i32]) -> Result<Vec<i32>>;
type WithMutStackFramesNativeMethod = fn(&[i32], &mut StackFrames) -> Result<Vec<i32>>;

enum NativeMethod {
    Basic(BasicNativeMethod),
    WithMutStackFrames(WithMutStackFramesNativeMethod),
}

static SYSTEM_NATIVE_TABLE: Lazy<HashMap<&'static str, NativeMethod>> = Lazy::new(|| {
    let mut table = HashMap::new();
    table.insert(
        "java/lang/invoke/MethodHandle:invokeExact", // this is a normalized polymorphic signature
        WithMutStackFrames(method_handle_invoke_exact_wrp),
    );
    table.insert(
        "java/lang/invoke/MethodHandle:invokeBasic", // this is a normalized polymorphic signature
        WithMutStackFrames(method_handle_invoke_basic_wrp),
    );
    table.insert(
        "java/lang/invoke/MethodHandle:invoke", // this is a normalized polymorphic signature
        WithMutStackFrames(method_handle_invoke_wrp),
    );
    table.insert(
        "java/lang/invoke/VarHandle:set", // this is a normalized polymorphic signature
        Basic(var_handle_set_wrp),
    );
    table.insert(
        "java/lang/invoke/VarHandle:get", // this is a normalized polymorphic signature
        Basic(var_handle_get_wrp),
    );
    table.insert(
        "java/lang/invoke/VarHandle:compareAndSet", // this is a normalized polymorphic signature
        Basic(var_handle_compare_and_set_wrp),
    );

    table
});

pub(crate) fn invoke_native_method(
    method_signature: &str,
    args: &[i32],
    is_static: bool,
    stack_frames: &mut StackFrames,
) -> Result<Vec<i32>> {
    let native_method = match SYSTEM_NATIVE_TABLE.get(method_signature) {
        Some(native_method) => native_method,
        None => return invoke(method_signature, args, is_static),
    };

    match native_method {
        Basic(method) => method(args),
        WithMutStackFrames(method) => method(args, stack_frames),
    }
}
