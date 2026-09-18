use crate::from_mutf8_ptr;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::helper::{clazz_ref, klass};
use crate::vm::jni::jni_value::JNIValue;
use crate::vm::jni::utils::set_pending_no_such_field_error;
use crate::vm::method_area::lookup::lookup_for_static_field_by_descriptor;
use jdescriptor::TypeDescriptor;
use jni_sys::{
    jboolean, jbyte, jchar, jclass, jdouble, jfieldID, jfloat, jint, jlong, jobject, jshort,
    JNIEnv,
};
use std::ffi::c_char;
use std::ptr::null_mut;

pub(super) extern "system" fn get_static_field_id(
    _env: *mut JNIEnv,
    clazz: jclass,
    name_mutf8: *const c_char,
    sig_mutf8: *const c_char,
) -> jfieldID {
    let name_str = from_mutf8_ptr!(name_mutf8).expect("Failed to convert field name from CESU-8");
    let sig_str =
        from_mutf8_ptr!(sig_mutf8).expect("Failed to convert field signature from CESU-8");
    let requested_class = klass(clazz as i32).expect("Failed to get class from reference");
    StaticInit::initialize_java_class(&requested_class)
        .expect("Failed to initialize class before getting static field ID");
    let Ok(descriptor) = sig_str.parse::<TypeDescriptor>() else {
        set_pending_no_such_field_error(&name_str);
        return null_mut();
    };
    let field = lookup_for_static_field_by_descriptor(
        requested_class.this_class_name(),
        &name_str,
        &descriptor,
    )
    .expect("Failed to resolve static field");

    if let Some((declaring_class_name, _)) = field {
        let declaring_class_ref =
            clazz_ref(&declaring_class_name).expect("Failed to get declaring class reference");
        let declaring_class =
            klass(declaring_class_ref).expect("Failed to get declaring class from reference");
        let offset = declaring_class
            .get_static_field_offset(&name_str)
            .expect("Failed to get static field offset");
        encode_field_id(declaring_class_ref, offset)
    } else {
        set_pending_no_such_field_error(&name_str);
        null_mut()
    }
}

macro_rules! get_static_field_impl {
    ($name:ident, $jni_ty:ty) => {
        pub(super) extern "system" fn $name(
            env: *mut JNIEnv,
            clazz: jclass,
            field_id: jfieldID,
        ) -> $jni_ty {
            get_static_field::<$jni_ty>(env, clazz, field_id)
        }
    };
}
get_static_field_impl!(get_static_object_field, jobject);
get_static_field_impl!(get_static_boolean_field, jboolean);
get_static_field_impl!(get_static_byte_field, jbyte);
get_static_field_impl!(get_static_char_field, jchar);
get_static_field_impl!(get_static_short_field, jshort);
get_static_field_impl!(get_static_int_field, jint);
get_static_field_impl!(get_static_long_field, jlong);
get_static_field_impl!(get_static_float_field, jfloat);
get_static_field_impl!(get_static_double_field, jdouble);
pub(super) extern "system" fn get_static_field<T: JNIValue>(
    _env: *mut JNIEnv,
    _clazz: jclass,
    field_id: jfieldID,
) -> T {
    let (declaring_class_ref, offset) = decode_field_id(field_id);
    let klass =
        klass(declaring_class_ref).expect("Failed to get declaring class from static jfieldID");
    let raw = klass
        .get_static_field_by_offset(offset)
        .expect("Failed to get static field by offset")
        .raw_value()
        .expect("Failed to get raw value of static field");

    JNIValue::from_vec(&raw)
}

macro_rules! set_static_field_impl {
    ($name:ident, $jni_ty:ty) => {
        pub(super) extern "system" fn $name(
            env: *mut JNIEnv,
            clazz: jclass,
            field_id: jfieldID,
            value: $jni_ty,
        ) {
            set_static_field::<$jni_ty>(env, clazz, field_id, value)
        }
    };
}
set_static_field_impl!(set_static_object_field, jobject);
set_static_field_impl!(set_static_boolean_field, jboolean);
set_static_field_impl!(set_static_byte_field, jbyte);
set_static_field_impl!(set_static_char_field, jchar);
set_static_field_impl!(set_static_short_field, jshort);
set_static_field_impl!(set_static_int_field, jint);
set_static_field_impl!(set_static_long_field, jlong);
set_static_field_impl!(set_static_float_field, jfloat);
set_static_field_impl!(set_static_double_field, jdouble);
pub(super) extern "system" fn set_static_field<T: JNIValue>(
    _env: *mut JNIEnv,
    _clazz: jclass,
    field_id: jfieldID,
    value: T,
) {
    let (declaring_class_ref, offset) = decode_field_id(field_id);
    let klass =
        klass(declaring_class_ref).expect("Failed to get declaring class from static jfieldID");
    let field = klass
        .get_static_field_by_offset(offset)
        .expect("Failed to get static field by offset");
    let raw_value = value.to_vec();
    field
        .set_raw_value(raw_value)
        .expect("Failed to set raw value of static field")
}

fn encode_field_id(class_ref: i32, offset: i64) -> jfieldID {
    let class_bits = (class_ref as u32 as u64) << 32;
    let offset_bits = offset as u32 as u64;
    (class_bits | offset_bits) as usize as jfieldID
}

fn decode_field_id(field_id: jfieldID) -> (i32, i64) {
    let raw = field_id as usize as u64;
    ((raw >> 32) as u32 as i32, (raw & 0xFFFF_FFFF) as i64)
}
