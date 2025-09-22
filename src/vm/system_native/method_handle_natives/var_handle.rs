use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::with_heap_read_lock;

pub(crate) fn var_handle_set(handle_ref: i32, args_to_set: &[i32]) -> Result<()> {
    let name = with_heap_read_lock(|h| h.get_instance_name(handle_ref))?;

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
    } else {
        return Err(crate::vm::error::Error::new_execution(&format!(
            "Unsupported VarHandle type: {name}"
        )));
    }

    Ok(())
}

pub(crate) fn var_handle_get(handle_ref: i32, args_to_get: &[i32]) -> Result<Vec<i32>> {
    let name = with_heap_read_lock(|h| h.get_instance_name(handle_ref))?;

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
    } else {
        Err(crate::vm::error::Error::new_execution(&format!(
            "Unsupported VarHandle type: {name}"
        )))
    }
}
