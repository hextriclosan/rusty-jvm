use crate::vm::error::Result;
use crate::vm::heap::heap::with_heap_write_lock;

pub(crate) fn get_all_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    let result_ref = get_all()?;
    Ok(vec![result_ref])
}

fn get_all() -> Result<i32> {
    // Return an empty array of NetworkInterface objects
    // This is a minimal implementation that returns no network interfaces
    let network_interfaces: Vec<i32> = vec![];

    let result_ref = with_heap_write_lock(|heap| {
        heap.create_array_with_values("[Ljava/net/NetworkInterface;", &network_interfaces)
    });

    Ok(result_ref)
}
