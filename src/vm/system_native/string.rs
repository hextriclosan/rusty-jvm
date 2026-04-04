use crate::vm::error::{Error, Result};
use crate::vm::execution_engine::string_pool_helper::StringPoolHelper;
use crate::vm::heap::heap::HEAP;
use crate::vm::method_area::loaded_classes::CLASSES;
use std::sync::LazyLock;

const STRING_CLASS_NAME: &str = "java/lang/String";
const VALUE_FIELD: &str = "value";
const CODER_FIELD: &str = "coder";

static COMPACT_STRINGS: LazyLock<bool> = LazyLock::new(|| {
    let klass = CLASSES
        .get(STRING_CLASS_NAME)
        .expect("Failed to get String class");
    let raw = klass
        .static_field("COMPACT_STRINGS")
        .expect("Failed to get COMPACT_STRINGS field")
        .raw_value()
        .expect("Failed to get COMPACT_STRINGS value")[0];
    raw != 0
});

pub(crate) fn get_utf8_string_by_ref(string_ref: i32) -> Result<String> {
    let (is_latin, array_ref) = get_raw_string_info(string_ref)?;

    let result = if is_latin {
        let data = { HEAP.get_entire_raw_data(array_ref)?.to_vec() };
        String::from_utf8(data)?
    } else {
        let guard = HEAP.get_entire_raw_data(array_ref)?;
        if guard.len() % 2 != 0 {
            return Err(Error::new_native(&format!(
                "Invalid UTF-16 string: uneven number of bytes {:?}",
                guard
            )));
        }
        let utf16 = guard
            .chunks_exact(2)
            .map(|chunk| u16::from_ne_bytes([chunk[0], chunk[1]]))
            .collect::<Vec<_>>();

        String::from_utf16(&utf16)?
    };

    Ok(result)
}

/// Retrieves raw information about a string
/// Returns a tuple containing:
/// - A boolean indicating whether the string is compact (Latin-1) or not (UTF-16)
/// - An integer reference to the underlying character byte-array
pub(crate) fn get_raw_string_info(string_ref: i32) -> Result<(bool, i32)> {
    let array_ref = HEAP.get_object_field_value(string_ref, STRING_CLASS_NAME, VALUE_FIELD)?[0];
    // todo: re-check race condition between calls
    let coder = HEAP.get_object_field_value(string_ref, STRING_CLASS_NAME, CODER_FIELD)?[0];
    let is_latin = *COMPACT_STRINGS && coder == 0;

    Ok((is_latin, array_ref))
}

pub(crate) fn intern_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let reference = intern(args[0])?;
    Ok(vec![reference])
}
fn intern(reference: i32) -> Result<i32> {
    let string = get_utf8_string_by_ref(reference)?;

    StringPoolHelper::get_string(&string)
}
