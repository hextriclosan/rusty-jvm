use crate::vm::execution_engine::executor::Executor;
use crate::vm::execution_engine::static_init::StaticInit;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{clazz_ref, klass};
use crate::vm::jni::jni_invoke::jni_invoke_scalar;
use crate::vm::jni::utils::{decode_method_id, transform_args_to_vec};
use jni_sys::{jboolean, jclass, jmethodID, jobject, jvalue, JNIEnv};

pub(super) extern "system" fn get_object_class(_env: *mut JNIEnv, obj: jobject) -> jclass {
    let instance_name = HEAP
        .get_instance_name(obj as i32)
        .expect("Failed to get instance name from reference");
    clazz_ref(&instance_name).expect("Failed to get class reference from instance name") as jclass
}

pub(super) extern "system" fn is_same_object(
    _env: *mut JNIEnv,
    first: jobject,
    second: jobject,
) -> jboolean {
    (first == second) as jboolean
}

pub(super) extern "system" fn new_object_a(
    _env: *mut JNIEnv,
    clazz: jclass,
    method_id: jmethodID,
    args: *const jvalue,
) -> jobject {
    let target = klass(clazz as i32).expect("Failed to get class from reference");
    StaticInit::initialize_java_class(&target).expect("Failed to initialize class");
    let (declaring_class_ref, method_index) = decode_method_id(method_id as usize);
    let constructor = klass(declaring_class_ref)
        .and_then(|class| class.get_method_by_index(method_index))
        .expect("Failed to get constructor from method ID");
    let arguments = transform_args_to_vec(&constructor, args);
    let context = format!(
        "{}.{}",
        target.this_class_name(),
        constructor.name_signature()
    );
    jni_invoke_scalar(
        Executor::invoke_args_constructor(
            target.this_class_name(),
            constructor.name_signature(),
            &arguments,
            Some(&context),
        ),
        &context,
    )
}
