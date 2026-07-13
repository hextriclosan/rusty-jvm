use crate::vm::commons::auto_dash_map::auto_dash_map::AutoDashMap;
use crate::vm::commons::auto_dash_map::auto_dash_map_i64::AutoDashMapI64;
use crate::vm::error::{Error, Result};
use crate::vm::heap::heap::HEAP;
use crate::vm::system_native::zip::common::DEFAULT_WINDOW_BITS;
use miniz_oxide::deflate::core::{create_comp_flags_from_zip_params, CompressorOxide};
use miniz_oxide::deflate::stream::deflate;
use miniz_oxide::{MZError, MZFlush, MZStatus, StreamResult};
use std::sync::LazyLock;

static REGISTRY: LazyLock<AutoDashMapI64<CompressorOxide>> =
    LazyLock::new(|| AutoDashMapI64::new(1));

/// `java.util.zip.Deflater.init(IIZ)J`
pub(crate) fn init(level: i32, strategy: i32, nowrap: bool) -> Result<i64> {
    let deflate_flags = create_comp_flags_from_zip_params(
        level,
        if nowrap {
            -DEFAULT_WINDOW_BITS
        } else {
            DEFAULT_WINDOW_BITS
        },
        strategy,
    );
    let compress = CompressorOxide::new(deflate_flags);

    let inserted_key = REGISTRY.insert_auto(compress);
    Ok(inserted_key)
}

/// `java.util.zip.Deflater.deflateBytesBytes(J[BII[BIIII)J`
pub(crate) fn deflate_bytes_bytes(
    _this: i32,
    addr: i64,
    input_array_ref: i32,
    input_off: i32,
    input_len: i32,
    output_array_ref: i32,
    output_off: i32,
    output_len: i32,
    flush: i32,
    _params: i32, // carries the compression level and strategy, not used here
) -> Result<i64> {
    let mut entry = REGISTRY.get_mut(addr).ok_or_else(|| {
        Error::new_execution(&format!(
            "Deflater not found in registry for address: {}",
            addr
        ))
    })?;
    let comp = entry.value_mut();

    let input = {
        let input_array = HEAP.get_entire_raw_data(input_array_ref)?;
        input_array[input_off as usize..(input_off + input_len) as usize].to_vec()
    };

    let mut output_array = HEAP.get_entire_raw_data_mut(output_array_ref)?;
    let output = &mut output_array[output_off as usize..(output_off + output_len) as usize];
    let stream_result = deflate(comp, &input, output, MZFlush::new(flush)?);

    check_deflate_status(stream_result)
}

/// Returns the packed jlong status like HotSpot's checkDeflateStatus
fn check_deflate_status(stream_result: StreamResult) -> Result<i64> {
    let input_used = stream_result.bytes_consumed;
    let output_used = stream_result.bytes_written;
    let mut finished = 0;

    let status = stream_result.status;
    match status {
        Ok(MZStatus::Ok) => {}
        Ok(MZStatus::StreamEnd) => {
            finished = 1;
        }
        Err(MZError::Buf) => {
            // java code will try to handle this by resizing buffer (probably)
        }
        Ok(MZStatus::NeedDict)
        | Err(MZError::ErrNo)
        | Err(MZError::Stream)
        | Err(MZError::Data)
        | Err(MZError::Mem)
        | Err(MZError::Version)
        | Err(MZError::Param) => {
            // throw InternalError here
            return Err(Error::new_execution(&format!("Deflate error: {status:?}")));
        }
        Err(_) => {
            // throw InternalError here
            return Err(Error::new_execution(&format!(
                "Unknown deflate error: {status:?}"
            )));
        }
    };

    // Pack into jlong like OpenJDK java code assumes
    Ok(((input_used as i64) & 0x7FFF_FFFF)
        | (((output_used as i64) & 0x7FFF_FFFF) << 31)
        | ((finished as i64) << 62))
}

/// `java.util.zip.Deflater.end(J)V`
pub(crate) fn end(addr: i64) -> Result<()> {
    REGISTRY.remove(addr).ok_or_else(|| {
        Error::new_execution(&format!("Address {addr} does not exist in REGISTRY"))
    })?;

    Ok(())
}
