use crate::vm::heap::heap::HEAP;
use crate::vm::helper::klass;
use crate::vm::jni::utils::{
    set_pending_array_index_out_of_bounds_exception,
    set_pending_array_region_out_of_bounds_exception, set_pending_negative_array_size_exception,
    set_pending_null_pointer_exception,
};
use crate::vm::jni::{buffer_leases, handles};
use crate::vm::stack::rooted_slots::RootedSlots;
use jni_sys::{
    jarray, jboolean, jbooleanArray, jbyte, jbyteArray, jchar, jcharArray, jclass, jdouble,
    jdoubleArray, jfloat, jfloatArray, jint, jintArray, jlong, jlongArray, jobject, jobjectArray,
    jshort, jshortArray, jsize, JNIEnv, JNI_TRUE,
};
use std::ffi::c_void;

#[cfg(test)]
mod budget_tests {
    use super::*;
    use crate::vm::gc::test_allocations::bootstrap_oom;
    use crate::vm::jni::{java_thread::JavaThread, local_frames::NativeFrame};
    use crate::vm::method_area::loaded_classes::CLASSES;

    #[test]
    fn gc_budget_jni_arrays_preserve_oom_and_filled_array_contents() {
        let emergency = bootstrap_oom();
        let _native = NativeFrame::enter().unwrap();
        let component = CLASSES
            .get("java/lang/Object")
            .unwrap()
            .mirror_clazz_ref()
            .unwrap();
        let class = handles::new_local(component);
        let failed = new_object_array(std::ptr::null_mut(), i32::MAX, class, class);
        assert!(failed.is_null());
        assert_eq!(JavaThread::take_pending_exception(), Some(emergency));
        assert!(new_primitive_type_array_impl(i32::MAX, "[J").is_null());
        assert_eq!(JavaThread::take_pending_exception(), Some(emergency));
        let filled = new_object_array(std::ptr::null_mut(), 3, class, class);
        let reference = handles::resolve(filled).unwrap();
        assert_ne!(reference, 0);
        for index in 0..3 {
            assert_eq!(
                HEAP.get_array_value(reference, index).unwrap(),
                vec![component]
            );
        }
        assert_eq!(JavaThread::get_pending_exception(), None);
    }
}

pub(super) extern "system" fn get_array_length(_env: *mut JNIEnv, input: jarray) -> jint {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    let array_ref = handles::resolve_or_return!(input);
    if array_ref == 0 {
        set_pending_null_pointer_exception("Array is null");
        return 0;
    }

    HEAP.get_array_len(array_ref).unwrap_or(0) as jint // OpenJDK returns 0 for non-arrays
}

pub(super) extern "system" fn new_object_array(
    _env: *mut JNIEnv,
    len: jsize,
    clazz: jclass,
    init: jobject,
) -> jobjectArray {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    let clazz_ref = handles::resolve_or_return!(clazz);
    let init_obj_ref = handles::resolve_or_return!(init);

    if clazz_ref == 0 {
        set_pending_null_pointer_exception("Element class is null");
        return std::ptr::null_mut();
    }

    if len < 0 {
        set_pending_negative_array_size_exception(len);
        return std::ptr::null_mut();
    }

    if HEAP
        .get_instance_name(clazz_ref)
        .expect("Invalid class reference")
        != "java/lang/Class"
    {
        panic!("Class reference is not a Class"); // OpenJDK crashes here, why we shouldn't
    }

    let klass = klass(clazz_ref).expect("Failed to get class from reference");
    let class_name = klass.this_class_name();
    let type_name = if class_name.starts_with("[") {
        format!("[{class_name}")
    } else {
        format!("[L{};", class_name)
    };

    let mut destination = RootedSlots::new();
    let result = HEAP
        .create_array(&type_name, len, &mut destination)
        .and_then(|reference| {
            if init_obj_ref != 0 {
                // Fill admitted storage, rather than building an unadmitted len-sized Vec.
                let mut bytes = HEAP.get_entire_raw_data_mut(reference)?;
                for slot in bytes.chunks_exact_mut(4) {
                    slot.copy_from_slice(&init_obj_ref.to_ne_bytes());
                }
            }
            Ok(reference)
        });
    match result {
        Ok(reference) => handles::new_local(reference),
        Err(error) => {
            if let Some(throwable) = error.throwable_ref() {
                crate::vm::jni::java_thread::JavaThread::set_pending_if_empty(throwable);
            } else {
                handles::install_emergency_oom();
            }
            std::ptr::null_mut()
        }
    }
}

