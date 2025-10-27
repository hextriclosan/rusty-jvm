use crate::vm::error::{Error, Result};
use crate::vm::exception::helpers::throw_class_not_found_exception;
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::{with_heap_read_lock, with_heap_write_lock};
use crate::vm::helper::{strip_nest_host, undecorate};
use crate::vm::method_area::class_modifiers::ClassModifier;
use crate::vm::method_area::instance_checker::InstanceChecker;
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::method_area::primitives_helper::PRIMITIVE_CODE_BY_TYPE;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use crate::{throw_and_return, unwrap_or_return_err, unwrap_result_or_return_default};
use std::sync::Arc;

const PUBLIC: u16 = 0x00000001;

pub(crate) fn get_superclass_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let current_clazz_ref = args[0];
    let super_clazz_ref = get_superclass(current_clazz_ref)?;

    Ok(vec![super_clazz_ref])
}
fn get_superclass(clazz_ref: i32) -> Result<i32> {
    let current_clazz_ref = with_method_area(|method_area| {
        let name = method_area.get_from_reflection_table(clazz_ref)?;
        let rc = method_area.get(&name)?;
        let parent = if !rc.is_interface() {
            rc.parent().clone()
        } else {
            None
        };

        let current_clazz_ref =
            parent.map_or(Ok(0), |parent| method_area.load_reflection_class(&parent));
        current_clazz_ref
    })?;

    Ok(current_clazz_ref)
}

pub(crate) fn get_primitive_class_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let string_ref = args[0];
    let class_ref = get_primitive_class(string_ref)?;
    Ok(vec![class_ref])
}

fn get_primitive_class(string_ref: i32) -> Result<i32> {
    let string_content = get_utf8_string_by_ref(string_ref)?;
    let mapped_class_name = map_primitive_class(&string_content)?;

    let reflection_ref =
        with_method_area(|method_area| method_area.load_reflection_class(mapped_class_name))?;

    Ok(reflection_ref)
}
fn map_primitive_class(primitive_type: &str) -> Result<&str> {
    let matched = PRIMITIVE_CODE_BY_TYPE.get(primitive_type).ok_or_else(|| {
        Error::new_execution(&format!("Unsupported primitive type: {primitive_type}"))
    })?;

    Ok(matched)
}

pub(crate) fn class_init_class_name_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let class_ref = args[0];
    let string_ref = init_class_name(class_ref)?;

    Ok(vec![string_ref])
}
/// Returns the class name for the given class reference.
/// Updates Class.name field in the heap.
fn init_class_name(class_ref: i32) -> Result<i32> {
    let class_name = with_method_area(|method_area| {
        let class_name = method_area.get_from_reflection_table(class_ref)?;
        let class_name = undecorate(&class_name);
        let jc = method_area.get(class_name)?;
        let class_name = jc.external_name();
        Ok::<String, Error>(class_name.to_string())
    })?;

    let string_ref = StringPoolHelper::get_string(&class_name)?;

    with_heap_write_lock(|heap| {
        heap.set_object_field_value(class_ref, "java/lang/Class", "name", vec![string_ref])
    })?;

    Ok(string_ref)
}

