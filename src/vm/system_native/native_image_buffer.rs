use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::method_area::method_area::with_method_area;

pub(crate) fn get_native_map_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let image_path_ref = args[0];
    let byte_buffer_ref = get_native_map(image_path_ref)?;
    Ok(vec![byte_buffer_ref])
}
fn get_native_map(_image_path_ref: i32) -> Result<i32> {
    // fixme: check _image_path_ref is $JAVA_HOME/lib/modules
    let (address, capacity) = with_method_area(|a| {
        let raw_jimage = a.jimage_raw();
        (raw_jimage.as_ptr() as usize as i64, raw_jimage.len() as i64)
    });

    let byte_buffer_ref = Executor::invoke_args_constructor(
        "java/nio/DirectByteBuffer",
        "<init>:(JJ)V",
        &[address.into(), capacity.into()],
        Some("native image buffer creation"),
    )?;

    Ok(byte_buffer_ref)
}
