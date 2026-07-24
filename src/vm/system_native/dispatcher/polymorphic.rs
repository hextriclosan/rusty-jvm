use crate::vm::error::{Error, Result};
use crate::vm::system_native::method_handle_natives::wrappers::{
    method_handle_invoke_basic_wrp, method_handle_invoke_exact_wrp, method_handle_invoke_wrp,
    var_handle_compare_and_exchange_wrp, var_handle_compare_and_set_wrp, var_handle_get_wrp,
    var_handle_set_wrp,
};

pub(crate) fn invoke_polymorphic(
    class_name: &str,
    method_name: &str,
    args: &[i32],
) -> Result<Vec<i32>> {
    match (class_name, method_name) {
        ("java/lang/invoke/MethodHandle", "invokeExact") => method_handle_invoke_exact_wrp(args),
        ("java/lang/invoke/MethodHandle", "invokeBasic") => method_handle_invoke_basic_wrp(args),
        ("java/lang/invoke/MethodHandle", "invoke") => method_handle_invoke_wrp(args),
        ("java/lang/invoke/VarHandle", "set") => var_handle_set_wrp(args),
        ("java/lang/invoke/VarHandle", "get") => var_handle_get_wrp(args),
        ("java/lang/invoke/VarHandle", "compareAndSet") => var_handle_compare_and_set_wrp(args),
        // A strong (volatile) CAS validly implements every `weakCompareAndSet*`: they are all
        // permitted to fail spuriously and only differ in memory ordering (plain / acquire / release),
        // which a stronger ordering satisfies. Callers loop and retry regardless
        // (e.g. `AtomicReference.updateAndGet`). Route all four spellings to the same strong CAS.
        (
            "java/lang/invoke/VarHandle",
            "weakCompareAndSet"
            | "weakCompareAndSetPlain"
            | "weakCompareAndSetAcquire"
            | "weakCompareAndSetRelease",
        ) => var_handle_compare_and_set_wrp(args),
        // Likewise the acquire/release-ordered exchanges reduce to the volatile `compareAndExchange`.
        (
            "java/lang/invoke/VarHandle",
            "compareAndExchange" | "compareAndExchangeAcquire" | "compareAndExchangeRelease",
        ) => var_handle_compare_and_exchange_wrp(args),
        _ => Err(Error::new_native(&format!(
            "Unknown polymorphic-signature native method: {class_name}:{method_name}"
        ))),
    }
}
