use crate::vm::error::{Error, Result};
use crate::vm::exception::helpers::throw_class_not_found_exception;
use crate::vm::exception::throwing_result::ThrowingResult;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{clazz_ref, klass, strip_nest_host};
use crate::vm::method_area::class_modifiers::ClassModifier;
use crate::vm::method_area::instance_checker::InstanceChecker;
use crate::vm::method_area::primitives_helper::PRIMITIVE_CODE_BY_TYPE;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use crate::{throw_and_return, unwrap_or_return_err, unwrap_result_or_return_default};

const PUBLIC: u16 = 0x00000001;

pub(crate) fn get_superclass_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let current_clazz_ref = args[0];
    let super_clazz_ref = get_superclass(current_clazz_ref)?;

    Ok(vec![super_clazz_ref])
}
fn get_superclass(clazz_ref_: i32) -> Result<i32> {
    let klass = klass(clazz_ref_)?;
    let parent = if !klass.is_interface() {
        klass.parent().clone()
    } else {
        None
    };

    parent
        .map(|parent_name| clazz_ref(&parent_name))
        .unwrap_or(Ok(0))
}

pub(crate) fn get_primitive_class_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let string_ref = args[0];
    let class_ref = get_primitive_class(string_ref)?;
    Ok(vec![class_ref])
}

fn get_primitive_class(string_ref: i32) -> Result<i32> {
    let string_content = get_utf8_string_by_ref(string_ref)?;
    let mapped_class_name = map_primitive_class(&string_content)?;

    clazz_ref(mapped_class_name)
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
    let klass = klass(class_ref)?;
    let class_name = klass.external_name();
    let string_ref = StringPoolHelper::get_string(&class_name)?;

    HEAP.set_object_field_value(class_ref, "java/lang/Class", "name", vec![string_ref])?;

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
    let reflection_ref = match clazz_ref(&internal_name) {
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
    let jc = klass(class_ref)?;
    let interfaces = jc.interfaces();

    let interface_refs = interfaces
        .iter()
        .map(|interface| clazz_ref(interface))
        .collect::<Result<Vec<i32>>>()?;

    let result_ref = HEAP.create_array_with_values("[Ljava/lang/Class;", &interface_refs);
    Ok(result_ref)
}

pub(crate) fn get_declaring_class0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let clazz_ref = args[0];
    let declaring_class_ref = get_declaring_class0(clazz_ref)?;
    Ok(vec![declaring_class_ref])
}
fn get_declaring_class0(class_ref: i32) -> Result<i32> {
    let jc = klass(class_ref)?;
    let declaring_class_ref = jc
        .declaring_class()
        .as_ref()
        .map(|declaring_class| {
            let declaring_class_ref = clazz_ref(declaring_class);
            declaring_class_ref
        })
        .unwrap_or(Ok(0));

    declaring_class_ref
}

pub(crate) fn get_declared_fields0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let class_ref = args[0];
    let public_only = args[1] != 0;
    let fields_ref = get_declared_fields(class_ref, public_only)?;
    Ok(vec![fields_ref])
}

fn get_declared_fields(class_ref: i32, public_only: bool) -> Result<i32> {
    let jc = klass(class_ref)?;

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

    let result_ref = HEAP.create_array_with_values("[Ljava/lang/reflect/Field;", &fields_info_ref);

    Ok(result_ref)
}

pub(crate) fn get_declared_methods0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let class_ref = args[0];
    let public_only = args[1] != 0;
    let methods_ref = get_declared_methods(class_ref, public_only)?;
    Ok(vec![methods_ref])
}
fn get_declared_methods(class_ref: i32, public_only: bool) -> Result<i32> {
    let jc = klass(class_ref)?;
    let java_methods = jc.get_methods();

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

    let result_ref = HEAP.create_array_with_values("[Ljava/lang/reflect/Method;", &method_refs);
    Ok(result_ref)
}

