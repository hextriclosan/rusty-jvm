//! `RegisterNatives` and `UnregisterNatives`, as HotSpot's `jni_RegisterNatives`,
//! `Method::register_native` and `jni_UnregisterNatives` do them.

use crate::vm::exception::pending_helpers::set_pending_null_pointer_exception;
use crate::vm::helper::klass;
use crate::vm::jni::utils::{set_pending_internal_error, set_pending_no_such_method_error};
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::method_area::loaded_classes::CLASSES;
use dashmap::DashMap;
use jni_sys::{jclass, jint, JNIEnv, JNINativeMethod, JNI_ERR, JNI_OK};
use std::ffi::{c_char, CStr};
use std::sync::{Arc, LazyLock};

/// What `RegisterNatives` bound to a native method.
#[derive(Clone, Copy, Debug)]
pub(crate) enum RegisteredNative {
    /// The address of the function to call.
    Function(i64),
    /// No function, for a null function pointer: like HotSpot, which then throws
    /// `UnsatisfiedLinkError` for a method it has run before, rather than linking it by its JNI
    /// symbol name again. HotSpot links a method it hasn't run yet by name, which isn't tracked
    /// here.
    Unlinked,
}

/// The bindings of `RegisterNatives`, by the declaring class and `name:descriptor` of their native
/// method. A native method without one is linked by its JNI symbol name.
static REGISTERED_NATIVES: LazyLock<DashMap<(String, String), RegisteredNative>> =
    LazyLock::new(DashMap::new);

/// `RegisterNatives`. Like HotSpot, binds the methods one by one: those before a failing one stay
/// bound.
pub(crate) unsafe extern "system" fn register_natives(
    _env: *mut JNIEnv,
    clazz: jclass,
    methods: *const JNINativeMethod,
    method_count: jint,
) -> jint {
    // HotSpot crashes on these
    if clazz.is_null() || (method_count > 0 && methods.is_null()) {
        let _ = set_pending_null_pointer_exception();
        return JNI_ERR;
    }
    let class = match klass(clazz as i32) {
        Ok(class) => class,
        Err(error) => {
            let _ = set_pending_internal_error(&error.to_string());
            return JNI_ERR;
        }
    };
    // a negative count registers nothing, as in HotSpot
    let methods = match usize::try_from(method_count) {
        Ok(count) if count > 0 => unsafe { std::slice::from_raw_parts(methods, count) },
        _ => &[],
    };

    for method in methods {
        // HotSpot crashes on these too
        if method.name.is_null() || method.signature.is_null() {
            let _ = set_pending_null_pointer_exception();
            return JNI_ERR;
        }
        if let Err(message) = unsafe { register_native(&class, method) } {
            let _ = set_pending_no_such_method_error(&message);
            return JNI_ERR;
        }
    }
    JNI_OK
}

/// Binds one native method of `class` or a superclass, or gives the message of the
/// `NoSuchMethodError` that HotSpot throws.
unsafe fn register_native(
    class: &JavaClass,
    method: &JNINativeMethod,
) -> std::result::Result<(), String> {
    let name = unsafe { decode_mutf8(method.name) };
    let signature = unsafe { decode_mutf8(method.signature) };
    // HotSpot first reports a name or signature that no loaded class uses as "not found", which
    // depends on what it has loaded; the rest it reports as below
    let external_name = || {
        let (parameters, return_type) = external_signature(&signature);
        let class_name = class.this_class_name().replace('/', ".");
        format!(
            "{return_type} {class_name}.{name}({})",
            parameters.join(", ")
        )
    };

    let name_signature = format!("{name}:{signature}");
    let Some(java_method) = lookup_method(class, &name_signature) else {
        return Err(format!(
            "Method '{}' name or signature does not match",
            external_name()
        ));
    };
    if !java_method.is_native() {
        return Err(format!(
            "Method '{}' is not declared as native",
            external_name()
        ));
    }

    let binding = if method.fnPtr.is_null() {
        RegisteredNative::Unlinked
    } else {
        RegisteredNative::Function(method.fnPtr as usize as i64)
    };
    REGISTERED_NATIVES.insert(
        (java_method.class_name().to_string(), name_signature),
        binding,
    );
    Ok(())
}

