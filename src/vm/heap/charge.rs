//! Versioned formula in docs/managed-heap-accounting.md.
use crate::vm::gc::budget::{add, aligned, bytes, AllocError, AllocResult};
use crate::vm::gc::layout::InstanceFields;
use crate::vm::heap::java_instance::HeapValue;
use crate::vm::method_area::field::FieldValue;
use indexmap::IndexMap;
use std::mem::size_of;

/// Estimated heap-table and recorded-charge-table bucket overhead per entry.
const ENTRY_ALLOWANCE: u64 = 64;
/// Estimated IndexMap hash/index bucket overhead per capacity slot.
const MAP_SLOT_ALLOWANCE: usize = {
    let Some(words) = size_of::<usize>().checked_mul(2) else {
        panic!("map slot words must fit usize");
    };
    let Some(bytes) = words.checked_add(1) else {
        panic!("map slot allowance must fit usize");
    };
    bytes
};

pub(super) fn header() -> AllocResult<u64> {
    aligned(
        bytes(1, size_of::<HeapValue>())?
            .checked_add(ENTRY_ALLOWANCE)
            .ok_or(AllocError::Arithmetic)?,
    )
}

pub(super) fn array(descriptor_capacity: usize, payload_capacity: usize) -> AllocResult<u64> {
    let mut total = header()?;
    add(&mut total, bytes(descriptor_capacity, 1)?)?;
    add(&mut total, bytes(payload_capacity, 1)?)?;
    Ok(total)
}

pub(super) fn fields(fields: &InstanceFields, planned: bool) -> AllocResult<u64> {
    let mut total = header()?;
    let outer_capacity = if planned {
        fields.len()
    } else {
        fields.capacity()
    };
    add(
        &mut total,
        bytes(
            outer_capacity,
            size_of::<(String, IndexMap<String, FieldValue>)>()
                .checked_add(MAP_SLOT_ALLOWANCE)
                .ok_or(AllocError::Arithmetic)?,
        )?,
    )?;
    for (class, values) in fields {
        add(
            &mut total,
            bytes(
                if planned {
                    class.len()
                } else {
                    class.capacity()
                },
                1,
            )?,
        )?;
        let capacity = if planned {
            values.len()
        } else {
            values.capacity()
        };
        add(
            &mut total,
            bytes(
                capacity,
                size_of::<(String, FieldValue)>()
                    .checked_add(MAP_SLOT_ALLOWANCE)
                    .ok_or(AllocError::Arithmetic)?,
            )?,
        )?;
        for (name, value) in values {
            add(
                &mut total,
                bytes(if planned { name.len() } else { name.capacity() }, 1)?,
            )?;
            add(
                &mut total,
                bytes(value.storage_capacity(planned)?, size_of::<i32>())?,
            )?;
        }
    }
    Ok(total)
}

pub(super) fn intern(capacity: usize) -> AllocResult<u64> {
    let mut total = aligned(
        bytes(1, size_of::<(String, i32)>())?
            .checked_add(ENTRY_ALLOWANCE)
            .ok_or(AllocError::Arithmetic)?,
    )?;
    add(&mut total, bytes(capacity, 1)?)?;
    Ok(total)
}
