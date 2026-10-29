use jni::sys::{jbyte, jclass, jlong, jobject, JNIEnv};
use std::ptr::NonNull;

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nio_JniDirectBufferDemo_create(
    env: *mut JNIEnv,
    _class: jclass,
    capacity: jlong,
) -> jobject {
    let memory = vec![0u8; capacity as usize].into_boxed_slice();
    let address = Box::into_raw(memory).cast::<u8>();
    unsafe { ((*(*env)).v24.NewDirectByteBuffer)(env, address.cast(), capacity) }
}

/// Wraps a dangling pointer, for capacities that are never accessed.
#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nio_JniDirectBufferDemo_createUnbacked(
    env: *mut JNIEnv,
    _class: jclass,
    capacity: jlong,
) -> jobject {
    let address = NonNull::<u8>::dangling().as_ptr();
    unsafe { ((*(*env)).v24.NewDirectByteBuffer)(env, address.cast(), capacity) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nio_JniDirectBufferDemo_address(
    env: *mut JNIEnv,
    _class: jclass,
    buffer: jobject,
) -> jlong {
    unsafe { ((*(*env)).v24.GetDirectBufferAddress)(env, buffer) as usize as jlong }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nio_JniDirectBufferDemo_capacity(
    env: *mut JNIEnv,
    _class: jclass,
    buffer: jobject,
) -> jlong {
    unsafe { ((*(*env)).v24.GetDirectBufferCapacity)(env, buffer) }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nio_JniDirectBufferDemo_fill(
    env: *mut JNIEnv,
    _class: jclass,
    buffer: jobject,
    value: jbyte,
) {
    unsafe {
        let address = ((*(*env)).v24.GetDirectBufferAddress)(env, buffer).cast::<u8>();
        let capacity = ((*(*env)).v24.GetDirectBufferCapacity)(env, buffer) as usize;
        std::ptr::write_bytes(address, value as u8, capacity);
    }
}

#[no_mangle]
pub extern "system" fn Java_samples_javacore_loadlibrary_nio_JniDirectBufferDemo_release(
    env: *mut JNIEnv,
    _class: jclass,
    buffer: jobject,
) {
    unsafe {
        let address = ((*(*env)).v24.GetDirectBufferAddress)(env, buffer).cast::<u8>();
        let capacity = ((*(*env)).v24.GetDirectBufferCapacity)(env, buffer) as usize;
        drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
            address, capacity,
        )));
    }
}
