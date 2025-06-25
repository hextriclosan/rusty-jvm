use assert_matches::assert_matches;
use jimage_rs::error::JImageError;
use jimage_rs::JImage;
use std::borrow::Cow;
use std::path::PathBuf;

const WITHOUT_PARENT_EXPECTED: &[u8] =
    include_bytes!("test_data/mods/java.base/without_parent.txt");
const WITHOUT_EXT_EXPECTED: &[u8] =
    include_bytes!("test_data/mods/java.base/java/lang/without_ext");
const HANK_EXPECTED: &[u8] = include_bytes!("test_data/mods/java.base/java/lang/hank.txt");

#[cfg(not(target_endian = "big"))] // todo: fix this test for big endian systems
#[test]
fn should_read_uncompressed_little_endian_jimage() {
    let image = JImage::open("tests/test_data/lib/non-compressed_little-endian.jimage")
        .expect("Failed to read jimage file");

    let without_parent_actual = image.find_resource("/java.base/without_parent.txt");
    let without_ext_actual = image.find_resource("/java.base/java/lang/without_ext");
    let hank_actual = image.find_resource("/java.base/java/lang/hank.txt");

    assert_matches!(
        without_parent_actual,
        Ok(Some(Cow::Borrowed(WITHOUT_PARENT_EXPECTED)))
    );
    assert_matches!(
        without_ext_actual,
        Ok(Some(Cow::Borrowed(WITHOUT_EXT_EXPECTED)))
    );
    assert_matches!(hank_actual, Ok(Some(Cow::Borrowed(HANK_EXPECTED))));
}

#[cfg(not(target_endian = "little"))] // todo: merge me with prevoius test
#[test]
fn should_read_uncompressed_big_endian_jimage() {
    let image = JImage::open("tests/test_data/lib/non-compressed_big-endian.jimage")
        .expect("Failed to read jimage file");

    let without_parent_actual = image.find_resource("/java.base/without_parent.txt");
    let without_ext_actual = image.find_resource("/java.base/java/lang/without_ext");
    let hank_actual = image.find_resource("/java.base/java/lang/hank.txt");

    assert_matches!(
        without_parent_actual,
        Ok(Some(Cow::Borrowed(WITHOUT_PARENT_EXPECTED)))
    );
    assert_matches!(
        without_ext_actual,
        Ok(Some(Cow::Borrowed(WITHOUT_EXT_EXPECTED)))
    );
    assert_matches!(hank_actual, Ok(Some(Cow::Borrowed(HANK_EXPECTED))));
}

#[test]
fn should_return_error_if_file_does_not_exist() {
    let path = PathBuf::from("non_existent.jimage");
    let result = JImage::open(&path);

    assert_matches!(result, Err(JImageError::Io { .. }));
}

#[test]
fn should_return_error_if_magic_is_non_valid() {
    let result = JImage::open("tests/test_data/lib/non-valid-magic.jimage");

    let non_valid_magic = if cfg!(target_endian = "little") {
        0xDEADBEEF
    } else {
        0xEFBEADDE
    };

    assert_matches!(result, Err(JImageError::Magic ( magic )) if magic == non_valid_magic);
}
