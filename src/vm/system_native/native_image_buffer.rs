use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::method_area::method_area::with_method_area;

/// `jdk.internal.jimage.NativeImageBuffer.getNativeMap(Ljava/lang/String;)Ljava/nio/ByteBuffer;`
pub(crate) fn get_native_map(_this: i32, _image_path_ref: i32) -> Result<i32> {
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
