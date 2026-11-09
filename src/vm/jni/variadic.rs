use crate::vm::helper::klass;
use crate::vm::jni::utils::{decode_method_id, set_pending_internal_error};
use crate::vm::jni::{
    instance_methods_impl as instance, object_operations_impl, static_methods_impl as static_,
};
use jdescriptor::TypeDescriptor;
use jni_sys::{
    jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jmethodID, jobject, jshort,
    jvalue, va_list, JNIEnv,
};
use std::ffi::{c_char, c_void};

#[derive(Clone, Copy)]
enum InvocationKind {
    Virtual,
    Nonvirtual,
    Static,
    Constructor,
}

extern "C" {
    fn rusty_jvm_next_argument(arguments: *mut c_void, tag: c_char, output: *mut jvalue);
}

#[no_mangle]
unsafe extern "C" fn rusty_jvm_invoke_variadic(
    env: *mut JNIEnv,
    receiver: jobject,
    target: jclass,
    method_id: jmethodID,
    arguments: *mut c_void,
    kind: jint,
) -> jvalue {
    let kind = match kind {
        0 => InvocationKind::Virtual,
        1 => InvocationKind::Nonvirtual,
        2 => InvocationKind::Static,
        3 => InvocationKind::Constructor,
        _ => {
            set_pending_internal_error("Unknown JNI invocation kind");
            return jvalue { j: 0 };
        }
    };
    let (class_ref, index) = decode_method_id(method_id as usize);
    let method = match klass(class_ref).and_then(|class| class.get_method_by_index(index)) {
        Ok(method) => method,
        Err(error) => {
            set_pending_internal_error(&format!("Failed to resolve variadic JNI method: {error}"));
            return jvalue { j: 0 };
        }
    };
    let descriptor = method.get_method_descriptor();
    let values: Vec<jvalue> = descriptor
        .parameter_types()
        .iter()
        .map(|parameter| {
            let tag = match parameter {
                TypeDescriptor::Boolean => b'Z',
                TypeDescriptor::Byte => b'B',
                TypeDescriptor::Char => b'C',
                TypeDescriptor::Short => b'S',
                TypeDescriptor::Integer => b'I',
                TypeDescriptor::Long => b'J',
                TypeDescriptor::Float => b'F',
                TypeDescriptor::Double => b'D',
                TypeDescriptor::Object(_) | TypeDescriptor::Array(_, _) => b'L',
                TypeDescriptor::Void => unreachable!(),
            };
            let mut value = jvalue { j: 0 };
            rusty_jvm_next_argument(arguments, tag as c_char, &mut value);
            value
        })
        .collect();
    if matches!(kind, InvocationKind::Constructor) {
        return jvalue {
            l: object_operations_impl::new_object_a(env, target, method_id, values.as_ptr()),
        };
    }
    macro_rules! invoke {
        ($member:ident, $virtual:ident, $nonvirtual:ident, $static:ident) => {
            jvalue {
                $member: match kind {
                    InvocationKind::Virtual => {
                        instance::$virtual(env, receiver, method_id, values.as_ptr())
                    }
                    InvocationKind::Nonvirtual => {
                        instance::$nonvirtual(env, receiver, target, method_id, values.as_ptr())
                    }
                    InvocationKind::Static => {
                        static_::$static(env, target, method_id, values.as_ptr())
                    }
                    InvocationKind::Constructor => unreachable!(),
                },
            }
        };
    }
    match descriptor.return_type() {
        TypeDescriptor::Boolean => invoke!(
            z,
            call_boolean_method_a,
            call_non_virtual_boolean_method_a,
            call_static_boolean_method_a
        ),
        TypeDescriptor::Byte => invoke!(
            b,
            call_byte_method_a,
            call_non_virtual_byte_method_a,
            call_static_byte_method_a
        ),
        TypeDescriptor::Char => invoke!(
            c,
            call_char_method_a,
            call_non_virtual_char_method_a,
            call_static_char_method_a
        ),
        TypeDescriptor::Short => invoke!(
            s,
            call_short_method_a,
            call_non_virtual_short_method_a,
            call_static_short_method_a
        ),
        TypeDescriptor::Integer => invoke!(
            i,
            call_int_method_a,
            call_non_virtual_int_method_a,
            call_static_int_method_a
        ),
        TypeDescriptor::Long => invoke!(
            j,
            call_long_method_a,
            call_non_virtual_long_method_a,
            call_static_long_method_a
        ),
        TypeDescriptor::Float => invoke!(
            f,
            call_float_method_a,
            call_non_virtual_float_method_a,
            call_static_float_method_a
        ),
        TypeDescriptor::Double => invoke!(
            d,
            call_double_method_a,
            call_non_virtual_double_method_a,
            call_static_double_method_a
        ),
        TypeDescriptor::Object(_) | TypeDescriptor::Array(_, _) => invoke!(
            l,
            call_object_method_a,
            call_non_virtual_object_method_a,
            call_static_object_method_a
        ),
        TypeDescriptor::Void => {
            match kind {
                InvocationKind::Virtual => {
                    instance::call_void_method_a(env, receiver, method_id, values.as_ptr())
                }
                InvocationKind::Nonvirtual => instance::call_non_virtual_void_method_a(
                    env,
                    receiver,
                    target,
                    method_id,
                    values.as_ptr(),
                ),
                InvocationKind::Static => {
                    static_::call_static_void_method_a(env, target, method_id, values.as_ptr())
                }
                InvocationKind::Constructor => unreachable!(),
            }
            jvalue { j: 0 }
        }
    }
}

