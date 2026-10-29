use assert_matches::assert_matches;
use jimage_rs::error::JImageError;
use jimage_rs::JImage;
use rstest::rstest;
use std::collections::BTreeSet;
use std::path::PathBuf;

const WITHOUT_PARENT_EXPECTED: &[u8] =
    include_bytes!("test_data/mods/java.base/without_parent.txt");
const WITHOUT_EXT_EXPECTED: &[u8] =
    include_bytes!("test_data/mods/java.base/java/lang/without_ext");
const HANK_EXPECTED: &[u8] = include_bytes!("test_data/mods/java.base/java/lang/hank.txt");

const PREVIEW_MODULE_HANK_EXPECTED: &[u8] =
    include_bytes!("test_data/mods_preview/java.base/java/lang/hank.txt");
const WITH_PREVIEW_REGULAR_EXPECTED: &[u8] =
    include_bytes!("test_data/mods_preview/java.base/java/lang/with_preview.txt");
const WITH_PREVIEW_PREVIEW_EXPECTED: &[u8] =
    include_bytes!("test_data/mods_preview/java.base/META-INF/preview/java/lang/with_preview.txt");
const PREVIEW_ONLY_EXPECTED: &[u8] =
    include_bytes!("test_data/mods_preview/java.base/META-INF/preview/java/lang/preview_only.txt");

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

// jimage 1.1 files, built by JDK 27 from `mods_preview`: their locations carry the PREVIEW_FLAGS
// attribute.
#[rstest]
#[case::non_compressed_little_endian(
    "tests/test_data/lib/v1.1_non-compressed_little-endian.jimage"
)]
#[case::non_compressed_big_endian("tests/test_data/lib/v1.1_non-compressed_big-endian.jimage")]
#[case::compressed_little_endian("tests/test_data/lib/v1.1_compressed_little-endian.jimage")]
#[case::compressed_big_endian("tests/test_data/lib/v1.1_compressed_big-endian.jimage")]
fn should_read_jimages_v1_1(#[case] path: &str) {
    let image = JImage::open(path).expect("Failed to read jimage file");

    // regular lookups resolve the non-preview versions
    let hank_actual = image.find_resource("/java.base/java/lang/hank.txt");
    let with_preview_actual = image.find_resource("/java.base/java/lang/with_preview.txt");
    assert_eq!(hank_actual.unwrap().unwrap(), PREVIEW_MODULE_HANK_EXPECTED);
    assert_eq!(
        with_preview_actual.unwrap().unwrap(),
        WITH_PREVIEW_REGULAR_EXPECTED
    );

    // a preview-only resource has no regular location
    let preview_only_actual = image.find_resource("/java.base/java/lang/preview_only.txt");
    assert!(preview_only_actual.unwrap().is_none());

    // preview versions are ordinary entries under META-INF/preview
    let with_preview_preview_actual =
        image.find_resource("/java.base/META-INF/preview/java/lang/with_preview.txt");
    let preview_only_preview_actual =
        image.find_resource("/java.base/META-INF/preview/java/lang/preview_only.txt");
    assert_eq!(
        with_preview_preview_actual.unwrap().unwrap(),
        WITH_PREVIEW_PREVIEW_EXPECTED
    );
    assert_eq!(
        preview_only_preview_actual.unwrap().unwrap(),
        PREVIEW_ONLY_EXPECTED
    );

    let resource_names = image
        .resource_names_iter()
        .map(|v| {
            let (module, name) = v.expect("Failed to read resource name").get_full_name();
            module + "/" + &name
        })
        .collect::<BTreeSet<_>>();
    let expected = [
        "java.base/META-INF/preview/java/lang/preview_only.txt",
        "java.base/META-INF/preview/java/lang/with_preview.txt",
        "java.base/java/lang/Object.class",
        "java.base/java/lang/hank.txt",
        "java.base/java/lang/with_preview.txt",
        "java.base/module-info.class",
    ]
    .map(String::from)
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(expected, resource_names);
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

#[test]
fn should_return_resource_name() {
    let image = JImage::open("tests/test_data/lib/non-compressed_little-endian.jimage")
        .expect("Failed to read jimage file");
    let resource_names = image
        .resource_names_iter()
        .filter_map(Result::ok)
        .map(|v| {
            let (module, name) = v.get_full_name();
            module + "/" + &name
        })
        .collect::<BTreeSet<_>>();

    let expected = vec![
        "java.base/java/lang/Object.class".into(),
        "java.base/java/lang/hank.txt".into(),
        "java.base/java/lang/without_ext".into(),
        "java.base/module-info.class".into(),
        "java.base/without_parent.txt".into(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(expected, resource_names);
}