pub(super) extern "system" fn get_object_array_element(
    env: *mut JNIEnv,
    array: jobjectArray,
    index: jsize,
) -> jobject {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    let array_ref = handles::resolve_or_return!(array);
    if array_ref == 0 {
        set_pending_null_pointer_exception("Array is null");
        return std::ptr::null_mut();
    }

    let len = get_array_length(env, array);
    if index >= len || index < 0 {
        set_pending_array_index_out_of_bounds_exception(index, len);
        return std::ptr::null_mut();
    }

    let raw = HEAP
        .get_array_value(array_ref, index)
        .expect("Failed to get array element");
    handles::new_local(raw[0])
}

pub(super) extern "system" fn set_object_array_element(
    env: *mut JNIEnv,
    array: jobjectArray,
    index: jsize,
    value: jobject,
) {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    let array_ref = handles::resolve_or_return!(array);
    if array_ref == 0 {
        set_pending_null_pointer_exception("Array is null");
        return;
    }

    let len = get_array_length(env, array);
    if index >= len || index < 0 {
        set_pending_array_index_out_of_bounds_exception(index, len);
        return;
    }

    HEAP.set_array_value(array_ref, index, vec![handles::resolve_or_return!(value)])
        .expect("Failed to set array element");
}

pub(super) extern "system" fn new_boolean_array(_env: *mut JNIEnv, len: jsize) -> jbooleanArray {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    new_primitive_type_array_impl(len, "[Z")
}

pub(super) extern "system" fn new_byte_array(_env: *mut JNIEnv, len: jsize) -> jbyteArray {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    new_primitive_type_array_impl(len, "[B")
}

pub(super) extern "system" fn new_char_array(_env: *mut JNIEnv, len: jsize) -> jcharArray {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    new_primitive_type_array_impl(len, "[C")
}

pub(super) extern "system" fn new_short_array(_env: *mut JNIEnv, len: jsize) -> jshortArray {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    new_primitive_type_array_impl(len, "[S")
}

pub(super) extern "system" fn new_int_array(_env: *mut JNIEnv, len: jsize) -> jintArray {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    new_primitive_type_array_impl(len, "[I")
}

pub(super) extern "system" fn new_long_array(_env: *mut JNIEnv, len: jsize) -> jlongArray {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    new_primitive_type_array_impl(len, "[J")
}

pub(super) extern "system" fn new_float_array(_env: *mut JNIEnv, len: jsize) -> jfloatArray {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    new_primitive_type_array_impl(len, "[F")
}

pub(super) extern "system" fn new_double_array(_env: *mut JNIEnv, len: jsize) -> jdoubleArray {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    new_primitive_type_array_impl(len, "[D")
}

fn new_primitive_type_array_impl(len: jsize, type_name: &str) -> jarray {
    if len < 0 {
        set_pending_negative_array_size_exception(len);
        return std::ptr::null_mut();
    }

    let mut destination = RootedSlots::new();
    match HEAP.create_array(type_name, len, &mut destination) {
        Ok(reference) => handles::new_local(reference),
        Err(_) => {
            handles::install_emergency_oom();
            std::ptr::null_mut()
        }
    }
}

