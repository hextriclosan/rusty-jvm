use crate::vm::error::Result;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::clazz_ref;
use crate::vm::monitor;
use murmur3::murmur3_32;
use std::io::Cursor;

/// `java.lang.Object.getClass()Ljava/lang/Class;`
pub(crate) fn get_class(obj_ref: i32) -> Result<i32> {
    let instance_name = HEAP.get_instance_name(obj_ref)?;

    let reflection_ref = clazz_ref(&instance_name)?;

    Ok(reflection_ref)
}

/// `java.lang.Object.clone()Ljava/lang/Object;`
pub(crate) fn clone(obj_ref: i32) -> Result<i32> {
    let cloned_obj_ref = HEAP.clone_instance(obj_ref)?;

    Ok(cloned_obj_ref)
}

/// `java.lang.Object.hashCode()I`
pub(crate) fn identity_hashcode(obj_ref: i32) -> Result<i32> {
    if obj_ref == 0 {
        return Ok(0);
    }

    let mut cursor = Cursor::new(obj_ref.to_le_bytes());
    let hashcode = murmur3_32(&mut cursor, 0)?;
    Ok(hashcode as i32)
}

/// `java.lang.Object.notify()V`
pub(crate) fn notify(obj_ref: i32) -> Result<()> {
    monitor::notify(obj_ref)
}

/// `java.lang.Object.notifyAll()V`
pub(crate) fn notify_all(obj_ref: i32) -> Result<()> {
    monitor::notify_all(obj_ref)
}

/// `java.lang.Object.wait0(J)V`
///
/// Releases the object's monitor, blocks until notified or the timeout elapses, then re-acquires —
/// the primitive behind `Object.wait`. `timeout_millis == 0` waits indefinitely.
pub(crate) fn wait0(obj_ref: i32, timeout_millis: i64) -> Result<()> {
    monitor::wait(obj_ref, timeout_millis)
}
