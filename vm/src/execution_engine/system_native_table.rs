use crate::error::Error;
use crate::helper::i64_to_vec;
use crate::system_native::class::{get_modifiers_wrp, get_primitive_class_wrp};
use crate::system_native::string::intern_wrp;
use crate::system_native::system::{arraycopy_wrp, current_time_millis_wrp};
use once_cell::sync::Lazy;
use std::collections::HashMap;

static SYSTEM_NATIVE_TABLE: Lazy<
    HashMap<&'static str, fn(&[i32]) -> crate::error::Result<Vec<i32>>>,
> = Lazy::new(|| {
    let mut table = HashMap::new();
    table.insert(
        "java/lang/System:currentTimeMillis:()J",
        current_time_millis_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/System:arraycopy:(Ljava/lang/Object;ILjava/lang/Object;II)V",
        arraycopy_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Class:getModifiers:()I",
        get_modifiers_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Class:getPrimitiveClass:(Ljava/lang/String;)Ljava/lang/Class;",
        get_primitive_class_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Class:desiredAssertionStatus0:(Ljava/lang/Class;)Z",
        bool_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/Unsafe:registerNatives:()V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/Unsafe:arrayBaseOffset0:(Ljava/lang/Class;)I",
        int_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/Unsafe:arrayIndexScale0:(Ljava/lang/Class;)I",
        int_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/String:intern:()Ljava/lang/String;",
        intern_wrp as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Float:floatToRawIntBits:(F)I",
        return_argument_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Double:doubleToRawLongBits:(D)J",
        return_argument_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "java/lang/Double:longBitsToDouble:(J)D",
        return_argument_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/CDS:initializeFromArchive:(Ljava/lang/Class;)V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert(
        "jdk/internal/misc/VM:initialize:()V",
        void_stub as fn(&[i32]) -> crate::error::Result<Vec<i32>>,
    );
    table.insert("java/lang/Runtime:maxMemory:()J", |_args: &[i32]| {
        return_argument_stub(&i64_to_vec(i64::MAX))
    });

    table
});

pub(crate) fn invoke_native_method(
    method_signature: &str,
    args: &[i32],
) -> crate::error::Result<Vec<i32>> {
    let native_method = SYSTEM_NATIVE_TABLE.get(method_signature).ok_or_else(|| {
        Error::new_native(&format!("Native method {method_signature} not found"))
    })?;

    let result = native_method(args)?;

    Ok(result)
}

fn void_stub(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    Ok(vec![])
}

fn bool_stub(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    Ok(vec![false as i32])
}

fn int_stub(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    Ok(vec![0])
}

fn return_argument_stub(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    Ok(args.to_vec())
}
