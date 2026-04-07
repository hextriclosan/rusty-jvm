use crate::from_mutf8_ptr;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::klass;
use crate::vm::jni::jni_value::JNIValue;
use crate::vm::method_area::loaded_classes::CLASSES;
use jni_sys::{
    jboolean, jbyte, jchar, jclass, jdouble, jfieldID, jfloat, jint, jlong, jobject, jshort,
    JNIEnv,
};
use std::ffi::c_char;

pub(super) extern "system" fn get_field_id(
    _env: *mut JNIEnv,
    clazz: jclass,
    name_mutf8: *const c_char,
    _sig_mutf8: *const c_char,
) -> jfieldID {
    let field_name =
        from_mutf8_ptr!(name_mutf8).expect("Failed to convert field name from CESU-8");
    let klass = klass(clazz as i32).expect("Failed to get class from reference");
    let class_name = klass.this_class_name();
    StaticInit::initialize_java_class(&klass)
        .expect("Failed to initialize class before getting field ID");
    let field_offset = klass
        .get_field_offset(&format!("{class_name}.{field_name}"))
        .expect("Failed to get field offset by name");
    field_offset as jfieldID
}

macro_rules! get_field_impl {
    ($name:ident, $jni_ty:ty) => {
        pub(super) extern "system" fn $name(
            env: *mut JNIEnv,
            obj: jobject,
            field_id: jfieldID,
        ) -> $jni_ty {
            get_field::<$jni_ty>(env, obj, field_id)
        }
    };
}
get_field_impl!(get_object_field, jobject);
get_field_impl!(get_boolean_field, jboolean);
get_field_impl!(get_byte_field, jbyte);
get_field_impl!(get_char_field, jchar);
get_field_impl!(get_short_field, jshort);
get_field_impl!(get_int_field, jint);
get_field_impl!(get_long_field, jlong);
get_field_impl!(get_float_field, jfloat);
get_field_impl!(get_double_field, jdouble);
pub(super) extern "system" fn get_field<T: JNIValue>(
    _env: *mut JNIEnv,
    obj: jobject,
    field_id: jfieldID,
) -> T {
    let instance_name = HEAP
        .get_instance_name(obj as i32)
        .expect("Failed to get instance name from reference");
    let klass = CLASSES
        .get(&instance_name)
        .expect("Failed to get class from instance name");
    let (class_name, field_name) = klass
        .get_field_name_by_offset(field_id as i64)
        .expect("Failed to get field name by offset");
    let raw = HEAP
        .get_object_field_value(obj as i32, &class_name, &field_name)
        .expect("Failed to get object field value");

    JNIValue::from_vec(&raw)
}

macro_rules! set_field_impl {
    ($name:ident, $jni_ty:ty) => {
        pub(super) extern "system" fn $name(
            env: *mut JNIEnv,
            obj: jobject,
            field_id: jfieldID,
            value: $jni_ty,
        ) {
            set_field::<$jni_ty>(env, obj, field_id, value)
        }
    };
}
set_field_impl!(set_object_field, jobject);
set_field_impl!(set_boolean_field, jboolean);
set_field_impl!(set_byte_field, jbyte);
set_field_impl!(set_char_field, jchar);
set_field_impl!(set_short_field, jshort);
set_field_impl!(set_int_field, jint);
set_field_impl!(set_long_field, jlong);
set_field_impl!(set_float_field, jfloat);
set_field_impl!(set_double_field, jdouble);
pub(super) extern "system" fn set_field<T: JNIValue>(
    _env: *mut JNIEnv,
    obj: jobject,
    field_id: jfieldID,
    value: T,
) {
    let instance_name = HEAP
        .get_instance_name(obj as i32)
        .expect("Failed to get instance name from reference");
    let klass = CLASSES
        .get(&instance_name)
        .expect("Failed to get class from instance name");
    let (class_name, field_name) = klass
        .get_field_name_by_offset(field_id as i64)
        .expect("Failed to get field name by offset");

    let raw_value = value.to_vec();

    HEAP.set_object_field_value(obj as i32, &class_name, &field_name, raw_value)
        .expect("Failed to set object field value");
}