pub(crate) fn for_name0_wrp(args: &[i32], stack_frames: &mut StackFrames) -> Result<Vec<i32>> {
    let name_ref = args[0];
    let initialize = args[1] != 0;
    let loader_ref = args[2];
    let caller_ref = args[3];

    let class_ref = unwrap_result_or_return_default!(
        for_name0(name_ref, initialize, loader_ref, caller_ref, stack_frames),
        vec![]
    );
    Ok(vec![class_ref])
}
fn for_name0(
    name_ref: i32,
    initialize: bool,
    _loader_ref: i32,
    _caller_ref: i32,
    stack_frames: &mut StackFrames,
) -> ThrowingResult<i32> {
    let name = unwrap_or_return_err!(get_utf8_string_by_ref(name_ref));
    let internal_name = name.replace('.', "/");
    let reflection_ref =
        match with_method_area(|method_area| method_area.load_reflection_class(&internal_name)) {
            Ok(r) => r,
            Err(_) => {
                throw_and_return!(throw_class_not_found_exception(&name, stack_frames))
            }
        };

    if initialize {
        unwrap_or_return_err!(StaticInit::initialize(&internal_name));
    }

    ThrowingResult::ok(reflection_ref)
}
pub(crate) fn get_interfaces0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let class_ref = args[0];
    let interfaces_ref = get_interfaces0(class_ref)?;
    Ok(vec![interfaces_ref])
}
fn get_interfaces0(class_ref: i32) -> Result<i32> {
    let interface_refs = with_method_area(|method_area| {
        let class_name = method_area.get_from_reflection_table(class_ref)?;
        let jc = method_area.get(&class_name)?;
        let interfaces = jc.interfaces();

        let interface_refs = interfaces
            .iter()
            .map(|interface| method_area.load_reflection_class(interface))
            .collect::<Result<Vec<i32>>>();
        interface_refs
    })?;

    let result_ref = with_heap_write_lock(|heap| {
        heap.create_array_with_values("[Ljava/lang/Class;", &interface_refs)
    });
    Ok(result_ref)
}

pub(crate) fn get_declaring_class0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let clazz_ref = args[0];
    let declaring_class_ref = get_declaring_class0(clazz_ref)?;
    Ok(vec![declaring_class_ref])
}
fn get_declaring_class0(clazz_ref: i32) -> Result<i32> {
    let declaring_class_ref = with_method_area(|method_area| {
        let class_name = method_area.get_from_reflection_table(clazz_ref)?;
        let jc = method_area.get(&class_name)?;
        let declaring_class_ref = jc
            .declaring_class()
            .as_ref()
            .map(|declaring_class| {
                let declaring_class_ref = method_area.load_reflection_class(declaring_class);
                declaring_class_ref
            })
            .unwrap_or(Ok(0));

        declaring_class_ref
    });

    declaring_class_ref
}

pub(crate) fn get_declared_fields0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let class_ref = args[0];
    let public_only = args[1] != 0;
    let fields_ref = get_declared_fields(class_ref, public_only)?;
    Ok(vec![fields_ref])
}

fn get_declared_fields(class_ref: i32, public_only: bool) -> Result<i32> {
    let jc = with_method_area(|method_area| {
        let class_name = method_area.get_from_reflection_table(class_ref)?;
        method_area.get(&class_name)
    })?;

    let fields_info = jc.get_fields_info();
    let fields_info_ref = fields_info
        .iter()
        .filter_map(|field_info| {
            if public_only && (field_info.flags() & PUBLIC) == 0 {
                return None; // Skip non-public fields
            }

            Some(field_info.reflection_ref())
        })
        .collect::<Result<Vec<_>>>()?;

    let result_ref = with_heap_write_lock(|heap| {
        heap.create_array_with_values("[Ljava/lang/reflect/Field;", &fields_info_ref)
    });

    Ok(result_ref)
}

pub(crate) fn get_declared_methods0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let class_ref = args[0];
    let public_only = args[1] != 0;
    let methods_ref = get_declared_methods(class_ref, public_only)?;
    Ok(vec![methods_ref])
}
fn get_declared_methods(class_ref: i32, public_only: bool) -> Result<i32> {
    let java_methods = with_method_area(|method_area| {
        let class_name = method_area.get_from_reflection_table(class_ref)?;
        let jc = method_area.get(&class_name)?;
        Ok::<Vec<Arc<JavaMethod>>, Error>(jc.get_methods())
    })?;

    let method_refs = java_methods
        .iter()
        .filter_map(|java_method| {
            if (java_method.name() == "<init>") || (java_method.name() == "<clinit>") {
                return None; // Skip constructors and static initializers
            }

            if public_only && (java_method.access_flags() as u16 & PUBLIC) == 0 {
                return None; // Skip non-public methods
            }

            Some(java_method.reflection_ref())
        })
        .collect::<Result<Vec<_>>>()?;

    let result_ref = with_heap_write_lock(|heap| {
        heap.create_array_with_values("[Ljava/lang/reflect/Method;", &method_refs)
    });
    Ok(result_ref)
}

