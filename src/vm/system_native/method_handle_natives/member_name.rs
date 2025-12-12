use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::klass;
use crate::vm::system_native::method_handle_natives::resolved_method_name::ResolvedMethodName;
use crate::vm::system_native::method_handle_natives::types::ReferenceKind;
use crate::vm::system_native::method_handle_natives::types::ReferenceKind::{
    REF_getField, REF_getStatic, REF_invokeSpecial, REF_invokeStatic, REF_newInvokeSpecial,
    REF_putField, REF_putStatic,
};
use crate::vm::system_native::string::get_utf8_string_by_ref;
use getset::{CopyGetters, Getters};

const MEMBER_NAME: &'static str = "java/lang/invoke/MemberName";

#[derive(Debug, Getters, CopyGetters)]
pub struct MemberName {
    #[get_copy = "pub"]
    member_name_ref: i32,
    #[get_copy = "pub"]
    flags: i32,
    #[get = "pub"]
    class_name: String,
    #[get = "pub"]
    name: String,
    #[get_copy = "pub"]
    type_obj_ref: i32,
    #[get_copy = "pub"]
    reference_kind: ReferenceKind,
    method: Option<ResolvedMethodName>,
}

impl MemberName {
    pub fn new(member_name_ref: i32) -> Result<Self> {
        let flags = HEAP.get_object_field_value(member_name_ref, MEMBER_NAME, "flags")?[0];
        let class_ref = HEAP.get_object_field_value(member_name_ref, MEMBER_NAME, "clazz")?[0];
        let name_ref = HEAP.get_object_field_value(member_name_ref, MEMBER_NAME, "name")?[0];
        let type_obj_ref = HEAP.get_object_field_value(member_name_ref, MEMBER_NAME, "type")?[0];

        let class_name = klass(class_ref)?.this_class_name().to_owned();
        let name = get_utf8_string_by_ref(name_ref)?;
        let reference_kind = get_reference_kind(flags)?;
        let method = load_method(member_name_ref)?;

        Ok(Self {
            member_name_ref,
            flags,
            class_name,
            name,
            type_obj_ref,
            reference_kind,
            method,
        })
    }

    pub fn method(&self) -> Option<&ResolvedMethodName> {
        self.method.as_ref()
    }

    pub fn propagate_flags(&mut self, flags: i32) -> Result<()> {
        HEAP.set_object_field_value(self.member_name_ref, MEMBER_NAME, "flags", vec![flags])?;
        self.flags = flags;
        Ok(())
    }

    pub fn propagate_method(&mut self, method: ResolvedMethodName) -> Result<()> {
        self.method = Some(method);

        if let Some(method) = &self.method {
            method.propagate_all()?;
        }

        self.propagate_method_ref()?;
        Ok(())
    }

    fn propagate_method_ref(&self) -> Result<()> {
        if let Some(method) = self.method.as_ref() {
            HEAP.set_object_field_value(
                self.member_name_ref,
                MEMBER_NAME,
                "method",
                vec![method.resolved_method_name_ref()],
            )?;
        }
        Ok(())
    }

    // This method returns ref to an array of 2 elements: Object[] {Long vmindex, Object vmtarget}
    pub fn get_member_vm_info(&self) -> Result<i32> {
        let reference_kind = self.reference_kind();
        let array_ref = HEAP.create_array("[Ljava/lang/Object;", 2);
        // vmindex it is an index of the method in the vtable, HotSpot uses negative value for methods that don't need dynamic dispatch
        // We don't have vtable (yet), thus we use -2 for all methods that are not either of virtual or interface
        let vmindex = match reference_kind {
            REF_invokeStatic | REF_invokeSpecial | REF_newInvokeSpecial => -2,
            _ => 0,
        } as i64;
        let args = vec![vmindex.into()];
        let long_instance_ref = Executor::invoke_args_constructor(
            "java/lang/Long",
            "<init>:(J)V",
            &args,
            Some("Long instance creation"),
        )?;

        let vmtarget = match reference_kind {
            REF_getField | REF_getStatic | REF_putField | REF_putStatic => self.type_obj_ref(),
            _ => self.member_name_ref,
        };

        HEAP.set_array_value(array_ref, 0, vec![long_instance_ref])?;
        HEAP.set_array_value(array_ref, 1, vec![vmtarget])?;

        Ok(array_ref)
    }
}

fn load_method(member_name_ref: i32) -> Result<Option<ResolvedMethodName>> {
    let resolved_method_name_ref =
        HEAP.get_object_field_value(member_name_ref, MEMBER_NAME, "method")?[0];

    if resolved_method_name_ref == 0 {
        return Ok(None);
    }

    let resolved_method_name = ResolvedMethodName::new_load_by_ref(resolved_method_name_ref)?;
    Ok(Some(resolved_method_name))
}

const KIND_SHIFT: u32 = 24;
const KIND_MASK: u32 = 0x0F;
/**
 * Mimics MemberName.getReferenceKind():
 *  public byte getReferenceKind() {
 *      return (byte) ((flags >>> MN_REFERENCE_KIND_SHIFT) & MN_REFERENCE_KIND_MASK);
 *  }
 * todo: use more strict and safe way to get reference kind
 */
fn get_reference_kind(flags: i32) -> Result<ReferenceKind> {
    let result = (flags as u32 >> KIND_SHIFT) & KIND_MASK;

    Ok(ReferenceKind::try_from(result as u8)?)
}

pub fn set_reference_kind(flags: i32, reference_kind: ReferenceKind) -> i32 {
    ((flags as u32 & !(KIND_MASK << KIND_SHIFT))
        | ((reference_kind as u32 & KIND_MASK) << KIND_SHIFT)) as i32
}
