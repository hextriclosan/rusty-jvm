//! Fixed, descriptor-derived schemas. Neither the schema nor its keys have a mutable API.
use crate::vm::error::{Error, Result};
use crate::vm::gc::budget::AllocError;
use crate::vm::gc::references::{self, Kind};
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
    reference_kind: Kind,
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
        let mut reference_kind = Kind::Other;
        for (declaring_class, fields) in segments {
            match declaring_class.as_str() {
                references::REFERENCE => references::validate_fields(&fields)?,
                "java/lang/ref/SoftReference" => reference_kind = Kind::Soft,
                "java/lang/ref/WeakReference" => reference_kind = Kind::Weak,
                "java/lang/ref/PhantomReference" => reference_kind = Kind::Phantom,
                "java/lang/ref/FinalReference" => reference_kind = Kind::Final,
                _ => {}
            }
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
            reference_kind,
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
        references.referent = self.visit_references(fields, |reference| {
            references.strong.push(reference);
            Ok(())
        })?;
        Ok(references)
    }

    pub(crate) fn reference_kind(&self) -> Kind {
        self.reference_kind
    }

    /// The collector supplies pre-reserved storage. The visitor only copies/deduplicates
    /// IDs, and must not acquire another heap guard or reserve managed storage.
    pub(crate) fn visit_references(
        &self,
        fields: &InstanceFields,
        mut visit: impl FnMut(i32) -> Result<()>,
    ) -> Result<Option<i32>> {
        let mut referent = None;
        if self.segments.len() != fields.len() {
            return Err(AllocError::InvalidLayout.into());
        }
        // validate() fixes both IndexMap orders before publication; field writes
        // cannot add, remove or reorder keys. Walk that order without hashing the
        // declaring class and field name again for every edge in every epoch.
        for (segment, (_, values)) in self.segments.iter().zip(fields) {
            if segment.fields.len() != values.len() {
                return Err(AllocError::InvalidLayout.into());
            }
            for (slot, (_, value)) in segment.fields.iter().zip(values) {
                match slot.kind {
                    FieldKind::Value => {}
                    FieldKind::Strong | FieldKind::Referent => {
                        // Use the instance's materialized map, not lazy class metadata or
                        // inherited lookup (which would confuse shadowed bare names).
                        let value = value.reference_snapshot()?;
                        match slot.kind {
                            FieldKind::Strong if value != 0 => visit(value)?,
                            FieldKind::Strong => {}
                            FieldKind::Referent => referent = Some(value),
                            FieldKind::Value => unreachable!(),
                        }
                    }
                }
            }
        }
        Ok(referent)
    }
}

#[cfg(test)]
mod tests;