pub(crate) fn get_declared_constructors0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let class_ref = args[0];
    let constructors_ref = get_declared_constructors(class_ref)?;
    Ok(vec![constructors_ref])
}
fn get_declared_constructors(class_ref: i32) -> Result<i32> {
    let java_methods = with_method_area(|method_area| {
        let class_name = method_area.get_from_reflection_table(class_ref)?;
        let jc = method_area.get(&class_name)?;
        Ok::<Vec<Arc<JavaMethod>>, Error>(jc.get_methods())
    })?;

    let method_refs = java_methods
        .iter()
        .filter(|java_method| java_method.name() == "<init>")
        .map(|java_method| java_method.reflection_ref())
        .collect::<Result<Vec<_>>>()?;

    let result_ref = with_heap_write_lock(|heap| {
        heap.create_array_with_values("[Ljava/lang/reflect/Constructor;", &method_refs)
    });
    Ok(result_ref)
}

pub(crate) fn get_enclosing_method0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let class_ref = args[0];
    let enclosing_method_ref = get_enclosing_method0(class_ref)?;
    Ok(vec![enclosing_method_ref])
}
fn get_enclosing_method0(class_ref: i32) -> Result<i32> {
    if let Some((class_name, name, descriptor)) = with_method_area(|method_area| {
        let class_name = method_area.get_from_reflection_table(class_ref)?;
        let jc = method_area.get(&class_name)?;
        Ok::<Option<(String, String, String)>, Error>(jc.enclosing_method().clone())
    })? {
        let class_name_ref =
            with_method_area(|method_area| method_area.load_reflection_class(&class_name))?;
        let name_ref = StringPoolHelper::get_string(&name)?;
        let descriptor_ref = StringPoolHelper::get_string(&descriptor)?;

        let array_ref = with_heap_write_lock(|heap| {
            heap.create_array_with_values(
                "[Ljava/lang/reflect/Method;",
                &[class_name_ref, name_ref, descriptor_ref],
            )
        });

        Ok(array_ref) // new Object[] {class_name, name, descriptor}
    } else {
        Ok(0)
    }
}

pub(crate) fn get_raw_annotations_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let class_ref = args[0];
    let raw_annotations_ref = get_raw_annotations(class_ref)?;

    Ok(vec![raw_annotations_ref])
}
fn get_raw_annotations(reference: i32) -> Result<i32> {
    let annotations_raw = with_method_area(|method_area| {
        let name = method_area.get_from_reflection_table(reference)?;
        let rc = method_area.get(&name)?;
        let annotations_raw = rc.annotations_raw().clone();
        Ok::<Option<Vec<u8>>, Error>(annotations_raw)
    })?;

    let annotations_ref = annotations_raw
        .map(|annotations_raw| {
            let vec = annotations_raw
                .iter()
                .map(|x| *x as i32)
                .collect::<Vec<_>>();
            let annotations =
                with_heap_write_lock(|heap| heap.create_array_with_values("[B", &vec));
            annotations
        })
        .unwrap_or(0);

    Ok(annotations_ref)
}

pub(crate) fn get_simple_binary_name0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let class_ref = args[0];
    let simple_binary_name_ref = get_simple_binary_name0(class_ref)?;
    Ok(vec![simple_binary_name_ref])
}
fn get_simple_binary_name0(class_ref: i32) -> Result<i32> {
    let name = with_method_area(|method_area| {
        let name = method_area.get_from_reflection_table(class_ref)?;
        Ok::<String, Error>(name)
    })?;

    let string_ref = name
        .rsplit_once('$')
        .map(|(_, last_token)| last_token)
        .filter(|s| s.parse::<u64>().is_err())
        .map(|s| StringPoolHelper::get_string(s))
        .unwrap_or(Ok(0));
    string_ref
}