pub(crate) fn get_declared_constructors0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let class_ref = args[0];
    let constructors_ref = get_declared_constructors(class_ref)?;
    Ok(vec![constructors_ref])
}
fn get_declared_constructors(class_ref: i32) -> Result<i32> {
    let jc = klass(class_ref)?;
    let java_methods = jc.get_methods();
    let method_refs = java_methods
        .iter()
        .filter(|java_method| java_method.name() == "<init>")
        .map(|java_method| java_method.reflection_ref())
        .collect::<Result<Vec<_>>>()?;

    let result_ref =
        HEAP.create_array_with_values("[Ljava/lang/reflect/Constructor;", &method_refs);
    Ok(result_ref)
}

pub(crate) fn get_enclosing_method0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let class_ref = args[0];
    let enclosing_method_ref = get_enclosing_method0(class_ref)?;
    Ok(vec![enclosing_method_ref])
}
fn get_enclosing_method0(class_ref: i32) -> Result<i32> {
    let jc = klass(class_ref)?;
    if let Some((class_name, name, descriptor)) = jc.enclosing_method() {
        let class_name_ref = clazz_ref(&class_name)?;
        let name_ref = StringPoolHelper::get_string(&name)?;
        let descriptor_ref = StringPoolHelper::get_string(&descriptor)?;

        let array_ref = HEAP.create_array_with_values(
            "[Ljava/lang/reflect/Method;",
            &[class_name_ref, name_ref, descriptor_ref],
        );
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
fn get_raw_annotations(class_ref: i32) -> Result<i32> {
    let rc = klass(class_ref)?;
    let annotations_raw = rc.annotations_raw().to_owned();
    let annotations_ref = annotations_raw
        .map(|annotations_raw| {
            let vec = annotations_raw
                .iter()
                .map(|x| *x as i32)
                .collect::<Vec<_>>();
            let annotations = HEAP.create_array_with_values("[B", &vec);
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
    let klass = klass(class_ref)?;

    let string_ref = klass
        .this_class_name()
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
    let assignable = klass(assignable_class_ref)?;
    let assignee = klass(assignee_class_ref)?;

    InstanceChecker::checkcast(assignee.this_class_name(), assignable.this_class_name())
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

    let klass = klass(this_class_ref)?;
    let class_name_to_check = HEAP.get_instance_name(obj_ref_to_check)?;

    InstanceChecker::checkcast(&class_name_to_check, klass.this_class_name())
}

pub(crate) fn get_constant_pool_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let clazz_ref = args[0];
    let constant_pool_ref = get_constant_pool(clazz_ref)?;
    Ok(vec![constant_pool_ref])
}
fn get_constant_pool(clazz_ref: i32) -> Result<i32> {
    const NAME: &'static str = "jdk/internal/reflect/ConstantPool";
    let constant_pool_ref = Executor::invoke_default_constructor(NAME)?;
    HEAP.set_object_field_value(constant_pool_ref, NAME, "constantPoolOop", vec![clazz_ref])?;

    Ok(constant_pool_ref)
}

pub(crate) fn get_nest_host0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let clazz_ref = args[0];
    let nest_host_class_ref = get_nest_host0(clazz_ref)?;
    Ok(vec![nest_host_class_ref])
}
fn get_nest_host0(class_ref: i32) -> Result<i32> {
    let klass = klass(class_ref)?;

    let nest_host_class_ref = strip_nest_host(klass.this_class_name())
        .map(|name| clazz_ref(&name))
        .unwrap_or(Ok(class_ref));

    nest_host_class_ref
}

pub(crate) fn is_record0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let clazz_ref = args[0];
    let record = is_record0(clazz_ref)?;
    Ok(vec![if record { 1 } else { 0 }])
}
fn is_record0(clazz_ref: i32) -> Result<bool> {
    let rc = klass(clazz_ref)?;

    let record = rc.class_modifiers().contains(ClassModifier::Final)
        && rc
            .parent()
            .as_ref()
            .map_or(false, |p| p == "java/lang/Record");
    Ok(record)
}
