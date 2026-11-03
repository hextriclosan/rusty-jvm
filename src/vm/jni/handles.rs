//! JNI tokens are never dereferenced, tagged, recycled, or used as heap keys.
use crate::vm::error::{Error, Result};
use crate::vm::heap::heap::HEAP;
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::jni::utils::set_pending_internal_error;
use crate::vm::method_area::java_class::JavaClass;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::stack::rooted_slots::RootedSlots;
use jni_sys::{jobject, jobjectRefType};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex, MutexGuard, OnceLock};
use std::thread::{self, ThreadId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ReferenceKind {
    Local,
    Global,
    WeakGlobal,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct HandleRecord {
    pub(crate) kind: ReferenceKind,
    pub(crate) target: i32,
    pub(crate) owner_thread: Option<ThreadId>,
    pub(crate) frame: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum HandleError {
    OutOfMemory,
    InvalidReference,
    MissingFrame,
}

impl HandleError {
    pub(super) fn set_pending(self) {
        match self {
            Self::OutOfMemory => {
                // Preallocated before native dispatch is enabled. No allocation or Java reentry.
                if let Some(&throwable) = EMERGENCY_OOM.get() {
                    JavaThread::set_pending_if_empty(throwable);
                }
            }
            Self::InvalidReference => set_pending_internal_error("Invalid JNI reference"),
            Self::MissingFrame => set_pending_internal_error("JNI local frame is missing"),
        }
    }
}

pub(super) type HandleResult<T> = std::result::Result<T, HandleError>;

pub(super) struct HandleTable {
    next: Option<usize>,
    pub(super) records: HashMap<usize, HandleRecord>,
}

impl HandleTable {
    pub(super) fn new() -> Self {
        Self {
            next: Some(1),
            records: HashMap::new(),
        }
    }

    pub(super) fn reserve(&mut self, count: usize) -> HandleResult<()> {
        if let Some(additional) = count.checked_sub(1) {
            self.next
                .and_then(|next| next.checked_add(additional))
                .ok_or(HandleError::OutOfMemory)?;
        }
        self.records
            .try_reserve(count)
            .map_err(|_| HandleError::OutOfMemory)
    }

    pub(super) fn insert(&mut self, record: HandleRecord) -> HandleResult<usize> {
        if record.target <= 0 {
            return Err(HandleError::InvalidReference);
        }
        self.reserve(1)?;
        let token = self.next.ok_or(HandleError::OutOfMemory)?;
        if token == 0 || self.records.contains_key(&token) {
            return Err(HandleError::OutOfMemory);
        }
        self.next = token.checked_add(1);
        self.records.insert(token, record);
        Ok(token)
    }

    pub(super) fn record(&self, token: usize) -> Option<&HandleRecord> {
        self.records.get(&token).filter(|record| {
            record.owner_thread.is_none() || record.owner_thread == Some(thread::current().id())
        })
    }

    pub(super) fn resolve(&self, token: usize) -> HandleResult<i32> {
        if token == 0 {
            return Ok(0);
        }
        self.record(token)
            .map(|record| record.target)
            .ok_or(HandleError::InvalidReference)
    }
}

static TABLE: LazyLock<Mutex<HandleTable>> = LazyLock::new(|| Mutex::new(HandleTable::new()));
static EMERGENCY_OOM: OnceLock<i32> = OnceLock::new();
static EMERGENCY_OOM_CLASSES: OnceLock<Vec<Arc<JavaClass>>> = OnceLock::new();

pub(crate) fn emergency_oom_ref() -> Option<i32> {
    EMERGENCY_OOM.get().copied()
}

pub(crate) fn install_emergency_oom() {
    if let Some(reference) = emergency_oom_ref() {
        JavaThread::set_pending_if_empty(reference);
    }
}

/// Prepared at bootstrap: catch matching must not load classes or allocate on OOM.
pub(crate) fn emergency_oom_is_instance(class_name: &str) -> bool {
    EMERGENCY_OOM_CLASSES.get().is_some_and(|classes| {
        classes
            .iter()
            .any(|class| class.this_class_name() == class_name)
    })
}

pub(super) fn table() -> MutexGuard<'static, HandleTable> {
    TABLE
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Bootstrap without Java constructors, native calls, or token allocation.
pub(crate) fn initialize() -> Result<()> {
    let mut classes = Vec::new();
    let mut class = CLASSES.get("java/lang/OutOfMemoryError")?;
    loop {
        classes.push(Arc::clone(&class));
        match class.parent().as_deref() {
            Some(parent) => class = CLASSES.get(parent)?,
            None => break,
        }
    }
    EMERGENCY_OOM_CLASSES
        .set(classes)
        .map_err(|_| Error::new_execution("Emergency OOM metadata already initialized"))?;
    let instance = with_method_area(|area| {
        area.create_instance_with_default_fields("java/lang/OutOfMemoryError")
    })?;
    let mut destination = RootedSlots::new();
    let target = HEAP.create_instance(instance, &mut destination)?;
    HEAP.finish_emergency_bootstrap().map_err(|_| {
        Error::new_execution(
            "Configured managed heap is too small for emergency OutOfMemoryError bootstrap",
        )
    })?;
    EMERGENCY_OOM.set(target).map_err(|_| {
        crate::vm::error::Error::new_execution("JNI emergency exception already initialized")
    })?;
    crate::vm::gc::trigger::enable();
    Ok(())
}

pub(super) fn as_object(token: usize) -> jobject {
    // The address is only an opaque integer key; no allocation has this provenance.
    token as jobject
}

pub(crate) fn resolve(object: jobject) -> Option<i32> {
    let result = table().resolve(object as usize);
    match result {
        Ok(target) => Some(target),
        Err(error) => {
            error.set_pending();
            None
        }
    }
}

pub(crate) fn new_local(target: i32) -> jobject {
    if target == 0 {
        return std::ptr::null_mut();
    }
    finish(JavaThread::with_jni_frames(|frames| {
        frames.new_local(target, &mut table())
    }))
}

pub(super) fn finish(result: HandleResult<usize>) -> jobject {
    match result {
        Ok(token) => as_object(token),
        Err(error) => {
            error.set_pending();
            std::ptr::null_mut()
        }
    }
}

pub(super) fn new_reference(object: jobject, kind: ReferenceKind) -> jobject {
    // Resolve and publish under the same lock, with no Java call or safepoint in between.
    // TODO(GC): the coordinator's mutator permit will enclose this operation in task 4.
    let result = JavaThread::with_jni_frames(|frames| {
        let mut table = table();
        let target = table.resolve(object as usize)?;
        if target == 0 {
            return Ok(0);
        }
        match kind {
            ReferenceKind::Local => frames.new_local(target, &mut table),
            ReferenceKind::Global | ReferenceKind::WeakGlobal => table.insert(HandleRecord {
                kind,
                target,
                owner_thread: None,
                frame: None,
            }),
        }
    });
    finish(result)
}

pub(super) fn delete_reference(object: jobject, kind: ReferenceKind) {
    if object.is_null() {
        return;
    }
    JavaThread::with_jni_frames(|frames| {
        let mut table = table();
        if let Some(record) = table.record(object as usize).copied() {
            if record.kind == kind {
                table.records.remove(&(object as usize));
                if let Some(frame) = record.frame {
                    frames.delete_local(frame, object as usize);
                }
            }
        }
    });
}

pub(super) fn reference_type(object: jobject) -> jobjectRefType {
    match table().record(object as usize).map(|record| record.kind) {
        Some(ReferenceKind::Local) => jobjectRefType::JNILocalRefType,
        Some(ReferenceKind::Global) => jobjectRefType::JNIGlobalRefType,
        Some(ReferenceKind::WeakGlobal) => jobjectRefType::JNIWeakGlobalRefType,
        None => jobjectRefType::JNIInvalidRefType,
    }
}

/// Only the collector, with all mutators stopped, may supply this liveness predicate.
pub(crate) fn clear_weak_references(mut is_live: impl FnMut(i32) -> bool) {
    for record in table().records.values_mut() {
        if record.kind == ReferenceKind::WeakGlobal
            && record.target != 0
            && !is_live(record.target)
        {
            record.target = 0;
        }
    }
}

pub(crate) fn global_roots() -> Result<Vec<i32>> {
    let table = TABLE.try_lock().map_err(|error| {
        Error::new_execution(&format!("Cannot snapshot JNI global handles: {error}"))
    })?;
    let mut roots: Vec<_> = table
        .records
        .values()
        .filter(|record| record.kind == ReferenceKind::Global)
        .map(|record| record.target)
        .collect();
    roots.extend(EMERGENCY_OOM.get().copied());
    Ok(roots)
}

pub(crate) fn local_roots() -> Vec<i32> {
    let owner = thread::current().id();
    table()
        .records
        .values()
        .filter(|record| record.owner_thread == Some(owner))
        .map(|record| record.target)
        .collect()
}

#[cfg(test)]
pub(crate) fn new_wide_test_local(target: i32) -> jobject {
    finish(JavaThread::with_jni_frames(|frames| {
        let mut table = table();
        let wide = usize::try_from(u64::from(u32::MAX) + 1).unwrap_or(1);
        // Advance only: even tests may not reuse a token already handed out.
        table.next = table.next.map(|next| next.max(wide));
        frames.new_local(target, &mut table)
    }))
}

macro_rules! resolve_or_return {
    ($object:expr) => {
        $crate::vm::jni::handles::resolve_or_return!($object, Default::default())
    };
    ($object:expr, $sentinel:expr) => {
        match $crate::vm::jni::handles::resolve($object) {
            Some(target) => target,
            None => return $sentinel,
        }
    };
}
pub(crate) use resolve_or_return;

#[cfg(test)]
mod tests;
