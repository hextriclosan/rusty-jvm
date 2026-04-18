use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::i32toi64;
use crate::vm::stack::stack_value::StackValueKind;

/// `set`-family polymorphic VarHandle accessors (`set`, `setVolatile`,
/// `setRelease`, `setOpaque`). The Java-level signatures only differ by
/// the receiver/value type – not by access mode – so we forward to the
/// generated helper of the same `method_name` on the concrete VarHandle
/// subtype.
pub(crate) fn var_handle_set(
    handle_ref: i32,
    args_to_set: &[i32],
    method_name: &str,
) -> Result<()> {
    let name = HEAP.get_instance_name(handle_ref)?;

    if name == "java/lang/invoke/VarHandleInts$Array" {
        let array_ref = args_to_set[0];
        let index = args_to_set[1];
        let value = args_to_set[2];

        crate::vm::concurrency::block_on_async(Executor::invoke_non_static_method(
            &name,
            &format!("{method_name}:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)V"),
            handle_ref,
            &[array_ref.into(), index.into(), value.into()],
        ))?;
        Ok(())
    } else if name == "java/lang/invoke/VarHandleByteArrayAsInts$ArrayHandle" {
        let array_ref = args_to_set[0];
        let index = args_to_set[1];
        let value = args_to_set[2];

        crate::vm::concurrency::block_on_async(Executor::invoke_non_static_method(
            &name,
            &format!("{method_name}:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)V"),
            handle_ref,
            &[array_ref.into(), index.into(), value.into()],
        ))?;
        Ok(())
    } else if name == "java/lang/invoke/VarHandleByteArrayAsLongs$ArrayHandle" {
        let array_ref = args_to_set[0];
        let index = args_to_set[1];
        let value = i32toi64(args_to_set[3], args_to_set[2]);

        crate::vm::concurrency::block_on_async(Executor::invoke_non_static_method(
            &name,
            &format!("{method_name}:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)V"),
            handle_ref,
            &[array_ref.into(), index.into(), value.into()],
        ))?;
        Ok(())
    } else if name == "java/lang/invoke/VarHandleInts$FieldInstanceReadWrite" {
        let receiver = args_to_set[0];
        let value = args_to_set[1];

        crate::vm::concurrency::block_on_async(Executor::invoke_non_static_method(
            &name,
            &format!("{method_name}:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)V"),
            handle_ref,
            &[receiver.into(), value.into()],
        ))?;
        Ok(())
    } else if name == "java/lang/invoke/VarHandleLongs$FieldInstanceReadWrite" {
        let receiver = args_to_set[0];
        let value = i32toi64(args_to_set[2], args_to_set[1]);

        crate::vm::concurrency::block_on_async(Executor::invoke_non_static_method(
            &name,
            &format!("{method_name}:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;J)V"),
            handle_ref,
            &[receiver.into(), value.into()],
        ))?;
        Ok(())
    } else if name == "java/lang/invoke/VarHandleReferences$FieldInstanceReadWrite" {
        let receiver = args_to_set[0];
        let value = args_to_set[1];

        crate::vm::concurrency::block_on_async(Executor::invoke_non_static_method(
            &name,
            &format!(
                "{method_name}:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;)V"
            ),
            handle_ref,
            &[receiver.into(), value.into()],
        ))?;
        Ok(())
    } else {
        Err(crate::vm::error::Error::new_execution(&format!(
            "var_handle_{method_name} - Unsupported VarHandle type: {name}"
        )))
    }
}