/// The method of `class` or its nearest superclass with exactly this name and descriptor, as
/// HotSpot's `Klass::lookup_method` finds it.
fn lookup_method(class: &JavaClass, name_signature: &str) -> Option<Arc<JavaMethod>> {
    // the method table also finds a @PolymorphicSignature method by its name alone
    let declared = class.try_get_method(name_signature).filter(|method| {
        format!("{}:{}", method.name(), method.get_method_descriptor()) == name_signature
    });
    declared.or_else(|| {
        let parent = CLASSES.get(class.parent().as_deref()?).ok()?;
        lookup_method(&parent, name_signature)
    })
}

/// The parameter types and return type of a method descriptor as Java source names them, such as
/// `int`, `java.lang.String` and `int[]`. Like HotSpot, a malformed descriptor gives what can be
/// read of it, so `(II` gives the parameters `int, int` and an empty return type.
fn external_signature(signature: &str) -> (Vec<String>, String) {
    let mut parameters = Vec::new();
    let mut rest = signature.strip_prefix('(').unwrap_or(signature);
    while let Some((type_name, after)) = external_type(rest) {
        parameters.push(type_name);
        rest = after;
    }
    let return_type = rest
        .strip_prefix(')')
        .and_then(external_type)
        .map(|(type_name, _)| type_name)
        .unwrap_or_default();
    (parameters, return_type)
}

/// The first type of a descriptor as Java source names it, and the rest of the descriptor.
fn external_type(descriptor: &str) -> Option<(String, &str)> {
    let dimensions = descriptor.len() - descriptor.trim_start_matches('[').len();
    let element = &descriptor[dimensions..];
    let (name, rest) = match element.chars().next()? {
        'B' => ("byte".to_string(), &element[1..]),
        'C' => ("char".to_string(), &element[1..]),
        'D' => ("double".to_string(), &element[1..]),
        'F' => ("float".to_string(), &element[1..]),
        'I' => ("int".to_string(), &element[1..]),
        'J' => ("long".to_string(), &element[1..]),
        'S' => ("short".to_string(), &element[1..]),
        'Z' => ("boolean".to_string(), &element[1..]),
        'V' => ("void".to_string(), &element[1..]),
        'L' => {
            let (class_name, rest) = element[1..].split_once(';')?;
            (class_name.replace('/', "."), rest)
        }
        _ => return None,
    };
    Some((name + &"[]".repeat(dimensions), rest))
}

/// `UnregisterNatives`. Like HotSpot, unbinds the native methods declared in the class, not those
/// of its superclasses, which are then linked by their JNI symbol names again.
pub(crate) unsafe extern "system" fn unregister_natives(_env: *mut JNIEnv, clazz: jclass) -> jint {
    // HotSpot crashes on this
    if clazz.is_null() {
        let _ = set_pending_null_pointer_exception();
        return JNI_ERR;
    }
    let class_name = match klass(clazz as i32) {
        Ok(class) => class.this_class_name().to_owned(),
        Err(error) => {
            let _ = set_pending_internal_error(&error.to_string());
            return JNI_ERR;
        }
    };
    REGISTERED_NATIVES.retain(|(declaring_class, _), _| declaring_class != &class_name);
    JNI_OK
}

/// What `RegisterNatives` bound to a native method, by its declaring class and `name:descriptor`.
pub(crate) fn registered_native(
    class_name: &str,
    name_signature: &str,
) -> Option<RegisteredNative> {
    REGISTERED_NATIVES
        .get(&(class_name.to_owned(), name_signature.to_owned()))
        .map(|entry| *entry)
}

/// A modified UTF-8 string from JNI, with any undecodable bytes replaced.
unsafe fn decode_mutf8(value: *const c_char) -> String {
    let bytes = unsafe { CStr::from_ptr(value) }.to_bytes();
    match cesu8::from_java_cesu8(bytes) {
        Ok(decoded) => decoded.into_owned(),
        Err(_) => String::from_utf8_lossy(bytes).into_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected(parameters: &[&str], return_type: &str) -> (Vec<String>, String) {
        let parameters = parameters.iter().map(|p| p.to_string()).collect();
        (parameters, return_type.to_string())
    }

    #[test]
    fn should_name_signature_types_like_hotspot() {
        assert_eq!(external_signature("()V"), expected(&[], "void"));
        assert_eq!(
            external_signature("(ILjava/lang/String;[[J)[Ljava/lang/Object;"),
            expected(
                &["int", "java.lang.String", "long[][]"],
                "java.lang.Object[]"
            )
        );
        // malformed: what can be read of it
        assert_eq!(external_signature("(II"), expected(&["int", "int"], ""));
        assert_eq!(external_signature("(Ljava/lang/String"), expected(&[], ""));
    }
}
