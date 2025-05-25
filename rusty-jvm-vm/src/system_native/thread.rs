use crate::method_area::method_area::with_method_area;

pub(crate) fn current_thread_wrp(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let result = current_thread()?;

    Ok(vec![result])
}
fn current_thread() -> crate::error::Result<i32> {
    let thread_id = with_method_area(|method_area| {
        method_area.system_thread_id() // since we do not spawn threads, primordial system thread returned here
    })?;
    Ok(thread_id)
}
