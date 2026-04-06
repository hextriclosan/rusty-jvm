use crate::from_mutf8_ptr;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::helper::klass;
use crate::vm::jni::jni_value::JNIValue;
use jni_sys::{
    jboolean, jbyte, jchar, jclass, jdouble, jfieldID, jfloat, jint, jlong, jobject, jshort,
    JNIEnv,
};
use std::ffi::c_char;

pub(super) extern "system" fn get_static_field_id(
    _env: *mut JNIEnv,
    clazz: jclass,
    name_mutf8: *const c_char,
    _sig_mutf8: *const c_char,
) -> jfieldID {
    let name_str = from_mutf8_ptr!(name_mutf8).expect("Failed to convert field name from CESU-8");
    let klass = klass(clazz as i32).expect("Failed to get class from reference");
    StaticInit::initialize_java_class(&klass)
        .expect("Failed to initialize class before getting static field ID");
    let static_field_offset = klass
        .get_static_field_offset(&name_str)
        .expect("Failed to get static field offset by name");
    static_field_offset as jfieldID
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
    clazz: jclass,
    field_id: jfieldID,
) -> T {
    let klass = klass(clazz as i32).expect("Failed to get class from reference");
    let raw = klass
        .get_static_field_by_offset(field_id as i64)
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
    clazz: jclass,
    field_id: jfieldID,
    value: T,
) {
    let klass = klass(clazz as i32).expect("Failed to get class from reference");
    let field = klass
        .get_static_field_by_offset(field_id as i64)
        .expect("Failed to get static field by offset");
    let raw_value = value.to_vec();
    field
        .set_raw_value(raw_value)
        .expect("Failed to set raw value of static field")
}