pub(crate) fn is_assignable_from_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let assignable_class_ref = args[0];
    let assignee_class_ref = args[1];
    let is_assignable = is_assignable_from(assignable_class_ref, assignee_class_ref)?;

    Ok(vec![if is_assignable { 1 } else { 0 }])
}
fn is_assignable_from(assignable_class_ref: i32, assignee_class_ref: i32) -> Result<bool> {
    let is_assignable = with_method_area(|method_area| {
        let assignable_class_name = method_area.get_from_reflection_table(assignable_class_ref)?;
        let assignee_class_name = method_area.get_from_reflection_table(assignee_class_ref)?;

        let instance_of =
            InstanceChecker::checkcast(&assignee_class_name, &assignable_class_name)?;
        Ok::<bool, Error>(instance_of)
    })?;

    Ok(is_assignable)
}

pub(crate) fn class_is_instance_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let this_class_ref = args[0];
    let obj_ref_to_check = args[1];
    let is_instance = class_is_instance(this_class_ref, obj_ref_to_check)?;

    Ok(vec![if is_instance { 1 } else { 0 }])
}
fn class_is_instance(this_class_ref: i32, obj_ref_to_check: i32) -> Result<bool> {
    if obj_ref_to_check == 0 {
        return Ok(false);
    }

    let this_class_name =
        with_method_area(|method_area| method_area.get_from_reflection_table(this_class_ref))?;
    let class_name_to_check =
        with_heap_read_lock(|heap| heap.get_instance_name(obj_ref_to_check))?;

    InstanceChecker::checkcast(&class_name_to_check, &this_class_name)
}

pub(crate) fn get_constant_pool_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let clazz_ref = args[0];
    let constant_pool_ref = get_constant_pool(clazz_ref)?;
    Ok(vec![constant_pool_ref])
}
fn get_constant_pool(clazz_ref: i32) -> Result<i32> {
    const NAME: &'static str = "jdk/internal/reflect/ConstantPool";
    let constant_pool_ref = Executor::invoke_default_constructor(NAME)?;
    with_heap_write_lock(|heap| {
        heap.set_object_field_value(constant_pool_ref, NAME, "constantPoolOop", vec![clazz_ref])
    })?;

    Ok(constant_pool_ref)
}

pub(crate) fn get_nest_host0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let clazz_ref = args[0];
    let nest_host_class_ref = get_nest_host0(clazz_ref)?;
    Ok(vec![nest_host_class_ref])
}
fn get_nest_host0(clazz_ref: i32) -> Result<i32> {
    let clazz_name =
        with_method_area(|method_area| method_area.get_from_reflection_table(clazz_ref))?;

    let nest_host_class_ref = strip_nest_host(clazz_name.as_str())
        .map(|name| with_method_area(|method_area| method_area.load_reflection_class(name)))
        .unwrap_or(Ok(clazz_ref));

    nest_host_class_ref
}

pub(crate) fn is_record0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let clazz_ref = args[0];
    let record = is_record0(clazz_ref)?;
    Ok(vec![if record { 1 } else { 0 }])
}
fn is_record0(clazz_ref: i32) -> Result<bool> {
    let rc = with_method_area(|method_area| {
        let name = method_area.get_from_reflection_table(clazz_ref)?;
        let rc = method_area.get(&name)?;
        Ok::<_, Error>(rc)
    })?;

    let record = rc.class_modifiers().contains(ClassModifier::Final)
        && rc
            .parent()
            .as_ref()
            .map_or(false, |p| p == "java/lang/Record");
    Ok(record)
}
