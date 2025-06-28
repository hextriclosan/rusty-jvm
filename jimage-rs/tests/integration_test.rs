use assert_matches::assert_matches;
use jimage_rs::error::JImageError;
use jimage_rs::JImage;
use rstest::rstest;
use std::path::PathBuf;

const WITHOUT_PARENT_EXPECTED: &[u8] =
    include_bytes!("test_data/mods/java.base/without_parent.txt");
const WITHOUT_EXT_EXPECTED: &[u8] =
    include_bytes!("test_data/mods/java.base/java/lang/without_ext");
const HANK_EXPECTED: &[u8] = include_bytes!("test_data/mods/java.base/java/lang/hank.txt");

#[rstest]
#[case::non_compressed_little_endian("tests/test_data/lib/non-compressed_little-endian.jimage")]
#[case::non_compressed_big_endian("tests/test_data/lib/non-compressed_big-endian.jimage")]
#[case::compressed_little_endian("tests/test_data/lib/compressed_little-endian.jimage")]
#[case::compressed_big_endian("tests/test_data/lib/compressed_big-endian.jimage")]
fn should_read_jimages(#[case] path: &str) {
    let image = JImage::open(path).expect("Failed to read jimage file");

    let without_parent_actual = image.find_resource("/java.base/without_parent.txt");
    let without_ext_actual = image.find_resource("/java.base/java/lang/without_ext");
    let hank_actual = image.find_resource("/java.base/java/lang/hank.txt");

    assert_eq!(
        without_parent_actual.unwrap().unwrap(),
        WITHOUT_PARENT_EXPECTED
    );
    assert_eq!(without_ext_actual.unwrap().unwrap(), WITHOUT_EXT_EXPECTED);
    assert_eq!(hank_actual.unwrap().unwrap(), HANK_EXPECTED);
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
    assert_matches!(result, Err(JImageError::Magic { magic, context }) if magic == [0xDE, 0xAD, 0xBE, 0xEF] && context == "header");
}

#[test]
fn should_return_error_if_impossible_to_read() {
    let result = JImage::open("tests/test_data/lib/3bytes-file.jimage");
    assert_matches!(result, Err(JImageError::RawRead { from, to }) if from == 0 && to == 4);
}
