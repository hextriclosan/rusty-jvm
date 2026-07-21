use crate::vm::error::Result;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::clazz_ref;
use murmur3::murmur3_32;
use std::io::Cursor;
use std::time::Duration;

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

/// `java.lang.Object.notifyAll()V`
pub(crate) fn notify_all(_obj_ref: i32) -> Result<()> {
    // todo: implement me
    Ok(())
}

/// `java.lang.Object.wait0(J)V`
///
/// Placeholder until real object monitors land (Phase 3): it blocks the calling thread rather than
/// participating in wait/notify. A zero (indefinite) timeout parks forever — correct for the
/// GC-less system daemons (Finalizer/Cleaner) that wait for work that never arrives — while a
/// positive timeout parks for that long and returns. `notify`/`notifyAll` do not yet wake it.
pub(crate) fn wait0(_obj_ref: i32, timeout_millis: i64) -> Result<()> {
    if timeout_millis == 0 {
        loop {
            std::thread::park();
        }
    }
    std::thread::park_timeout(Duration::from_millis(timeout_millis as u64));
    Ok(())
}