macro_rules! impl_get_array {
    ($name:ident, $jni_ty:ty, $array_ty:ty) => {
        pub(super) extern "system" fn $name(
            _env: *mut JNIEnv,
            array: $array_ty,
            is_copy: *mut jboolean,
        ) -> *mut $jni_ty {
            let _managed = crate::vm::safepoint::ManagedAccess::enter();
            get_primitive_type_array_elements::<$jni_ty>(
                handles::resolve_or_return!(array),
                is_copy,
            )
        }
    };
}
impl_get_array!(get_boolean_array_elements, jboolean, jbooleanArray);
impl_get_array!(get_byte_array_elements, jbyte, jbyteArray);
impl_get_array!(get_char_array_elements, jchar, jcharArray);
impl_get_array!(get_short_array_elements, jshort, jshortArray);
impl_get_array!(get_int_array_elements, jint, jintArray);
impl_get_array!(get_long_array_elements, jlong, jlongArray);
impl_get_array!(get_float_array_elements, jfloat, jfloatArray);
impl_get_array!(get_double_array_elements, jdouble, jdoubleArray);
fn get_primitive_type_array_elements<T: Copy>(array_ref: i32, is_copy: *mut jboolean) -> *mut T {
    if array_ref == 0 {
        set_pending_null_pointer_exception("Array is null");
        return std::ptr::null_mut();
    }

    let raw_data = HEAP
        .get_entire_raw_data(array_ref)
        .expect("Failed to get array elements")
        .value()
        .clone();
    // Critical access erases T to u8, but must still align for any primitive element.
    let raw_ptr =
        buffer_leases::copy(array_ref, &raw_data, align_of::<u64>().max(align_of::<T>())).cast();

    if !is_copy.is_null() {
        unsafe {
            *is_copy = JNI_TRUE; // we always return a copy
        }
    }

    raw_ptr
}

macro_rules! impl_release_array {
    ($name:ident, $jni_ty:ty, $array_ty:ty) => {
        pub(super) extern "system" fn $name(
            env: *mut JNIEnv,
            array: $array_ty,
            elems: *mut $jni_ty,
            mode: jint,
        ) {
            let _managed = crate::vm::safepoint::ManagedAccess::enter();
            release_primitive_type_array_elements::<$jni_ty>(env, array, elems, mode)
        }
    };
}
impl_release_array!(release_boolean_array_elements, jboolean, jbooleanArray);
impl_release_array!(release_byte_array_elements, jbyte, jbyteArray);
impl_release_array!(release_char_array_elements, jchar, jcharArray);
impl_release_array!(release_short_array_elements, jshort, jshortArray);
impl_release_array!(release_int_array_elements, jint, jintArray);
impl_release_array!(release_long_array_elements, jlong, jlongArray);
impl_release_array!(release_float_array_elements, jfloat, jfloatArray);
impl_release_array!(release_double_array_elements, jdouble, jdoubleArray);
fn release_primitive_type_array_elements<T>(
    _env: *mut JNIEnv,
    array: jarray,
    elems: *mut T,
    mode: jint,
) {
    let array_ref = handles::resolve_or_return!(array);
    buffer_leases::release(elems.cast(), array_ref, Some(mode));
}

pub(super) extern "system" fn get_primitive_array_critical(
    _env: *mut JNIEnv,
    array: jarray,
    is_copy: *mut jboolean,
) -> *mut c_void {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    // Copy-based even for critical access: the lease roots the target, without disabling GC.
    get_primitive_type_array_elements::<u8>(handles::resolve_or_return!(array), is_copy).cast()
}

pub(super) extern "system" fn release_primitive_array_critical(
    _env: *mut JNIEnv,
    array: jarray,
    elements: *mut c_void,
    mode: jint,
) {
    let _managed = crate::vm::safepoint::ManagedAccess::enter();
    let array_ref = handles::resolve_or_return!(array);
    buffer_leases::release(elements.cast(), array_ref, Some(mode));
}

