use crate::vm::commons::auto_dash_map::auto_dash_map::AutoDashMap;
use crate::vm::commons::auto_dash_map::auto_dash_map_i64::AutoDashMapI64;
use crate::vm::error::{Error, Result};
use crate::vm::heap::heap::with_heap_write_lock;
use crate::vm::helper::{i32toi64, i64_to_vec};
use crate::vm::system_native::zip::common::DEFAULT_WINDOW_BITS;
use miniz_oxide::inflate::stream::{inflate, InflateState};
use miniz_oxide::{DataFormat, MZError, MZFlush, MZStatus, StreamResult};
use std::sync::LazyLock;

static REGISTRY: LazyLock<AutoDashMapI64<InflateState>> = LazyLock::new(|| AutoDashMapI64::new(1));

pub(crate) fn java_util_zip_inflater_initids_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![])
}

pub(crate) fn java_util_zip_inflater_init_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let nowrap = args[0] != 0;
    let addr = inflater_init(nowrap);
    Ok(i64_to_vec(addr))
}
fn inflater_init(nowrap: bool) -> i64 {
    let inflate_state = InflateState::new(DataFormat::from_window_bits(if nowrap {
        -DEFAULT_WINDOW_BITS
    } else {
        DEFAULT_WINDOW_BITS
    }));

    let addr = REGISTRY.insert_auto(inflate_state);
    addr
}

pub(crate) fn java_util_zip_inflater_inflate_bytes_bytes_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _this_obj_ref = args[0];
    let addr = i32toi64(args[2], args[1]);
    let input_array_ref = args[3];
    let input_off = args[4];
    let input_len = args[5];
    let output_array_ref = args[6];
    let output_off = args[7];
    let output_len = args[8];

    let res = inflater_inflate_bytes_bytes(
        addr,
        input_array_ref,
        input_off,
        input_len,
        output_array_ref,
        output_off,
        output_len,
    )?;

    Ok(i64_to_vec(res))
}

fn inflater_inflate_bytes_bytes(
    addr: i64,
    input_array_ref: i32,
    input_off: i32,
    input_len: i32,
    output_array_ref: i32,
    output_off: i32,
    output_len: i32,
) -> Result<i64> {
    let mut entry = REGISTRY.get_mut(addr).ok_or_else(|| {
        Error::new_execution(&format!(
            "Inflater not found in registry for address: {}",
            addr
        ))
    })?;
    let inflate_state = entry.value_mut();

    let stream_result = with_heap_write_lock(|h| {
        let input = {
            let input_array = h.get_entire_raw_data(input_array_ref)?;
            input_array[input_off as usize..(input_off + input_len) as usize].to_vec()
        };

        let mut output_array = h.get_entire_raw_data_mut(output_array_ref)?;
        let output = &mut output_array[output_off as usize..(output_off + output_len) as usize];
        let stream_result = inflate(inflate_state, &input, output, MZFlush::Sync);

        Ok::<StreamResult, Error>(stream_result)
    })?;

    check_inflate_status(stream_result)
}

fn check_inflate_status(stream_result: StreamResult) -> Result<i64> {
    let mut input_used = 0usize;
    let mut output_used = 0usize;
    let mut finished = 0;
    let mut need_dict = 0;

    let status = stream_result.status;
    match status {
        Ok(MZStatus::Ok) => {
            input_used = stream_result.bytes_consumed;
            output_used = stream_result.bytes_written;
        }
        Ok(MZStatus::StreamEnd) => {
            input_used = stream_result.bytes_consumed;
            output_used = stream_result.bytes_written;
            finished = 1;
        }
        Ok(MZStatus::NeedDict) => {
            input_used = stream_result.bytes_consumed;
            output_used = stream_result.bytes_written;
            need_dict = 1;
        }
        Err(MZError::Buf) => {
            // java code will try to handle this by resizing buffer (probably)
        }
        Err(MZError::Data) => {
            // This is a data error, we should throw DataFormatException
            return Err(Error::new_execution(&format!(
                "data format error in inflater stream_result: {stream_result:?}"
            )));
        }
        Err(MZError::Mem) => {
            // This is a memory error, we should throw OutOfMemoryError
            return Err(Error::new_execution(&format!(
                "memory error in inflater stream_result: {stream_result:?}"
            )));
        }
        Err(MZError::ErrNo)
        | Err(MZError::Stream)
        | Err(MZError::Version)
        | Err(MZError::Param) => {
            // throw InternalError here
            return Err(Error::new_execution(&format!("Inflate error: {status:?}")));
        }
    };

    // Pack into jlong like OpenJDK java code assumes
    Ok(((input_used as i64) & 0x7FFF_FFFF)
        | (((output_used as i64) & 0x7FFF_FFFF) << 31)
        | ((finished as i64) << 62)
        | ((need_dict as i64) << 63))
}

pub(crate) fn java_util_zip_inflater_end_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let addr = i32toi64(args[1], args[0]);
    inflater_end(addr)?;

    Ok(vec![])
}
fn inflater_end(addr: i64) -> Result<()> {
    REGISTRY.remove(addr).ok_or_else(|| {
        Error::new_execution(&format!("Address {addr} does not exist in REGISTRY"))
    })?;

    Ok(())
}
