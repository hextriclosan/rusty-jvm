//! Reference policy is classified when immutable layouts are built, never by the tracer.
use crate::vm::error::{Error, Result};
use crate::vm::method_area::field::FieldInfo;
use std::sync::Arc;

pub(crate) const REFERENCE: &str = "java/lang/ref/Reference";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    Other,
    Soft,
    Weak,
    Phantom,
    Final,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Reference {
    pub(crate) object: i32,
    pub(crate) kind: Kind,
    pub(crate) referent: i32,
    pub(crate) discovered: i32,
    pub(crate) next: i32,
}

impl Reference {
    pub(crate) fn active(self) -> bool {
        // Some(0) from the layout scanner is a cleared referent, not a discovery.
        self.referent != 0 && self.discovered == 0 && self.next == 0
    }
}

pub(crate) fn validate_fields(fields: &[Arc<FieldInfo>]) -> Result<()> {
    for (name, descriptor) in [
        ("referent", "Ljava/lang/Object;"),
        ("queue", "Ljava/lang/ref/ReferenceQueue;"),
        ("next", "Ljava/lang/ref/Reference;"),
        ("discovered", "Ljava/lang/ref/Reference;"),
    ] {
        if !fields
            .iter()
            .any(|field| field.name() == name && field.type_descriptor().to_string() == descriptor)
        {
            return Err(Error::new_execution(&format!(
                "Unsupported Reference layout: expected {name}:{descriptor}"
            )));
        }
    }
    Ok(())
}
