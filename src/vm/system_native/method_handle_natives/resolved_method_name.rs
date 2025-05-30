use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::vm::helper::{i64_to_vec, vec_to_i64};
use getset::CopyGetters;

const RESOLVED_METHOD_NAME: &'static str = "java/lang/invoke/ResolvedMethodName";

#[derive(Debug, CopyGetters)]
#[get_copy = "pub"]
pub struct ResolvedMethodName {
    resolved_method_name_ref: i32,
    vmholder: i32,
    vmtarget: i64,
}

impl ResolvedMethodName {
    pub fn new_create_instance(vmholder: i32, vmtarget: i64) -> Result<Self> {
        let resolved_method_name_ref = Executor::invoke_default_constructor(RESOLVED_METHOD_NAME)?;
        Ok(Self {
            resolved_method_name_ref,
            vmholder,
            vmtarget,
        })
    }

    pub fn new_load_by_ref(resolved_method_name_ref: i32) -> Result<Self> {
        let vmholder = load_vmholder(resolved_method_name_ref)?;
        let vmtarget = load_vmtarget(resolved_method_name_ref)?;
        Ok(Self {
            resolved_method_name_ref,
            vmholder,
            vmtarget,
        })
    }

    /// Writes to the underlying java instance the values of vmholder and vmtarget:
    /// - vmholder = methodName.clazz;
    /// - vmtarget = ref to the target method's vtable (Method* in HotSpot), index in the IndexMap in our case
    pub fn propagate_all(&self) -> Result<()> {
        self.propagate_vmholder()?;
        self.propagate_vmtarget()?;
        Ok(())
    }

    fn propagate_vmholder(&self) -> Result<()> {
        with_heap_write_lock(|heap| {
            heap.set_object_field_value(
                self.resolved_method_name_ref,
                RESOLVED_METHOD_NAME,
                "vmholder",
                vec![self.vmholder],
            )
        })?;
        Ok(())
    }

    fn propagate_vmtarget(&self) -> Result<()> {
        with_heap_write_lock(|heap| {
            heap.set_object_field_value(
                self.resolved_method_name_ref,
                RESOLVED_METHOD_NAME,
                "vmtarget",
                i64_to_vec(self.vmtarget),
            )
        })?;
        Ok(())
    }
}

fn load_vmholder(resolved_method_name_ref: i32) -> Result<i32> {
    let vmholder_class_ref = with_heap_read_lock(|heap| {
        heap.get_object_field_value(resolved_method_name_ref, RESOLVED_METHOD_NAME, "vmholder")
    })?[0];
    Ok(vmholder_class_ref)
}

fn load_vmtarget(resolved_method_name_ref: i32) -> Result<i64> {
    let vmtarget_raw = with_heap_read_lock(|heap| {
        heap.get_object_field_value(resolved_method_name_ref, RESOLVED_METHOD_NAME, "vmtarget")
    })?;
    Ok(vec_to_i64(&vmtarget_raw))
}
