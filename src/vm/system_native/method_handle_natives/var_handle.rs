use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::i32toi64;
use crate::vm::stack::stack_value::StackValueKind;

pub(crate) fn var_handle_set(handle_ref: i32, args_to_set: &[i32]) -> Result<()> {
    let name = HEAP.get_instance_name(handle_ref)?;

    if name == "java/lang/invoke/VarHandleInts$Array" {
        let array_ref = args_to_set[0];
        let index = args_to_set[1];
        let value = args_to_set[2];

        Executor::invoke_non_static_method(
            &name,
            "set:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)V",
            handle_ref,
            &[array_ref.into(), index.into(), value.into()],
        )?;
        Ok(())
    } else if name == "java/lang/invoke/VarHandleByteArrayAsInts$ArrayHandle" {
        let array_ref = args_to_set[0];
        let index = args_to_set[1];
        let value = args_to_set[2];

        Executor::invoke_non_static_method(
            &name,
            "set:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)V",
            handle_ref,
            &[array_ref.into(), index.into(), value.into()],
        )?;
        Ok(())
    } else if name == "java/lang/invoke/VarHandleByteArrayAsLongs$ArrayHandle" {
        let array_ref = args_to_set[0];
        let index = args_to_set[1];
        let value = i32toi64(args_to_set[3], args_to_set[2]);

        Executor::invoke_non_static_method(
            &name,
            "set:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;IJ)V",
            handle_ref,
            &[array_ref.into(), index.into(), value.into()],
        )?;
        Ok(())
    } else {
        Err(crate::vm::error::Error::new_execution(&format!(
            "var_handle_set - Unsupported VarHandle type: {name}"
        )))
    }
}

pub(crate) fn var_handle_get(handle_ref: i32, args_to_get: &[i32]) -> Result<Vec<i32>> {
    let name = HEAP.get_instance_name(handle_ref)?;

    if name == "java/lang/invoke/VarHandleInts$Array" {
        let array_ref = args_to_get[0];
        let index = args_to_get[1];
        let ret = Executor::invoke_non_static_method(
            &name,
            "get:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I",
            handle_ref,
            &[array_ref.into(), index.into()],
        )?;
        Ok(ret)
    } else if name == "java/lang/invoke/VarHandleByteArrayAsShorts$ArrayHandle" {
        let array_ref = args_to_get[0];
        let index = args_to_get[1];
        let ret = Executor::invoke_non_static_method(
            &name,
            "get:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)S",
            handle_ref,
            &[array_ref.into(), index.into()],
        )?;
        Ok(ret)
    } else if name == "java/lang/invoke/VarHandleByteArrayAsInts$ArrayHandle" {
        let array_ref = args_to_get[0];
        let index = args_to_get[1];
        let ret = Executor::invoke_non_static_method(
            &name,
            "get:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)I",
            handle_ref,
            &[array_ref.into(), index.into()],
        )?;
        Ok(ret)
    } else if name == "java/lang/invoke/VarHandleByteArrayAsLongs$ArrayHandle" {
        let array_ref = args_to_get[0];
        let index = args_to_get[1];
        let ret = Executor::invoke_non_static_method(
            &name,
            "get:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;I)J",
            handle_ref,
            &[array_ref.into(), index.into()],
        )?;
        Ok(ret)
    } else {
        Err(crate::vm::error::Error::new_execution(&format!(
            "var_handle_get - Unsupported VarHandle type: {name}"
        )))
    }
}

pub(crate) fn var_handle_compare_and_set(
    handle_ref: i32,
    args_to_process: &[i32],
) -> Result<Vec<i32>> {
    let name = HEAP.get_instance_name(handle_ref)?;

    let mut all_args = vec![handle_ref];
    all_args.extend_from_slice(args_to_process);
    let all_args = all_args
        .into_iter()
        .map(|a| a.into())
        .collect::<Vec<StackValueKind>>();
    if name == "java/lang/invoke/VarHandleReferences$FieldInstanceReadWrite" {
        let ret = Executor::invoke_static_method(
            &name,
            "compareAndSet:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Z",
            &all_args,
        )?;
        Ok(ret)
    } else if name == "java/lang/invoke/VarHandleInts$FieldInstanceReadWrite" {
        // e.g. `ForkJoinTask.status`, updated via `STATUS.compareAndSet(this, s, s | flags)`.
        let ret = Executor::invoke_static_method(
            &name,
            "compareAndSet:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)Z",
            &all_args,
        )?;
        Ok(ret)
    } else if name == "java/lang/invoke/VarHandleBooleans$FieldInstanceReadWrite" {
        // `ForkJoinPool` relies on this path. The JDK routes `compareAndSetBoolean` through
        // `compareAndSetByte`, which word-aligns the offset (`offset & ~3`) and masks in a byte; this
        // is correct because field offsets are 4-byte aligned per slot (see `FIELD_OFFSET_SCALE`), so
        // each field owns its word and a sub-word CAS never aliases a neighbour.
        let ret = Executor::invoke_static_method(
            &name,
            "compareAndSet:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;ZZ)Z",
            &all_args,
        )?;
        Ok(ret)
    } else {
        Err(crate::vm::error::Error::new_execution(&format!(
            "var_handle_compare_and_set - Unsupported VarHandle type: {name}"
        )))
    }
}

/// Backs the signature-polymorphic `VarHandle.compareAndExchange`. Like
/// [`var_handle_compare_and_set`] but the guard method returns the *witness* value (the value found
/// in the field, equal to `expected` on success) instead of a boolean. Used by e.g.
/// `AtomicReference.compareAndExchange` / `updateAndGet`.
pub(crate) fn var_handle_compare_and_exchange(
    handle_ref: i32,
    args_to_process: &[i32],
) -> Result<Vec<i32>> {
    let name = HEAP.get_instance_name(handle_ref)?;

    let mut all_args = vec![handle_ref];
    all_args.extend_from_slice(args_to_process);
    let all_args = all_args
        .into_iter()
        .map(|a| a.into())
        .collect::<Vec<StackValueKind>>();
    if name == "java/lang/invoke/VarHandleReferences$FieldInstanceReadWrite" {
        Executor::invoke_static_method(
            &name,
            "compareAndExchange:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &all_args,
        )
    } else if name == "java/lang/invoke/VarHandleInts$FieldInstanceReadWrite" {
        Executor::invoke_static_method(
            &name,
            "compareAndExchange:(Ljava/lang/invoke/VarHandle;Ljava/lang/Object;II)I",
            &all_args,
        )
    } else {
        // Booleans are omitted only because nothing currently reaches this path; the aligned-offset
        // layout (`FIELD_OFFSET_SCALE`) makes a boolean branch safe to add here like the ones above.
        Err(crate::vm::error::Error::new_execution(&format!(
            "var_handle_compare_and_exchange - Unsupported VarHandle type: {name}"
        )))
    }
}
