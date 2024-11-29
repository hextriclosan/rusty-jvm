use crate::execution_engine::string_pool_helper::StringPoolHelper;
use crate::heap::heap::with_heap_read_lock;

const STRING_CLASS_NAME: &str = "java/lang/String";
const VALUE_FIELD: &str = "value";
const CODER_FIELD: &str = "coder";

pub(crate) fn get_utf8_string_by_ref(string_ref: i32) -> crate::error::Result<String> {
    let array_ref = with_heap_read_lock(|heap| {
        heap.get_object_field_value(string_ref, STRING_CLASS_NAME, VALUE_FIELD)
    })?;
    let array_ref = array_ref[0];
    // todo: re-check race condition between calls
    let coder = with_heap_read_lock(|heap| {
        heap.get_object_field_value(string_ref, STRING_CLASS_NAME, CODER_FIELD)
    })?;
    let coder = coder[0] as u8;

    let array_content = with_heap_read_lock(|heap| heap.get_entire_array(array_ref))?;
    let array_content = array_content
        .get_entire_value()
        .iter()
        .map(|v| v[0] as u8)
        .collect::<Vec<u8>>();

    const CODER_LATIN1: u8 = 0;
    const CODER_UTF16: u8 = 1;
    let result = match coder {
        CODER_LATIN1 => String::from_utf8(array_content)?,
        CODER_UTF16 => {
            if array_content.len() % 2 != 0 {
                return Err(crate::error::Error::new_native(&format!(
                    "Invalid UTF-16 string: uneven number of bytes {:?}",
                    array_content
                )));
            }

            let utf16 = array_content
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .collect::<Vec<_>>();

            String::from_utf16(&utf16)?
        }
        _ => {
            return Err(crate::error::Error::new_native(&format!(
                "Unsupported coder: {}",
                coder
            )));
        }
    };

    Ok(result)
}

pub(crate) fn intern_wrp(args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let reference = intern(args[0])?;
    Ok(vec![reference])
}
fn intern(reference: i32) -> crate::error::Result<i32> {
    let string = get_utf8_string_by_ref(reference)?;

    StringPoolHelper::get_string(string)
}
