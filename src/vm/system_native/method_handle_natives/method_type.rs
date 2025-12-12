use crate::vm::error::Result;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{decorate, klass};
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
        let ptype_class_refs =
            HEAP.get_object_field_value(type_obj_ref, METHOD_TYPE, "ptypes")?[0];
        let rtype_class_ref = HEAP.get_object_field_value(type_obj_ref, METHOD_TYPE, "rtype")?[0];

        let ptype_names = generate_parameters(ptype_class_refs)?;
        let rtype_name = generate_return_type(rtype_class_ref)?;

        Ok(Self {
            ptype_names,
            rtype_name,
        })
    }
}

fn generate_parameters(ptype_class_refs: i32) -> Result<String> {
    let len = HEAP.get_array_len(ptype_class_refs)?;
    let mut ptype_names = Vec::with_capacity(len as usize);
    for i in 0..len {
        let ptype_class_ref = HEAP.get_array_value(ptype_class_refs, i)?[0];
        let ptype_name = klass(ptype_class_ref)?.this_class_name().to_owned();
        ptype_names.push(decorate(ptype_name));
    }
    let ptype_names_string = format!("({})", ptype_names.join(""));
    Ok(ptype_names_string)
}

fn generate_return_type(rtype_class_ref: i32) -> Result<String> {
    let rtype_name = klass(rtype_class_ref)?.this_class_name().to_owned();
    Ok(decorate(rtype_name))
}
