use crate::error::Error;
use crate::heap::heap::with_heap_write_lock;
use nix::unistd::getcwd;

pub fn get_cwd_wrp(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let byte_array_ref = get_cwd()?;

    Ok(vec![byte_array_ref])
}
fn get_cwd() -> crate::error::Result<i32> {
    let path = match getcwd() {
        Ok(path) => path,
        Err(e) => {
            return Err(Error::new_native(&format!(
                "Failed to get current directory: {}",
                e
            )))
        }
    };

    let cwd = path
        .to_string_lossy()
        .chars()
        .map(|c| c as i32)
        .collect::<Vec<_>>();

    // Allocate the byte array in the heap and return its reference
    let array_ref = with_heap_write_lock(|heap| heap.create_array_with_values("[b", &cwd));

    Ok(array_ref)
}
