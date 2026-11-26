use jclassfile::class_file::{parse, parse_unchecked};

const TRIVIAL: &[u8] = include_bytes!("test_data/Trivial.class");

/// A class file that fails a single check of format checking, as `MalformedClassFileGenerator` of
/// `utils/classfile_generators` generates it. HotSpot rejects each of them with `ClassFormatError`.
macro_rules! malformed {
    ($name:literal) => {
        include_bytes!(concat!("test_data/malformed/", $name, ".class"))
    };
}

/// A `CONSTANT_Utf8_info` structure.
fn utf8(bytes: &[u8]) -> Vec<u8> {
    let mut entry = vec![0x01];
    entry.extend((bytes.len() as u16).to_be_bytes());
    entry.extend(bytes);

    entry
}

/// The constant pool of class `A`: `Class` #1 named by #2, then two strings for a `SourceFile`
/// attribute.
fn constant_pool() -> Vec<u8> {
    [
        vec![0x07, 0x00, 0x02], //                              1
        utf8(b"A"),             //                              2
        utf8(b"SourceFile"),    //                              3
        utf8(b"A.java"),        //                              4
    ]
    .concat()
}

/// A `SourceFile` attribute: name #3, the given `attribute_length`, then the given `info`.
fn source_file(attribute_length: u32, info: &[u8]) -> Vec<u8> {
    let mut attribute = vec![0x00, 0x03];
    attribute.extend(attribute_length.to_be_bytes());
    attribute.extend(info);

    attribute
}

/// A class file of version 52.0 without interfaces, fields and methods; its constant pool and its
/// only attribute are the given bytes.
fn class_file(constant_pool_count: u16, constant_pool: &[u8], attribute: &[u8]) -> Vec<u8> {
    let mut bytes = vec![0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x00, 0x00, 0x34];
    bytes.extend(constant_pool_count.to_be_bytes());
    bytes.extend(constant_pool);
    bytes.extend([
        0x00, 0x21, // access_flags
        0x00, 0x01, // this_class
        0x00, 0x00, // super_class
        0x00, 0x00, // interfaces_count
        0x00, 0x00, // fields_count
        0x00, 0x00, // methods_count
        0x00, 0x01, // attributes_count
    ]);
    bytes.extend(attribute);

    bytes
}

fn assert_rejected(bytes: &[u8], expected: &str) {
    match parse(bytes) {
        Err(error) => assert_eq!(error.to_string(), format!("Format Error: {expected}")),
        Ok(_) => panic!("Expected a format error: {expected}"),
    }
}

/// Asserts that a class file fails a check that does not depend on format checking of the
/// constant pool.
fn assert_rejected_even_unchecked(bytes: &[u8], expected: &str) {
    assert_rejected(bytes, expected);
    assert_eq!(
        parse_unchecked(bytes).unwrap_err().to_string(),
        format!("Format Error: {expected}")
    );
}

/// Asserts that a class file fails format checking of the constant pool, and nothing else.
fn assert_rejected_unless_unchecked(bytes: &[u8], expected: &str) {
    assert_rejected(bytes, expected);
    assert!(parse_unchecked(bytes).is_ok());
}

#[test]
fn should_accept_well_formed_class_file() {
    let bytes = class_file(5, &constant_pool(), &source_file(2, &[0x00, 0x04]));

    assert_eq!(parse(&bytes).unwrap(), parse_unchecked(&bytes).unwrap());
}

#[test]
fn should_reject_wrong_magic_number() {
    assert_rejected_even_unchecked(
        malformed!("WrongMagic"),
        "Not a valid class-file: magic number is 0xCAFEBABF (JVMS §4.8)",
    );
}

#[test]
fn should_reject_attribute_of_wrong_length() {
    assert_rejected_even_unchecked(
        malformed!("WrongAttributeLength"),
        "attribute SourceFile: attribute_length is 3, but its content takes 2 byte(s) (JVMS §4.8)",
    );
}

#[test]
fn should_reject_attribute_length_shorter_than_content() {
    assert_rejected_even_unchecked(
        &class_file(5, &constant_pool(), &source_file(1, &[0x00, 0x04])),
        "attribute SourceFile: content does not fit in attribute_length 1 (JVMS §4.8)",
    );
}

#[test]
fn should_reject_truncated_class_file() {
    assert_rejected_even_unchecked(
        malformed!("Truncated"),
        "class file is truncated: 2 byte(s) expected at offset 239, but its length is 240 (JVMS §4.8)",
    );
}

#[test]
fn should_reject_class_file_truncated_anywhere() {
    for len in 0..TRIVIAL.len() {
        let error = parse(&TRIVIAL[..len]).unwrap_err().to_string();

        assert!(
            error.starts_with("Format Error: class file is truncated: "),
            "{len} byte(s) of {}: {error}",
            TRIVIAL.len()
        );
        assert!(parse_unchecked(&TRIVIAL[..len]).is_err());
    }
}

#[test]
fn should_reject_extra_bytes_at_the_end() {
    assert_rejected_even_unchecked(
        malformed!("ExtraBytes"),
        "class file has 2 extra byte(s) at the end (JVMS §4.8)",
    );
}

#[test]
fn should_reject_malformed_utf8_constant() {
    assert_rejected_even_unchecked(
        malformed!("MalformedUtf8"),
        "Utf8 constant at offset 75 is not valid modified UTF-8 (JVMS §4.4.7)",
    );
}

#[test]
fn should_reject_constant_kind_newer_than_class_file_version() {
    assert_rejected_unless_unchecked(
        malformed!("UnsupportedConstantTag"),
        "constant pool entry #8 (MethodType): the tag was first defined in version 51.0 of the class file format, but the class file has major version 50 (JVMS §4.4)",
    );
}

#[test]
fn should_reject_constant_pool_index_to_entry_of_wrong_kind() {
    // `Class` #1 is named by itself, not by a `Utf8` entry
    let mut constant_pool = constant_pool();
    constant_pool[2] = 0x01;

    assert_rejected_unless_unchecked(
        &class_file(5, &constant_pool, &source_file(2, &[0x00, 0x04])),
        "constant pool entry #1 (Class): name_index #1 is not a Utf8 entry (JVMS §4.4.1)",
    );
}

#[test]
fn should_reject_eight_byte_constant_in_the_last_entry() {
    let mut constant_pool = constant_pool();
    constant_pool.extend([0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2A]);

    assert_rejected_unless_unchecked(
        &class_file(6, &constant_pool, &source_file(2, &[0x00, 0x04])),
        "constant pool entry #5: an 8-byte constant takes up two entries, but this is the last one (JVMS §4.4.5)",
    );
}

#[test]
fn should_reject_malformed_class_name() {
    assert_rejected_unless_unchecked(
        malformed!("MalformedClassName"),
        "constant pool entry #11 (Class): \"java.lang/Object\" is not a valid class or interface name (JVMS §4.4.1)",
    );
}

#[test]
fn should_reject_malformed_field_name() {
    assert_rejected_unless_unchecked(
        malformed!("MalformedFieldName"),
        "constant pool entry #13 (NameAndType): \"some.Text\" is not a valid field name (JVMS §4.4.6)",
    );
}

#[test]
fn should_reject_malformed_method_descriptor() {
    assert_rejected_unless_unchecked(
        malformed!("MalformedMethodDescriptor"),
        "constant pool entry #11 (NameAndType): \"(Ljava/lang/String:)V\" is not a valid method descriptor (JVMS §4.4.6)",
    );
}