macro_rules! impl_set_array_region {
    ($name:ident, $jni_ty:ty, $array_ty:ty) => {
        pub(super) extern "system" fn $name(
            env: *mut JNIEnv,
            array: $array_ty,
            start: jsize,
            len: jsize,
            buf: *const $jni_ty,
        ) {
            let _managed = crate::vm::safepoint::ManagedAccess::enter();
            set_primitive_type_array_region::<$jni_ty>(env, array, start, len, buf)
        }
    };
}
impl_set_array_region!(set_boolean_array_region, jboolean, jbooleanArray);
impl_set_array_region!(set_byte_array_region, jbyte, jbyteArray);
impl_set_array_region!(set_char_array_region, jchar, jcharArray);
impl_set_array_region!(set_short_array_region, jshort, jshortArray);
impl_set_array_region!(set_int_array_region, jint, jintArray);
impl_set_array_region!(set_long_array_region, jlong, jlongArray);
impl_set_array_region!(set_float_array_region, jfloat, jfloatArray);
impl_set_array_region!(set_double_array_region, jdouble, jdoubleArray);
fn set_primitive_type_array_region<T>(
    _env: *mut JNIEnv,
    array: jarray,
    start: jsize,
    len: jsize,
    buf: *const T,
) {
    let array_ref = handles::resolve_or_return!(array);
    if !validate_array_region(array_ref, start, len) {
        return;
    }

    if len == 0 {
        return;
    }
    let byte_buf = buf as *const u8;
    if byte_buf.is_null() {
        set_pending_null_pointer_exception("Buffer is null");
        return;
    }
    let start_in_bytes = start as usize * size_of::<T>();
    let len_in_bytes = len as usize * size_of::<T>();
    write_to_array(array_ref, byte_buf, start_in_bytes, len_in_bytes);
}

macro_rules! impl_get_array_region {
    ($name:ident, $jni_ty:ty, $array_ty:ty) => {
        pub(super) extern "system" fn $name(
            env: *mut JNIEnv,
            array: $array_ty,
            start: jsize,
            len: jsize,
            buf: *mut $jni_ty,
        ) {
            let _managed = crate::vm::safepoint::ManagedAccess::enter();
            get_primitive_type_array_region::<$jni_ty>(env, array, start, len, buf)
        }
    };
}
impl_get_array_region!(get_boolean_array_region, jboolean, jbooleanArray);
impl_get_array_region!(get_byte_array_region, jbyte, jbyteArray);
impl_get_array_region!(get_char_array_region, jchar, jcharArray);
impl_get_array_region!(get_short_array_region, jshort, jshortArray);
impl_get_array_region!(get_int_array_region, jint, jintArray);
impl_get_array_region!(get_long_array_region, jlong, jlongArray);
impl_get_array_region!(get_float_array_region, jfloat, jfloatArray);
impl_get_array_region!(get_double_array_region, jdouble, jdoubleArray);
fn get_primitive_type_array_region<T>(
    _env: *mut JNIEnv,
    array: jarray,
    start: jsize,
    len: jsize,
    buf: *mut T,
) {
    let array_ref = handles::resolve_or_return!(array);
    if !validate_array_region(array_ref, start, len) {
        return;
    }

    if len == 0 {
        return;
    }
    let byte_buf = buf as *mut u8;
    if byte_buf.is_null() {
        set_pending_null_pointer_exception("Buffer is null");
        return;
    }
    let start_in_bytes = start as usize * size_of::<T>();
    let len_in_bytes = len as usize * size_of::<T>();
    read_from_array(array_ref, byte_buf, start_in_bytes, len_in_bytes);
}

fn validate_array_region(array_ref: i32, start: jsize, len: jsize) -> bool {
    if array_ref == 0 {
        set_pending_null_pointer_exception("Array is null");
        return false;
    }
    let array_len = HEAP
        .get_array_len(array_ref)
        .expect("Failed to get array length");
    if len < 0 || start < 0 || start > array_len - len {
        set_pending_array_region_out_of_bounds_exception(start, len, array_len);
        return false;
    }
    true
}

fn write_to_array(array_ref: i32, elems: *const u8, start: usize, len: usize) {
    let slice = unsafe { std::slice::from_raw_parts(elems, len) };
    let mut guard = HEAP
        .get_entire_raw_data_mut(array_ref)
        .expect("Failed to commit array elements");
    let _ = &guard[start..(start + len)].copy_from_slice(slice);
}

fn read_from_array(array_ref: i32, elems: *mut u8, start: usize, len: usize) {
    let guard = HEAP
        .get_entire_raw_data(array_ref)
        .expect("Failed to read array elements");
    let slice = &guard[start..(start + len)];
    unsafe {
        std::ptr::copy_nonoverlapping(slice.as_ptr(), elems, len);
    }
}
