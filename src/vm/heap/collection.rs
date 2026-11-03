use super::heap::Heap;
use super::java_instance::HeapValue;
use crate::vm::commons::auto_dash_map::auto_dash_map::AutoDashMap;
use crate::vm::error::Result;
use crate::vm::gc::budget::AllocError;
use crate::vm::gc::references::Reference;

impl Heap {
    #[cfg(test)]
    pub(crate) fn set_limit_for_test(&self, max: u64) {
        self.admission.lock().budget.max = max;
    }

    /// Admission precedes every heap shard. No mutator may have an unfinished
    /// reservation at a collection boundary.
    pub(crate) fn collection_entries(&self) -> Result<Vec<(i32, u64)>> {
        let admission = self.admission.lock();
        if admission.budget.reserved != 0 {
            return Err(AllocError::InvalidLayout.into());
        }
        let mut entries = Vec::new();
        entries
            .try_reserve_exact(admission.charges.len())
            .map_err(|_| AllocError::HostAllocation)?;
        let mut total = 0_u64;
        for (&reference, &charge) in &admission.charges {
            total = total.checked_add(charge).ok_or(AllocError::Arithmetic)?;
            entries.push((reference, charge));
        }
        if total > admission.budget.used {
            return Err(AllocError::InvalidLayout.into());
        }
        Ok(entries)
    }

    pub(crate) fn memory(&self) -> (u64, u64) {
        let admission = self.admission.lock();
        (admission.budget.max, admission.budget.used)
    }

    pub(crate) fn contains(&self, reference: i32) -> bool {
        self.data.contains_key(reference)
    }

    /// Copy/deduplicate IDs into preflighted scratch under ONE guard. The visitor
    /// cannot follow an edge, allocate, or acquire admission. Following happens
    /// only after this method returns and the guard is gone.
    pub(crate) fn trace_edges(
        &self,
        reference: i32,
        visit: impl FnMut(i32) -> Result<()>,
    ) -> Result<Option<Reference>> {
        let entry = self.data.get(reference).ok_or(AllocError::InvalidLayout)?;
        match entry.value() {
            HeapValue::Object(instance) => instance.visit_references(reference, visit),
            HeapValue::Arr(array) => {
                array.visit_references(visit)?;
                Ok(None)
            }
        }
    }

    pub(crate) fn set_reference_link(&self, reference: i32, name: &str, value: i32) -> Result<()> {
        let entry = self.data.get(reference).ok_or(AllocError::InvalidLayout)?;
        match entry.value() {
            HeapValue::Object(instance) => instance.set_reference_field(name, value),
            HeapValue::Arr(_) => Err(AllocError::InvalidLayout.into()),
        }
    }
}