macro_rules! declare_call {
    ($plain:ident, $list:ident, ($($parameter:ty),*) -> $result:ty) => {
        extern "C" {
            pub(super) fn $plain(env: *mut JNIEnv, $(_: $parameter),*, ...) -> $result;
        }
        extern "system" {
            pub(super) fn $list(env: *mut JNIEnv, $(_: $parameter),*, arguments: va_list) -> $result;
        }
    };
}
declare_call!(rusty_jvm_NewObject, rusty_jvm_NewObjectV, (jclass, jmethodID) -> jobject);
declare_call!(rusty_jvm_CallObjectMethod, rusty_jvm_CallObjectMethodV, (jobject, jmethodID) -> jobject);
declare_call!(rusty_jvm_CallNonvirtualObjectMethod, rusty_jvm_CallNonvirtualObjectMethodV, (jobject, jclass, jmethodID) -> jobject);
declare_call!(rusty_jvm_CallStaticObjectMethod, rusty_jvm_CallStaticObjectMethodV, (jclass, jmethodID) -> jobject);
declare_call!(rusty_jvm_CallBooleanMethod, rusty_jvm_CallBooleanMethodV, (jobject, jmethodID) -> jboolean);
declare_call!(rusty_jvm_CallNonvirtualBooleanMethod, rusty_jvm_CallNonvirtualBooleanMethodV, (jobject, jclass, jmethodID) -> jboolean);
declare_call!(rusty_jvm_CallStaticBooleanMethod, rusty_jvm_CallStaticBooleanMethodV, (jclass, jmethodID) -> jboolean);
declare_call!(rusty_jvm_CallByteMethod, rusty_jvm_CallByteMethodV, (jobject, jmethodID) -> jbyte);
declare_call!(rusty_jvm_CallNonvirtualByteMethod, rusty_jvm_CallNonvirtualByteMethodV, (jobject, jclass, jmethodID) -> jbyte);
declare_call!(rusty_jvm_CallStaticByteMethod, rusty_jvm_CallStaticByteMethodV, (jclass, jmethodID) -> jbyte);
declare_call!(rusty_jvm_CallCharMethod, rusty_jvm_CallCharMethodV, (jobject, jmethodID) -> jchar);
declare_call!(rusty_jvm_CallNonvirtualCharMethod, rusty_jvm_CallNonvirtualCharMethodV, (jobject, jclass, jmethodID) -> jchar);
declare_call!(rusty_jvm_CallStaticCharMethod, rusty_jvm_CallStaticCharMethodV, (jclass, jmethodID) -> jchar);
declare_call!(rusty_jvm_CallShortMethod, rusty_jvm_CallShortMethodV, (jobject, jmethodID) -> jshort);
declare_call!(rusty_jvm_CallNonvirtualShortMethod, rusty_jvm_CallNonvirtualShortMethodV, (jobject, jclass, jmethodID) -> jshort);
declare_call!(rusty_jvm_CallStaticShortMethod, rusty_jvm_CallStaticShortMethodV, (jclass, jmethodID) -> jshort);
declare_call!(rusty_jvm_CallIntMethod, rusty_jvm_CallIntMethodV, (jobject, jmethodID) -> jint);
declare_call!(rusty_jvm_CallNonvirtualIntMethod, rusty_jvm_CallNonvirtualIntMethodV, (jobject, jclass, jmethodID) -> jint);
declare_call!(rusty_jvm_CallStaticIntMethod, rusty_jvm_CallStaticIntMethodV, (jclass, jmethodID) -> jint);
declare_call!(rusty_jvm_CallLongMethod, rusty_jvm_CallLongMethodV, (jobject, jmethodID) -> jlong);
declare_call!(rusty_jvm_CallNonvirtualLongMethod, rusty_jvm_CallNonvirtualLongMethodV, (jobject, jclass, jmethodID) -> jlong);
declare_call!(rusty_jvm_CallStaticLongMethod, rusty_jvm_CallStaticLongMethodV, (jclass, jmethodID) -> jlong);
declare_call!(rusty_jvm_CallFloatMethod, rusty_jvm_CallFloatMethodV, (jobject, jmethodID) -> jfloat);
declare_call!(rusty_jvm_CallNonvirtualFloatMethod, rusty_jvm_CallNonvirtualFloatMethodV, (jobject, jclass, jmethodID) -> jfloat);
declare_call!(rusty_jvm_CallStaticFloatMethod, rusty_jvm_CallStaticFloatMethodV, (jclass, jmethodID) -> jfloat);
declare_call!(rusty_jvm_CallDoubleMethod, rusty_jvm_CallDoubleMethodV, (jobject, jmethodID) -> jdouble);
declare_call!(rusty_jvm_CallNonvirtualDoubleMethod, rusty_jvm_CallNonvirtualDoubleMethodV, (jobject, jclass, jmethodID) -> jdouble);
declare_call!(rusty_jvm_CallStaticDoubleMethod, rusty_jvm_CallStaticDoubleMethodV, (jclass, jmethodID) -> jdouble);
declare_call!(rusty_jvm_CallVoidMethod, rusty_jvm_CallVoidMethodV, (jobject, jmethodID) -> ());
declare_call!(rusty_jvm_CallNonvirtualVoidMethod, rusty_jvm_CallNonvirtualVoidMethodV, (jobject, jclass, jmethodID) -> ());
declare_call!(rusty_jvm_CallStaticVoidMethod, rusty_jvm_CallStaticVoidMethodV, (jclass, jmethodID) -> ());
