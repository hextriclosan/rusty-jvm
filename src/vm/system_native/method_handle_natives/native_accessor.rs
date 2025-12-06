use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::heap::java_instance::Array;
use crate::vm::helper::clazz_ref;
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::method_area::loaded_classes::CLASSES;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::stack::stack_value::{StackValue, StackValueKind};
use std::sync::Arc;

pub fn native_accessor_invoke0(method_ref: i32, obj_ref: i32, args_ref: i32) -> Result<i32> {
    let (method, args) = resolve_method_and_args(method_ref, args_ref)?;

    let ret = if obj_ref == 0 {
        Executor::invoke_static_method(method.class_name(), method.name_signature(), &args)?[0]
    } else {
        let ret = Executor::invoke_non_static_method(
            method.class_name(),
            method.name_signature(),
            obj_ref,
            &args,
        )?;
        match ret.len() {
            0 => 0,
            1 => ret[0],
            2 => {
                return Err(Error::new_execution(&format!(
                    "Method {}.{} returned multiple values, which is not supported (yet)",
                    method.class_name(),
                    method.name_signature()
                )))
            }
            _ => {
                return Err(Error::new_execution(&format!(
                    "Method {}.{} returned unexpected number of values: {}",
                    method.class_name(),
                    method.name_signature(),
                    ret.len()
                )))
            }
        }
    };

    Ok(ret)
}

pub fn native_accessor_newinstance0(constructor_ref: i32, args_ref: i32) -> Result<i32> {
    let (method, args) = resolve_method_and_args(constructor_ref, args_ref)?;

    Executor::invoke_args_constructor(
        method.class_name(),
        method.name_signature(),
        &args,
        Some("Native Accessor newInstance0"),
    )
}

fn resolve_method_and_args(
    method_or_constructor_ref: i32,
    args_ref: i32,
) -> Result<(Arc<JavaMethod>, Vec<StackValueKind>)> {
    let method_name = HEAP.get_instance_name(method_or_constructor_ref)?;
    let clazz_ref =
        HEAP.get_object_field_value(method_or_constructor_ref, method_name.as_str(), "clazz")?[0];
    let slot =
        HEAP.get_object_field_value(method_or_constructor_ref, method_name.as_str(), "slot")?[0];
    let parameter_types_ref = HEAP.get_object_field_value(
        method_or_constructor_ref,
        method_name.as_str(),
        "parameterTypes",
    )?[0];

    let entire_array_args = if args_ref != 0 {
        HEAP.get_entire_array(args_ref)?
    } else {
        Array::new("[Ljava/lang/Object;", 0)
    };
    let parameter_types = HEAP.get_entire_array(parameter_types_ref)?;

    let jc = with_method_area(|a| {
        let clazz_name = a.get_from_reflection_table(clazz_ref)?;
        CLASSES.get(&clazz_name) // fixme!!! get Klass from clazz_ref directly
    })?;

    let method = jc.get_method_by_index(slot as i64)?;
    let entire_value = entire_array_args.get_entire_value();
    let args = entire_value
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let clazz_ref = parameter_types.get_value(index as i32)?[0];
            unbox_if_needed(value[0], clazz_ref)
        })
        .collect::<Result<Vec<StackValueKind>>>()?;
    Ok((method, args))
}

fn unbox_if_needed(obj_ref: i32, clazz_ref_: i32) -> Result<StackValueKind> {
    let primitive =
        Executor::invoke_non_static_method("java/lang/Class", "isPrimitive:()Z", clazz_ref_, &[])?
            [0]
            != 0;

    if !primitive {
        return Ok(StackValueKind::I32(obj_ref)); // Return as object if not primitive
    }

    let res = if clazz_ref_ == clazz_ref("B")? {
        let raw =
            Executor::invoke_non_static_method("java/lang/Byte", "byteValue:()B", obj_ref, &[])?;
        StackValueKind::I32(i32::from_vec(&raw))
    } else if clazz_ref_ == clazz_ref("Z")? {
        let raw = Executor::invoke_non_static_method(
            "java/lang/Boolean",
            "booleanValue:()Z",
            obj_ref,
            &[],
        )?;
        StackValueKind::I32(i32::from_vec(&raw))
    } else if clazz_ref_ == clazz_ref("S")? {
        let raw =
            Executor::invoke_non_static_method("java/lang/Short", "shortValue:()S", obj_ref, &[])?;
        StackValueKind::I32(i32::from_vec(&raw))
    } else if clazz_ref_ == clazz_ref("C")? {
        let raw = Executor::invoke_non_static_method(
            "java/lang/Character",
            "charValue:()C",
            obj_ref,
            &[],
        )?;
        StackValueKind::I32(i32::from_vec(&raw))
    } else if clazz_ref_ == clazz_ref("I")? {
        let raw =
            Executor::invoke_non_static_method("java/lang/Integer", "intValue:()I", obj_ref, &[])?;
        StackValueKind::I32(i32::from_vec(&raw))
    } else if clazz_ref_ == clazz_ref("F")? {
        let raw =
            Executor::invoke_non_static_method("java/lang/Float", "floatValue:()F", obj_ref, &[])?;
        StackValueKind::F32(f32::from_vec(&raw))
    } else if clazz_ref_ == clazz_ref("J")? {
        let raw =
            Executor::invoke_non_static_method("java/lang/Long", "longValue:()J", obj_ref, &[])?;
        StackValueKind::I64(i64::from_vec(&raw))
    } else if clazz_ref_ == clazz_ref("D")? {
        let raw = Executor::invoke_non_static_method(
            "java/lang/Double",
            "doubleValue:()D",
            obj_ref,
            &[],
        )?;
        StackValueKind::F64(f64::from_vec(&raw))
    } else {
        return Err(Error::new_execution(&format!(
            "Unrecognized primitive type with clazz_ref {clazz_ref_}"
        )));
    };

    Ok(res)
}
