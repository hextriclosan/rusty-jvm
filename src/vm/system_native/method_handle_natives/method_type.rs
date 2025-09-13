use crate::vm::error::{Error, Result};
use crate::vm::heap::heap::with_heap_read_lock;
use crate::vm::helper::decorate;
use crate::vm::method_area::method_area::with_method_area;
use getset::Getters;

const METHOD_TYPE: &'static str = "java/lang/invoke/MethodType";

#[derive(Debug, Getters)]
#[get = "pub"]
pub struct MethodType {
    ptype_names: String,
    rtype_name: String,
}

impl MethodType {
    pub fn new(type_obj_ref: i32) -> Result<Self> {
        let (rtype_class_ref, ptype_class_refs) = with_heap_read_lock(|heap| {
            let rtype_class_ref =
                heap.get_object_field_value(type_obj_ref, METHOD_TYPE, "rtype")?[0];
            let ptype_class_refs =
                heap.get_object_field_value(type_obj_ref, METHOD_TYPE, "ptypes")?[0];
            Ok::<(i32, i32), Error>((rtype_class_ref, ptype_class_refs))
        })?;

        let ptype_names = generate_parameters(ptype_class_refs)?;
        let rtype_name = generate_return_type(rtype_class_ref)?;

        Ok(Self {
            ptype_names,
            rtype_name,
        })
    }
}

fn generate_parameters(ptype_class_refs: i32) -> Result<String> {
    let len = with_heap_read_lock(|heap| heap.get_array_len(ptype_class_refs))?;
    let mut ptype_names = Vec::with_capacity(len as usize);
    for i in 0..len {
        let ptype_class_ref =
            with_heap_read_lock(|heap| heap.get_array_value(ptype_class_refs, i))?[0];
        let ptype_name = with_method_area(|area| area.get_from_reflection_table(ptype_class_ref))?;
        ptype_names.push(decorate(ptype_name));
    }
    let ptype_names_string = format!("({})", ptype_names.join(""));
    Ok(ptype_names_string)
}

fn generate_return_type(rtype_class_ref: i32) -> Result<String> {
    let rtype_name = with_method_area(|area| area.get_from_reflection_table(rtype_class_ref))?;
    Ok(decorate(rtype_name))
}
