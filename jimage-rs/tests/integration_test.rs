use jimage_rs::JImage;

#[test]
fn should_read_jimage() {
    let image =
        JImage::new("tests/test_data/jimages/lib/modules").expect("Failed to read jimage file");

    let actual_resource = image
        .find_resource("/java.base/resource.txt")
        .expect("Failed to find resource");
    assert!(actual_resource.is_some(), "Resource not found");

    let expected_resource = include_bytes!("test_data/mods/java.base/resource.txt");

    assert_eq!(
        *actual_resource.unwrap(),
        *expected_resource,
        "Resource content does not match"
    );
}
