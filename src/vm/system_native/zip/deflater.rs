use crate::vm::commons::auto_dash_map::auto_dash_map::AutoDashMap;
use crate::vm::commons::auto_dash_map::auto_dash_map_i64::AutoDashMapI64;
use crate::vm::error::{Error, Result};
use crate::vm::heap::heap::with_heap_write_lock;
use crate::vm::helper::{i32toi64, i64_to_vec};
use crate::vm::system_native::zip::common::DEFAULT_WINDOW_BITS;
use miniz_oxide::deflate::core::{create_comp_flags_from_zip_params, CompressorOxide};
use miniz_oxide::deflate::stream::deflate;
use miniz_oxide::{MZError, MZFlush, MZStatus, StreamResult};
use std::sync::LazyLock;

static REGISTRY: LazyLock<AutoDashMapI64<CompressorOxide>> =
    LazyLock::new(|| AutoDashMapI64::new(1));

pub(crate) fn java_util_zip_deflater_init_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let level = args[0];
    let strategy = args[1];
    let nowrap = args[2] != 0;
    let addr = deflater_init(level, strategy, nowrap)?;

    Ok(i64_to_vec(addr))
}
fn deflater_init(level: i32, strategy: i32, nowrap: bool) -> Result<i64> {
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

pub(crate) fn java_util_zip_deflater_deflate_bytes_bytes_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _this_obj_ref = args[0];
    let addr = i32toi64(args[2], args[1]);
    let input_array_ref = args[3];
    let input_off = args[4];
    let input_len = args[5];
    let output_array_ref = args[6];
    let output_off = args[7];
    let output_len = args[8];
    let flush = args[9];
    let _params = args[10]; // carries the compression level and strategy, not used here

    let res = deflater_deflate_bytes_bytes(
        addr,
        input_array_ref,
        input_off,
        input_len,
        output_array_ref,
        output_off,
        output_len,
        flush,
    )?;

    Ok(i64_to_vec(res))
}

fn deflater_deflate_bytes_bytes(
    addr: i64,
    input_array_ref: i32,
    input_off: i32,
    input_len: i32,
    output_array_ref: i32,
    output_off: i32,
    output_len: i32,
    flush: i32,
) -> Result<i64> {
    let mut entry = REGISTRY
        .get_mut(addr)
        .ok_or_else(|| Error::new_execution("ouch"))?;
    let comp = entry.value_mut();

    let stream_result = with_heap_write_lock(|h| {
        let input = {
            let input_array = h.get_entire_raw_data(input_array_ref)?;
            input_array[input_off as usize..(input_off + input_len) as usize].to_vec()
        };

        let mut output_array = h.get_entire_raw_data_mut(output_array_ref)?;
        let output = &mut output_array[output_off as usize..(output_off + output_len) as usize];
        let stream_result = deflate(comp, &input, output, MZFlush::new(flush)?);

        Ok::<StreamResult, Error>(stream_result)
    })?;

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
    };

    // Pack into jlong like OpenJDK java code assumes
    Ok(((input_used as i64) & 0x7FFF_FFFF)
        | (((output_used as i64) & 0x7FFF_FFFF) << 31)
        | ((finished as i64) << 62))
}

pub(crate) fn java_util_zip_deflater_end_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let addr = i32toi64(args[1], args[0]);
    deflater_end(addr)?;

    Ok(vec![])
}
fn deflater_end(addr: i64) -> Result<()> {
    REGISTRY.remove(addr).ok_or_else(|| {
        Error::new_execution(&format!("Address {addr} does not exist in REGISTRY"))
    })?;

    Ok(())
}