/// `get`-family polymorphic VarHandle accessors (`get`, `getVolatile`,
/// `getAcquire`, `getOpaque`). Forwarded to the generated helper of the
/// matching name on the concrete VarHandle subtype.
pub(crate) fn var_handle_get(
    handle_ref: i32,
    args_to_get: &[i32],
    method_name: &str,
) -> Result<Vec<i32>> {
    let name = HEAP.get_instance_name(handle_ref)?;

    if name == "java/lang/invoke/VarHandleInts$Array" {
        let array_ref = args_to_get[0];
        let index = args_to_get[1];
        crate::vm::concurrency::block_on_async(Executor::invoke_non_static_method(
            &name,
            &format!("{method_name}:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I"),
            handle_ref,
            &[array_ref.into(), index.into()],
        ))
    } else if name == "java/lang/invoke/VarHandleByteArrayAsShorts$ArrayHandle" {
        let array_ref = args_to_get[0];
        let index = args_to_get[1];
        crate::vm::concurrency::block_on_async(Executor::invoke_non_static_method(
            &name,
            &format!("{method_name}:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)S"),
            handle_ref,
            &[array_ref.into(), index.into()],
        ))
    } else if name == "java/lang/invoke/VarHandleByteArrayAsInts$ArrayHandle" {
        let array_ref = args_to_get[0];
        let index = args_to_get[1];
        crate::vm::concurrency::block_on_async(Executor::invoke_non_static_method(
            &name,
            &format!("{method_name}:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I"),
            handle_ref,
            &[array_ref.into(), index.into()],
        ))
    } else if name == "java/lang/invoke/VarHandleByteArrayAsLongs$ArrayHandle" {
        let array_ref = args_to_get[0];
        let index = args_to_get[1];
        crate::vm::concurrency::block_on_async(Executor::invoke_non_static_method(
            &name,
            &format!("{method_name}:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)J"),
            handle_ref,
            &[array_ref.into(), index.into()],
        ))
    } else if name == "java/lang/invoke/VarHandleInts$FieldInstanceReadWrite" {
        let receiver = args_to_get[0];
        crate::vm::concurrency::block_on_async(Executor::invoke_non_static_method(
            &name,
            &format!("{method_name}:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)I"),
            handle_ref,
            &[receiver.into()],
        ))
    } else if name == "java/lang/invoke/VarHandleLongs$FieldInstanceReadWrite" {
        let receiver = args_to_get[0];
        crate::vm::concurrency::block_on_async(Executor::invoke_non_static_method(
            &name,
            &format!("{method_name}:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)J"),
            handle_ref,
            &[receiver.into()],
        ))
    } else if name == "java/lang/invoke/VarHandleReferences$FieldInstanceReadWrite" {
        let receiver = args_to_get[0];
        crate::vm::concurrency::block_on_async(Executor::invoke_non_static_method(
            &name,
            &format!(
                "{method_name}:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;)Ljava/lang/Object;"
            ),
            handle_ref,
            &[receiver.into()],
        ))
    } else {
        Err(crate::vm::error::Error::new_execution(&format!(
            "var_handle_{method_name} - Unsupported VarHandle type: {name}"
        )))
    }
}

pub(crate) fn var_handle_compare_and_set(
    handle_ref: i32,
    args_to_process: &[i32],
    method_name: &str,
) -> Result<Vec<i32>> {
    dispatch_var_handle_cas(handle_ref, args_to_process, method_name)
}

fn dispatch_var_handle_cas(
    handle_ref: i32,
    args_to_process: &[i32],
    method_name: &str,
) -> Result<Vec<i32>> {
    let name = HEAP.get_instance_name(handle_ref)?;

    let mut all_args = vec![handle_ref];
    all_args.extend_from_slice(args_to_process);
    let all_args = all_args
        .into_iter()
        .map(|a| a.into())
        .collect::<Vec<StackValueKind>>();

    // Generated VarHandle subclasses expose static helpers with the shape
    // `<op>(VarHandle handle, <receiver>, <expected>, <value>) -> Z`. The
    // receiver/value types depend on the concrete subtype.
    let descriptor: Option<&str> = match name.as_str() {
        "java/lang/invoke/VarHandleReferences$FieldInstanceReadWrite" => Some(
            "(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z",
        ),
        "java/lang/invoke/VarHandleInts$FieldInstanceReadWrite" => {
            Some("(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)Z")
        }
        "java/lang/invoke/VarHandleInts$FieldStaticReadWrite" => {
            Some("(Ljava/lang/invoke/VarHandle;II)Z")
        }
        "java/lang/invoke/VarHandleBooleans$FieldInstanceReadWrite" => {
            Some("(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ZZ)Z")
        }
        "java/lang/invoke/VarHandleBooleans$FieldStaticReadWrite" => {
            Some("(Ljava/lang/invoke/VarHandle;ZZ)Z")
        }
        "java/lang/invoke/VarHandleLongs$FieldInstanceReadWrite" => {
            Some("(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;JJ)Z")
        }
        "java/lang/invoke/VarHandleLongs$FieldStaticReadWrite" => {
            Some("(Ljava/lang/invoke/VarHandle;JJ)Z")
        }
        "java/lang/invoke/VarHandleInts$Array" => {
            Some("(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;III)Z")
        }
        "java/lang/invoke/VarHandleLongs$Array" => {
            Some("(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJJ)Z")
        }
        "java/lang/invoke/VarHandleReferences$Array" => Some(
            "(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ILjava/lang/Object;Ljava/lang/Object;)Z",
        ),
        _ => None,
    };

    let descriptor = descriptor.ok_or_else(|| {
        crate::vm::error::Error::new_execution(&format!(
            "var_handle_{method_name} - Unsupported VarHandle type: {name}"
        ))
    })?;

    let full_signature = format!("{method_name}:{descriptor}");
    crate::vm::concurrency::block_on_async(Executor::invoke_static_method(
        &name,
        &full_signature,
        &all_args,
    ))
}
