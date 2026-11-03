use crate::vm::jni::handles::{
    self, HandleError, HandleRecord, HandleResult, HandleTable, ReferenceKind,
};
use crate::vm::jni::java_thread::JavaThread;
use jni_sys::jobject;
use std::marker::PhantomData;
use std::rc::Rc;
use std::thread;

const MAX_LOCAL_CAPACITY: usize = 65_536;

struct LocalFrame {
    id: usize,
    native: bool,
    tokens: Vec<usize>,
}

#[derive(Default)]
pub(crate) struct LocalFrames {
    frames: Vec<LocalFrame>,
    next_frame: usize,
}

impl LocalFrames {
    fn reserve(frame: &mut LocalFrame, count: usize, table: &mut HandleTable) -> HandleResult<()> {
        if count > MAX_LOCAL_CAPACITY.saturating_sub(frame.tokens.len()) {
            return Err(HandleError::OutOfMemory);
        }
        table.reserve(count)?;
        frame
            .tokens
            .try_reserve(count)
            .map_err(|_| HandleError::OutOfMemory)
    }

    pub(super) fn ensure_capacity(&mut self, count: usize) -> HandleResult<()> {
        let frame = self.frames.last_mut().ok_or(HandleError::MissingFrame)?;
        Self::reserve(frame, count, &mut handles::table())
    }

    pub(super) fn push(&mut self, capacity: usize, native: bool) -> HandleResult<usize> {
        // All checks and storage reservations precede publication of a new frame.
        if capacity > MAX_LOCAL_CAPACITY {
            return Err(HandleError::OutOfMemory);
        }
        let id = self
            .next_frame
            .checked_add(1)
            .ok_or(HandleError::OutOfMemory)?;
        let mut frame = LocalFrame {
            id,
            native,
            tokens: Vec::new(),
        };
        Self::reserve(&mut frame, capacity, &mut handles::table())?;
        self.frames
            .try_reserve(1)
            .map_err(|_| HandleError::OutOfMemory)?;
        self.frames.push(frame);
        self.next_frame = id;
        Ok(id)
    }

    fn insert(
        frame: &mut LocalFrame,
        target: i32,
        table: &mut HandleTable,
    ) -> HandleResult<usize> {
        Self::reserve(frame, 1, table)?;
        let token = table.insert(HandleRecord {
            kind: ReferenceKind::Local,
            target,
            owner_thread: Some(thread::current().id()),
            frame: Some(frame.id),
        })?;
        frame.tokens.push(token);
        Ok(token)
    }

    pub(super) fn new_local(
        &mut self,
        target: i32,
        table: &mut HandleTable,
    ) -> HandleResult<usize> {
        Self::insert(
            self.frames.last_mut().ok_or(HandleError::MissingFrame)?,
            target,
            table,
        )
    }

    pub(super) fn delete_local(&mut self, id: usize, token: usize) {
        if let Some(frame) = self.frames.iter_mut().find(|frame| frame.id == id) {
            frame.tokens.retain(|&entry| entry != token);
        }
    }

    pub(super) fn pop(&mut self, object: jobject) -> HandleResult<usize> {
        let child = self.frames.last().ok_or(HandleError::MissingFrame)?;
        if child.native || self.frames.len() < 2 {
            return Err(HandleError::MissingFrame);
        }
        let mut table = handles::table();
        let target = table.resolve(object as usize)?;
        let parent = self.frames.len() - 2;
        // Destination first. A failure leaves the child and all its records intact.
        let result = if target == 0 {
            0
        } else {
            Self::insert(&mut self.frames[parent], target, &mut table)?
        };
        if let Some(child) = self.frames.pop() {
            for token in child.tokens {
                table.records.remove(&token);
            }
        }
        Ok(result)
    }

    pub(super) fn native_frame(&self) -> HandleResult<usize> {
        self.frames
            .iter()
            .rev()
            .find(|frame| frame.native)
            .map(|frame| frame.id)
            .ok_or(HandleError::MissingFrame)
    }

    fn discard_from(&mut self, id: usize) {
        let mut table = handles::table();
        while self.frames.last().is_some_and(|frame| frame.id >= id) {
            if let Some(frame) = self.frames.pop() {
                for token in frame.tokens {
                    table.records.remove(&token);
                }
            }
        }
    }
}

impl Drop for LocalFrames {
    fn drop(&mut self) {
        self.discard_from(0);
    }
}

/// Owns the base frame and any unbalanced PushLocalFrame calls in this native invocation.
pub(crate) struct NativeFrame {
    id: usize,
    _thread_bound: PhantomData<Rc<()>>,
}

impl NativeFrame {
    pub(crate) fn enter() -> Option<Self> {
        let result = JavaThread::with_jni_frames(|frames| frames.push(16, true));
        match result {
            Ok(id) => Some(Self {
                id,
                _thread_bound: PhantomData,
            }),
            Err(error) => {
                error.set_pending();
                None
            }
        }
    }
}

impl Drop for NativeFrame {
    fn drop(&mut self) {
        JavaThread::with_jni_frames(|frames| frames.discard_from(self.id));
        JavaThread::with_jni_leases(|leases| leases.retain(|lease| lease.frame < self.id));
    }
}
