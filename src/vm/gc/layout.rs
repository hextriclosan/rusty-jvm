//! Fixed, descriptor-derived schemas. Neither the schema nor its keys have a mutable API.
use crate::vm::error::{Error, Result};
use crate::vm::heap::java_instance::{ClassName, FieldNameType};
use crate::vm::helper::{get_length, is_reference};
use crate::vm::method_area::field::{FieldInfo, FieldValue};
use indexmap::IndexMap;
use std::sync::Arc;

pub(crate) type InstanceFields = IndexMap<ClassName, IndexMap<FieldNameType, FieldValue>>;

#[derive(Debug)]
enum FieldKind {
    Value,
    Strong,
    Referent,
}

#[derive(Debug)]
struct FieldLayout {
    name: String,
    width: usize,
    kind: FieldKind,
}

#[derive(Debug)]
struct SegmentLayout {
    declaring_class: String,
    fields: Box<[FieldLayout]>,
}

/// Shared by instances, including clones. Private boxed slices, no interior mutability,
/// and no mutable accessors make publication irreversible.
#[derive(Debug)]
pub(crate) struct InstanceLayout {
    segments: Box<[SegmentLayout]>,
}

/// The collector must process the distinguished referent separately from strong closure.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct References {
    pub(crate) strong: Vec<i32>,
    pub(crate) referent: Option<i32>,
}

impl InstanceLayout {
    /// Called before constructing an instance, never by the scanner. Supplying metadata
    /// explicitly also lets tests exercise the real schema without loading Java classes.
    pub(crate) fn new(
        segments: impl IntoIterator<Item = (String, Vec<Arc<FieldInfo>>)>,
    ) -> Result<Self> {
        let mut layout = Vec::new();
        for (declaring_class, fields) in segments {
            let mut slots = Vec::new();
            for field in fields {
                if field.declaring_class() != declaring_class {
                    return Err(Error::new_execution(
                        "Layout field has the wrong declaring class",
                    ));
                }
                let descriptor = field.type_descriptor();
                let kind = if is_reference(descriptor) {
                    if declaring_class == "java/lang/ref/Reference" && field.name() == "referent" {
                        FieldKind::Referent
                    } else {
                        FieldKind::Strong
                    }
                } else {
                    FieldKind::Value
                };
                slots.push(FieldLayout {
                    name: field.name().to_string(),
                    width: usize::try_from(get_length(descriptor)?)?,
                    kind,
                });
            }
            layout.push(SegmentLayout {
                declaring_class,
                fields: slots.into_boxed_slice(),
            });
        }
        Ok(Self {
            segments: layout.into_boxed_slice(),
        })
    }

    /// Reject incomplete or mismatched schemas before a heap entry can be published.
    pub(crate) fn validate(&self, fields: &InstanceFields) -> Result<()> {
        if self.segments.len() != fields.len() {
            return Err(Error::new_execution(
                "Instance layout has missing class segments",
            ));
        }
        for (segment, (class, values)) in self.segments.iter().zip(fields) {
            if segment.declaring_class != *class || segment.fields.len() != values.len() {
                return Err(Error::new_execution(
                    "Instance layout has mismatched fields",
                ));
            }
            for (slot, (name, value)) in segment.fields.iter().zip(values) {
                if slot.name != *name || value.storage_capacity(true)? != slot.width {
                    return Err(Error::new_execution(
                        "Instance layout has a mismatched field slot",
                    ));
                }
            }
        }
        Ok(())
    }

    pub(crate) fn references(&self, fields: &InstanceFields) -> Result<References> {
        let mut references = References::default();
        for segment in &self.segments {
            for slot in &segment.fields {
                match slot.kind {
                    FieldKind::Value => {}
                    FieldKind::Strong | FieldKind::Referent => {
                        // Use the instance's materialized map, not lazy class metadata or
                        // inherited lookup (which would confuse shadowed bare names).
                        let value = fields
                            .get(&segment.declaring_class)
                            .and_then(|values| values.get(&slot.name))
                            .ok_or_else(|| {
                                Error::new_execution("Missing instance reference slot")
                            })?
                            .reference_snapshot()?;
                        match slot.kind {
                            FieldKind::Strong => references.strong.push(value),
                            FieldKind::Referent => references.referent = Some(value),
                            FieldKind::Value => unreachable!(),
                        }
                    }
                }
            }
        }
        references.strong.retain(|&reference| reference != 0);
        Ok(references)
    }
}

#[cfg(test)]
mod tests;
