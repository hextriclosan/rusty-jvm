use jclassfile::attributes::Attribute::*;
use jclassfile::attributes::ElementValue::*;
use jclassfile::attributes::StackMapFrame::*;
use jclassfile::attributes::VerificationTypeInfo::*;
use jclassfile::attributes::*;
use jclassfile::class_file::{parse, ClassFile, ClassFlags};
use jclassfile::constant_pool::ConstantPool::*;
use jclassfile::fields::{FieldFlags, FieldInfo};
use jclassfile::methods::{MethodFlags, MethodInfo};

#[test]
fn should_load_and_parse() {
    let bytes = include_bytes!("test_data/Trivial.class");
    let actual_class_file = parse(bytes).unwrap();

    let expected_class_file = ClassFile::new(
        0xCAFEBABE,
        0,
        61,
        vec![
            Empty, //                               0
            Methodref { //                          1
                class_index: 2,
                name_and_type_index: 3,
            },
            Class { //                              2
                name_index: 4,
            },
            NameAndType { //                        3
                name_index: 5,
                descriptor_index: 6,
            },
            Utf8 { //                               4
                value: "java/lang/Object".into()
            },
            Utf8 { //                               5
                value: "<init>".into()
            },
            Utf8 { //                               6
                value: "()V".into()
            },
            Fieldref { //                           7
                class_index: 8,
                name_and_type_index: 9,
            },
            Class { //                              8
                name_index: 10,
            },
            NameAndType { //                        9
                name_index: 11,
                descriptor_index: 12,
            },
            Utf8 { //                               10
                value: "Trivial".into()
            },
            Utf8 { //                               11
                value: "someText".into()
            },
            Utf8 { //                               12
                value: "Ljava/lang/String;".into()
            },
            Methodref { //                          13
                class_index: 8,
                name_and_type_index: 14,
            },
            NameAndType { //                        14
                name_index: 5,
                descriptor_index: 15,
            },
            Utf8 { //                               15
                value: "(Ljava/lang/String;)V".into()
            },
            InvokeDynamic { //                      16
                bootstrap_method_attr_index: 0,
                name_and_type_index: 17,
            },
            NameAndType { //                        17
                name_index: 18,
                descriptor_index: 19,
            },
            Utf8 { //                               18
                value: "run".into()
            },
            Utf8 { //                               19
                value: "()Ljava/lang/Runnable;".into()
            },
            Class { //                              20
                name_index: 21,
            },
            Utf8 { //                               21
                value: "java/lang/Runnable".into()
            },
            Utf8 { //                               22
                value: "PI".into()
            },
            Utf8 { //                               23
                value: "F".into()
            },
            Utf8 { //                               24
                value: "ConstantValue".into()
            },
            Float { //                              25
                value: 3.1415927,
            },
            Utf8 { //                               26
                value: "SPEED_OF_LIGHT".into()
            },
            Utf8 { //                               27
                value: "I".into()
            },
            Integer { //                            28
                value: 299792458,
            },
            Utf8 { //                               29
                value: "MIN_INT".into()
            },
            Integer { //                            30
                value: -2147483648,
            },
            Utf8 { //                               31
                value: "MIN_LONG".into()
            },
            Utf8 { //                               32
                value: "J".into()
            },
            Long { //                               33
                value: -9223372036854775808,
            },
            Empty, //                               34
            Utf8 { //                               35
                value: "MAX_LONG".into()
            },
            Long { //                               36
                value: 9223372036854775807,
            },
            Empty, //                               37
            Utf8 { //                               38
                value: "MAX_DOUBLE".into()
            },
            Utf8 { //                               39
                value: "D".into()
            },
            Double { //                             40
                value: 1.7976931348623157E308,
            },
            Empty, //                               41
            Utf8 { //                               42
                value: "MIN_DOUBLE".into()
            },
            Double { //                             43
                value: 4.9E-324,
            },
            Empty, //                               44
            Utf8 { //                               45
                value: "Code".into()
            },
            Utf8 { //                               46
                value: "LineNumberTable".into()
            },
            Utf8 { //                               47
                value: "LocalVariableTable".into()
            },
            Utf8 { //                               48
                value: "this".into()
            },
            Utf8 { //                               49
                value: "LTrivial;".into()
            },
            Utf8 { //                               50
                value: "LocalVariableTypeTable".into()
            },
            Utf8 { //                               51
                value: "LTrivial<TT;>;".into()
            },
            Utf8 { //                               52
                value: "MethodParameters".into()
            },
            Utf8 { //                               53
                value: "add".into()
            },
            Utf8 { //                               54
                value: "(II)I".into()
            },
            Utf8 { //                               55
                value: "first".into()
            },
            Utf8 { //                               56
                value: "second".into()
            },
            Utf8 { //                               57
                value: "result".into()
            },
            Utf8 { //                               58
                value: "StackMapTable".into()
            },
            Utf8 { //                               59
                value: "Exceptions".into()
            },
            Class { //                              60
                name_index: 61
            },
            Utf8 { //                               61
                value: "java/lang/ClassNotFoundException".into()
            },
            Utf8 { //                               62
                value: "runnable".into()
            },
            Utf8 { //                               63
                value: "Ljava/lang/Runnable;".into()
            },
            Utf8 { //                               64
                value: "lambda$run$0".into()
            },
            Utf8 { //                               65
                value: "Signature".into()
            },
            Utf8 { //                               66
                value: "<T:Ljava/lang/Object;>Ljava/lang/Object;Ljava/lang/Runnable;".into()
            },
            Utf8 { //                               67
                value: "SourceFile".into()
            },
            Utf8 { //                               68
                value: "Trivial.java".into()
            },
            Utf8 { //                               69
                value: "NestMembers".into()
            },
            Class { //                              70
                name_index: 71,
            },
            Utf8 { //                               71
                value: "Trivial$InnerCls".into()
            },
            Class { //                              72
                name_index: 73
            }, Utf8 { //                            73
                value: "Trivial$1LocalCls".into()
            },
            Utf8 { //                               74
                value: "BootstrapMethods".into()
            },
            MethodHandle { //                       75
                reference_kind: 6,
                reference_index: 76,
            },
            Methodref { //                          76
                class_index: 77,
                name_and_type_index: 78,
            },
            Class { //                              77
                name_index: 79,
            },
            NameAndType { //                        78
                name_index: 80,
                descriptor_index: 81,
            },
            Utf8 { //                               79
                value: "java/lang/invoke/LambdaMetafactory".into()
            },
            Utf8 { //                               80
                value: "metafactory".into()
            },
            Utf8 { //                               81
                value: "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/CallSite;".into()
            },
            MethodType { //                         82
                descriptor_index: 6,
            },
            MethodHandle { //                       83
                reference_kind: 6,
                reference_index: 84,
            },
            Methodref { //                          84
                class_index: 8,
                name_and_type_index: 85,
            },
            NameAndType { //                        85
                name_index: 64,
                descriptor_index: 6,
            },
            Utf8 { //                               86
                value: "InnerClasses".into()
            },
            Utf8 { //                               87
                value: "InnerCls".into()
            },
            Utf8 { //                               88
                value: "LocalCls".into()
            },
            Class { //                              89
                name_index: 90,
            },
            Utf8 { //                               90
                value: "java/lang/invoke/MethodHandles$Lookup".into()
            },
            Class { //                              91
                name_index: 92,
            },
            Utf8 { //                               92
                value: "java/lang/invoke/MethodHandles".into()
            },
            Utf8 { //                               93
                value: "Lookup".into()
            },
        ],
        ClassFlags::ACC_PUBLIC | ClassFlags::ACC_SUPER,
        8,
        2,
        vec![20],
        vec![
            FieldInfo::new(FieldFlags::ACC_PUBLIC | FieldFlags::ACC_STATIC | FieldFlags::ACC_FINAL, 22, 23, vec![ConstantValue { constantvalue_index: 25 }]),
            FieldInfo::new(FieldFlags::ACC_PROTECTED | FieldFlags::ACC_STATIC | FieldFlags::ACC_FINAL, 26, 27, vec![ConstantValue { constantvalue_index: 28 }]),
            FieldInfo::new(FieldFlags::ACC_PRIVATE | FieldFlags::ACC_STATIC | FieldFlags::ACC_FINAL, 29, 27, vec![ConstantValue { constantvalue_index: 30 }]),
            FieldInfo::new(FieldFlags::ACC_PRIVATE | FieldFlags::ACC_STATIC | FieldFlags::ACC_FINAL, 31, 32, vec![ConstantValue { constantvalue_index: 33 }]),
            FieldInfo::new(FieldFlags::ACC_PRIVATE | FieldFlags::ACC_STATIC | FieldFlags::ACC_FINAL, 35, 32, vec![ConstantValue { constantvalue_index: 36 }]),
            FieldInfo::new(FieldFlags::ACC_PRIVATE | FieldFlags::ACC_STATIC | FieldFlags::ACC_FINAL, 38, 39, vec![ConstantValue { constantvalue_index: 40 }]),
            FieldInfo::new(FieldFlags::ACC_PRIVATE | FieldFlags::ACC_STATIC | FieldFlags::ACC_FINAL, 42, 39, vec![ConstantValue { constantvalue_index: 43 }]),
            FieldInfo::new(FieldFlags::ACC_PROTECTED, 11, 12, vec![]),
        ],
        vec![
            MethodInfo::new(MethodFlags::ACC_PUBLIC, 5, 15, vec![
                Code {
                    max_stack: 2,
                    max_locals: 2,
                    code: vec![0x2a, 0xb7, 0x00, 0x01, 0x2a, 0x2b, 0xb5, 0x00, 0x07, 0xb1],
                    exception_table: vec![],
                    attributes: vec![
                        LineNumberTable {
                            line_number_table: vec![
                                LineNumberRecord::new(0, 15),
                                LineNumberRecord::new(4, 16),
                                LineNumberRecord::new(9, 17),
                            ]
                        },
                        LocalVariableTable {
                            local_variable_table: vec![
                                LocalVariableTableRecord::new(0, 10, 48, 49, 0),
                                LocalVariableTableRecord::new(0, 10, 11, 12, 1),
                            ]
                        },
                        LocalVariableTypeTable {
                            local_variable_type_table: vec![
                                LocalVariableTypeTableRecord::new(0, 10, 48, 51, 0),
                            ]
                        },
                    ],
                },
                MethodParameters {
                    parameters: vec![
                        MethodParameterRecord::new(11, MethodParameterFlags::empty()),
                    ]
                },
            ],
            ),
            MethodInfo::new(MethodFlags::ACC_PUBLIC, 5, 6, vec![
                Code {
                    max_stack: 2,
                    max_locals: 1,
                    code: vec![0x2a, 0x01, 0xb7, 0x00, 0x0d, 0xb1],
                    exception_table: vec![],
                    attributes: vec![
                        LineNumberTable {
                            line_number_table: vec![
                                LineNumberRecord::new(0, 20),
                                LineNumberRecord::new(5, 21),
                            ]
                        },
                        LocalVariableTable {
                            local_variable_table: vec![
                                LocalVariableTableRecord::new(0, 6, 48, 49, 0)
                            ]
                        },
                        LocalVariableTypeTable {
                            local_variable_type_table: vec![
                                LocalVariableTypeTableRecord::new(0, 6, 48, 51, 0),
                            ]
                        },
                    ],
                }
            ],
            ),
            MethodInfo::new(MethodFlags::ACC_PUBLIC, 53, 54, vec![
                Code {
                    max_stack: 2,
                    max_locals: 4,
                    code: vec![0x1b, 0x1c, 0x60, 0x3e, 0x1d, 0x9e, 0x00, 0x07, 0x1d, 0xa7, 0x00, 0x04, 0x03, 0xac],
                    exception_table: vec![],
                    attributes: vec![
                        LineNumberTable {
                            line_number_table: vec![
                                LineNumberRecord::new(0, 24),
                                LineNumberRecord::new(4, 26),
                            ]
                        },
                        LocalVariableTable {
                            local_variable_table: vec![
                                LocalVariableTableRecord::new(0, 14, 48, 49, 0),
                                LocalVariableTableRecord::new(0, 14, 55, 27, 1),
                                LocalVariableTableRecord::new(0, 14, 56, 27, 2),
                                LocalVariableTableRecord::new(4, 10, 57, 27, 3),
                            ]
                        },
                        LocalVariableTypeTable {
                            local_variable_type_table: vec![
                                LocalVariableTypeTableRecord::new(0, 14, 48, 51, 0),
                            ]
                        },
                        StackMapTable {
                            entries: vec![
                                AppendFrame {
                                    frame_type: 252,
                                    offset_delta: 12,
                                    locals: vec![IntegerVariableInfo],
                                },
                                SameLocals1StackItemFrame {
                                    frame_type: 64,
                                    offset_delta: 0,
                                    stack: IntegerVariableInfo,
                                },
                            ]
                        },
                    ],
                },
                Exceptions { exception_index_table: vec![60] },
                MethodParameters {
                    parameters: vec![
                        MethodParameterRecord::new(55, MethodParameterFlags::empty()),
                        MethodParameterRecord::new(56, MethodParameterFlags::ACC_FINAL),
                    ]
                },
            ]),
            MethodInfo::new(MethodFlags::ACC_PUBLIC, 18, 6, vec![
                Code {
                    max_stack: 1,
                    max_locals: 2,
                    code: vec![0xba, 0x00, 0x10, 0x00, 0x00, 0x4c, 0xb1],
                    exception_table: vec![],
                    attributes: vec![
                        LineNumberTable {
                            line_number_table: vec![
                                LineNumberRecord::new(0, 31),
                                LineNumberRecord::new(6, 34),
                            ]
                        },
                        LocalVariableTable {
                            local_variable_table: vec![
                                LocalVariableTableRecord::new(0, 7, 48, 49, 0),
                                LocalVariableTableRecord::new(6, 1, 62, 63, 1),
                            ]
                        },
                        LocalVariableTypeTable {
                            local_variable_type_table: vec![
                                LocalVariableTypeTableRecord::new(0, 7, 48, 51, 0),
                            ]
                        },
                    ],
                }
            ],
            ),
            MethodInfo::new(MethodFlags::ACC_PRIVATE | MethodFlags::ACC_STATIC | MethodFlags::ACC_SYNTHETIC, 64, 6, vec![
                Code {
                    max_stack: 0,
                    max_locals: 0,
                    code: vec![0xb1],
                    exception_table: vec![],
                    attributes: vec![
                        LineNumberTable {
                            line_number_table: vec![LineNumberRecord::new(0, 31)],
                        }
                    ],
                }
            ]),
        ],
        vec![
            Signature { signature_index: 66 },
            SourceFile { sourcefile_index: 68 },
            NestMembers { classes: vec![70, 72] },
            BootstrapMethods { bootstrap_methods: vec![BootstrapMethodRecord::new(75, vec![82, 83, 82])] },
            InnerClasses {
                classes: vec![
                    InnerClassRecord::new(70, 8, 87, NestedClassFlags::empty()),
                    InnerClassRecord::new(72, 0, 88, NestedClassFlags::ACC_STATIC | NestedClassFlags::ACC_INTERFACE | NestedClassFlags::ACC_ABSTRACT),
                    InnerClassRecord::new(89, 91, 93, NestedClassFlags::ACC_PUBLIC | NestedClassFlags::ACC_STATIC | NestedClassFlags::ACC_FINAL),
                ]
            },
        ],
    );

    assert_eq!(actual_class_file, expected_class_file)
}

#[test]
fn should_load_and_parse_complex_runtime_visible_annotations() {
    let bytes = include_bytes!("test_data/FunctionalInterface.class");
    let actual_class_file = parse(bytes).unwrap();

    let expected_class_file = ClassFile::new(
        0xCAFEBABE,
        0,
        61,
        vec![
            Empty, //                                               0
            Class {
                //                                                  1
                name_index: 2,
            },
            Utf8 {
                //                                                  2
                value: "fake/java/lang/FunctionalInterface".into(),
            },
            Class {
                //                                                  3
                name_index: 4,
            },
            Utf8 {
                //                                                  4
                value: "java/lang/Object".into(),
            },
            Class {
                //                                                  5
                name_index: 6,
            },
            Utf8 {
                //                                                  6
                value: "java/lang/annotation/Annotation".into(),
            },
            Utf8 {
                //                                                  7
                value: "SourceFile".into(),
            },
            Utf8 {
                //                                                  8
                value: "FunctionalInterface.java".into(),
            },
            Utf8 {
                //                                                  9
                value: "RuntimeVisibleAnnotations".into(),
            },
            Utf8 {
                //                                                  10
                value: "Ljava/lang/annotation/Documented;".into(),
            },
            Utf8 {
                //                                                  11
                value: "Ljava/lang/annotation/Retention;".into(),
            },
            Utf8 {
                //                                                  12
                value: "value".into(),
            },
            Utf8 {
                //                                                  13
                value: "Ljava/lang/annotation/RetentionPolicy;".into(),
            },
            Utf8 {
                //                                                  14
                value: "RUNTIME".into(),
            },
            Utf8 {
                //                                                  15
                value: "Ljava/lang/annotation/Target;".into(),
            },
            Utf8 {
                //                                                  16
                value: "Ljava/lang/annotation/ElementType;".into(),
            },
            Utf8 {
                //                                                  17
                value: "TYPE".into(),
            },
        ],
        ClassFlags::ACC_PUBLIC
            | ClassFlags::ACC_INTERFACE
            | ClassFlags::ACC_ABSTRACT
            | ClassFlags::ACC_ANNOTATION,
        1,
        3,
        vec![5],
        vec![],
        vec![],
        vec![
            SourceFile {
                sourcefile_index: 8,
            },
            RuntimeVisibleAnnotations {
                annotations: vec![
                    Annotation::new(10, vec![]),
                    Annotation::new(
                        11,
                        vec![ElementValuePair::new(
                            12,
                            EnumConstValue {
                                tag: 'e' as u8,
                                type_name_index: 13,
                                const_name_index: 14,
                            },
                        )],
                    ),
                    Annotation::new(
                        15,
                        vec![ElementValuePair::new(
                            12,
                            ArrayValue {
                                tag: '[' as u8,
                                values: vec![EnumConstValue {
                                    tag: 'e' as u8,
                                    type_name_index: 16,
                                    const_name_index: 17,
                                }],
                            },
                        )],
                    ),
                ],
                raw: vec![
                    0, 3, 0, 10, 0, 0, 0, 11, 0, 1, 0, 12, 101, 0, 13, 0, 14, 0, 15, 0, 1, 0, 12,
                    91, 0, 1, 101, 0, 16, 0, 17,
                ],
            },
        ],
    );

    assert_eq!(actual_class_file, expected_class_file)
}

#[test]
fn should_load_and_parse_complex_runtime_invisible_annotations() {
    let bytes = include_bytes!("test_data/RuntimeInvisibleAnnotationUsage.class");
    let actual_class_file = parse(bytes).unwrap();

    let expected_class_file = ClassFile::new(
        0xCAFEBABE,
        0,
        61,
        vec![
            Empty, //                                               0
            Class {
                //                                                  1
                name_index: 2,
            },
            Utf8 {
                //                                                  2
                value: "RuntimeInvisibleAnnotationUsage".into(),
            },
            Class {
                //                                                  3
                name_index: 4,
            },
            Utf8 {
                //                                                  4
                value: "java/lang/Object".into(),
            },
            Utf8 {
                //                                                  5
                value: "SourceFile".into(),
            },
            Utf8 {
                //                                                  6
                value: "RuntimeInvisibleAnnotationUsage.java".into(),
            },
            Utf8 {
                //                                                  7
                value: "RuntimeInvisibleAnnotations".into(),
            },
            Utf8 {
                //                                                  8
                value: "LRuntimeInvisibleAnnotation;".into(),
            },
            Utf8 {
                //                                                  9
                value: "value".into(),
            },
            Utf8 {
                //                                                  10
                value: "This is a runtime invisible annotation".into(),
            },
        ],
        ClassFlags::ACC_PUBLIC | ClassFlags::ACC_INTERFACE | ClassFlags::ACC_ABSTRACT,
        1,
        3,
        vec![],
        vec![],
        vec![],
        vec![
            SourceFile {
                sourcefile_index: 6,
            },
            RuntimeInvisibleAnnotations {
                annotations: vec![Annotation::new(
                    8,
                    vec![ElementValuePair::new(
                        9,
                        ConstValueIndex {
                            tag: 's' as u8,
                            const_value_index: 10,
                        },
                    )],
                )],
            },
        ],
    );

    assert_eq!(actual_class_file, expected_class_file)
}

#[test]
fn should_load_and_parse_permitted_subclasses_annotation() {
    let bytes = include_bytes!("test_data/Shape.class");
    let actual_class_file = parse(bytes).unwrap();

    let expected_class_file = ClassFile::new(
        0xCAFEBABE,
        0,
        61,
        vec![
            Empty, //                                               0
            Class {
                //                                                  1
                name_index: 2,
            },
            Utf8 {
                //                                                  2
                value: "Shape".into(),
            },
            Class {
                //                                                  3
                name_index: 4,
            },
            Utf8 {
                //                                                  4
                value: "java/lang/Object".into(),
            },
            Utf8 {
                //                                                  5
                value: "SourceFile".into(),
            },
            Utf8 {
                //                                                  6
                value: "Shape.java".into(),
            },
            Utf8 {
                //                                                  7
                value: "PermittedSubclasses".into(),
            },
            Class {
                //                                                  8
                name_index: 9,
            },
            Utf8 {
                //                                                  9
                value: "Circle".into(),
            },
        ],
        ClassFlags::ACC_PUBLIC | ClassFlags::ACC_INTERFACE | ClassFlags::ACC_ABSTRACT,
        1,
        3,
        vec![],
        vec![],
        vec![],
        vec![
            SourceFile {
                sourcefile_index: 6,
            },
            PermittedSubclasses { classes: vec![8] },
        ],
    );

    assert_eq!(actual_class_file, expected_class_file)
}

#[test]
fn should_load_and_parse_annotation_default_annotation() {
    let bytes = include_bytes!("test_data/RuntimeInvisibleAnnotation.class");
    let actual_class_file = parse(bytes).unwrap();

    let expected_class_file = ClassFile::new(
        0xCAFEBABE,
        0,
        61,
        vec![
            Empty, //                                               0
            Class {
                //                                                  1
                name_index: 2,
            },
            Utf8 {
                //                                                  2
                value: "RuntimeInvisibleAnnotation".into(),
            },
            Class {
                //                                                  3
                name_index: 4,
            },
            Utf8 {
                //                                                  4
                value: "java/lang/Object".into(),
            },
            Class {
                //                                                  5
                name_index: 6,
            },
            Utf8 {
                //                                                  6
                value: "java/lang/annotation/Annotation".into(),
            },
            Utf8 {
                //                                                  7
                value: "value".into(),
            },
            Utf8 {
                //                                                  8
                value: "()Ljava/lang/String;".into(),
            },
            Utf8 {
                //                                                  9
                value: "AnnotationDefault".into(),
            },
            Utf8 {
                //                                                  10
                value: "I'm a default value".into(),
            },
            Utf8 {
                //                                                  11
                value: "SourceFile".into(),
            },
            Utf8 {
                //                                                  12
                value: "RuntimeInvisibleAnnotationUsage.java".into(),
            },
            Utf8 {
                //                                                  13
                value: "RuntimeVisibleAnnotations".into(),
            },
            Utf8 {
                //                                                  14
                value: "Ljava/lang/annotation/Retention;".into(),
            },
            Utf8 {
                //                                                  15
                value: "Ljava/lang/annotation/RetentionPolicy;".into(),
            },
            Utf8 {
                //                                                  16
                value: "CLASS".into(),
            },
        ],
        ClassFlags::ACC_INTERFACE | ClassFlags::ACC_ABSTRACT | ClassFlags::ACC_ANNOTATION,
        1,
        3,
        vec![5],
        vec![],
        vec![MethodInfo::new(
            MethodFlags::ACC_PUBLIC | MethodFlags::ACC_ABSTRACT,
            7,
            8,
            vec![AnnotationDefault {
                default_value: ConstValueIndex {
                    tag: 's' as u8,
                    const_value_index: 10,
                },
                raw: vec![115, 0, 10],
            }],
        )],
        vec![
            SourceFile {
                sourcefile_index: 12,
            },
            RuntimeVisibleAnnotations {
                annotations: vec![Annotation::new(
                    14,
                    vec![ElementValuePair::new(
                        7,
                        EnumConstValue {
                            tag: 'e' as u8,
                            type_name_index: 15,
                            const_name_index: 16,
                        },
                    )],
                )],
                raw: vec![0, 1, 0, 14, 0, 1, 0, 7, 101, 0, 15, 0, 16],
            },
        ],
    );

    assert_eq!(actual_class_file, expected_class_file)
}

#[test]
fn should_load_and_parse_local_class() {
    let bytes = include_bytes!("test_data/Trivial$1LocalCls.class");
    let actual_class_file = parse(bytes).unwrap();

    let expected_class_file = ClassFile::new(
        0xCAFEBABE,
        0,
        61,
        vec![
            Empty, //                                               0
            Class {
                //                                                  1
                name_index: 2,
            },
            Utf8 {
                //                                                  2
                value: "Trivial$1LocalCls".into(),
            },
            Class {
                //                                                  3
                name_index: 4,
            },
            Utf8 {
                //                                                  4
                value: "java/lang/Object".into(),
            },
            Utf8 {
                //                                                  5
                value: "SourceFile".into(),
            },
            Utf8 {
                //                                                  6
                value: "Trivial.java".into(),
            },
            Utf8 {
                //                                                  7
                value: "EnclosingMethod".into(),
            },
            Class {
                //                                                  8
                name_index: 9,
            },
            Utf8 {
                //                                                  9
                value: "Trivial".into(),
            },
            NameAndType {
                //                                                  10
                name_index: 11,
                descriptor_index: 12,
            },
            Utf8 {
                //                                                  11
                value: "run".into(),
            },
            Utf8 {
                //                                                  12
                value: "()V".into(),
            },
            Utf8 {
                //                                                  13
                value: "NestHost".into(),
            },
            Utf8 {
                //                                                  14
                value: "InnerClasses".into(),
            },
            Utf8 {
                //                                                  15
                value: "LocalCls".into(),
            },
        ],
        ClassFlags::ACC_INTERFACE | ClassFlags::ACC_ABSTRACT,
        1,
        3,
        vec![],
        vec![],
        vec![],
        vec![
            SourceFile {
                sourcefile_index: 6,
            },
            EnclosingMethod {
                class_index: 8,
                method_index: 10,
            },
            NestHost {
                host_class_index: 8,
            },
            InnerClasses {
                classes: vec![InnerClassRecord::new(
                    1,
                    0,
                    15,
                    NestedClassFlags::ACC_STATIC
                        | NestedClassFlags::ACC_INTERFACE
                        | NestedClassFlags::ACC_ABSTRACT,
                )],
            },
        ],
    );

    assert_eq!(actual_class_file, expected_class_file)
}

#[test]
fn should_load_and_parse_record() {
    let bytes = include_bytes!("test_data/Rcrd.class");
    let actual_class_file = parse(bytes).unwrap();

    let expected_class_file = ClassFile::new(
        0xCAFEBABE,
        0,
        66,
        vec![
            Empty, //                                               0
            Methodref { //                                          1
                class_index: 2,
                name_and_type_index: 3,
            },
            Class { //                                              2
                name_index: 4,
            },
            NameAndType { //                                        3
                name_index: 5,
                descriptor_index: 6,
            },
            Utf8 { //                                               4
                value: "java/lang/Record".into(),
            },
            Utf8 { //                                               5
                value: "<init>".into(),
            },
            Utf8 { //                                               6
                value: "()V".into(),
            },
            Fieldref { //                                           7
                class_index: 8,
                name_and_type_index: 9,
            },
            Class { //                                              8
                name_index: 10,
            },
            NameAndType { //                                        9
                name_index: 11,
                descriptor_index: 12,
            },
            Utf8 { //                                               10
                value: "Rcrd".into(),
            },
            Utf8 { //                                               11
                value: "recordArg".into(),
            },
            Utf8 { //                                               12
                value: "I".into(),
            },
            InvokeDynamic { //                                      13
                bootstrap_method_attr_index: 0,
                name_and_type_index: 14,
            },
            NameAndType { //                                        14
                name_index: 15,
                descriptor_index: 16,
            },
            Utf8 { //                                               15
                value: "toString".into(),
            },
            Utf8 { //                                               16
                value: "(LRcrd;)Ljava/lang/String;".into(),
            },
            InvokeDynamic { //                                      17
                bootstrap_method_attr_index: 0,
                name_and_type_index: 18,
            },
            NameAndType { //                                        18
                name_index: 19,
                descriptor_index: 20,
            },
            Utf8 { //                                               19
                value: "hashCode".into(),
            },
            Utf8 { //                                               20
                value: "(LRcrd;)I".into(),
            },
            InvokeDynamic { //                                      21
                bootstrap_method_attr_index: 0,
                name_and_type_index: 22,
            },
            NameAndType { //                                        22
                name_index: 23,
                descriptor_index: 24,
            },
            Utf8 { //                                               23
                value: "equals".into(),
            },
            Utf8 { //                                               24
                value: "(LRcrd;Ljava/lang/Object;)Z".into(),
            },
            Utf8 { //                                               25
                value: "(I)V".into(),
            },
            Utf8 { //                                               26
                value: "Code".into(),
            },
            Utf8 { //                                               27
                value: "LineNumberTable".into(),
            },
            Utf8 { //                                               28
                value: "LocalVariableTable".into(),
            },
            Utf8 { //                                               29
                value: "this".into(),
            },
            Utf8 { //                                               30
                value: "LRcrd;".into(),
            },
            Utf8 { //                                               31
                value: "MethodParameters".into(),
            },
            Utf8 { //                                               32
                value: "()Ljava/lang/String;".into(),
            },
            Utf8 { //                                               33
                value: "()I".into(),
            },
            Utf8 { //                                               34
                value: "(Ljava/lang/Object;)Z".into(),
            },
            Utf8 { //                                               35
                value: "o".into(),
            },
            Utf8 { //                                               36
                value: "Ljava/lang/Object;".into(),
            },
            Utf8 { //                                               37
                value: "SourceFile".into(),
            },
            Utf8 { //                                               38
                value: "Rcrd.java".into(),
            },
            Utf8 { //                                               39
                value: "Record".into(),
            },
            Utf8 { //                                               40
                value: "BootstrapMethods".into(),
            },
            String { //                                             41
                string_index: 11,
            },
            MethodHandle { //                                       42
                reference_kind: 1,
                reference_index: 7,
            },
            MethodHandle { //                                       43
                reference_kind: 6,
                reference_index: 44,
            },
            Methodref { //                                          44
                class_index: 45,
                name_and_type_index: 46,
            },
            Class { //                                              45
                name_index: 47,
            },
            NameAndType { //                                        46
                name_index: 48,
                descriptor_index: 49,
            },
            Utf8 { //                                               47
                value: "java/lang/runtime/ObjectMethods".into(),
            },
            Utf8 { //                                               48
                value: "bootstrap".into(),
            },
            Utf8 { //                                               49
                value: "(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/TypeDescriptor;Ljava/lang/Class;Ljava/lang/String;[Ljava/lang/invoke/MethodHandle;)Ljava/lang/Object;".into(),
            },
            Utf8 { //                                               50
                value: "InnerClasses".into(),
            },
            Class { //                                              51
                name_index: 52,
            },
            Utf8 { //                                               52
                value: "java/lang/invoke/MethodHandles$Lookup".into(),
            },
            Class { //                                              53
                name_index: 54,
            },
            Utf8 { //                                               54
                value: "java/lang/invoke/MethodHandles".into(),
            },
            Utf8 { //                                               55
                value: "Lookup".into(),
            },
        ],
        ClassFlags::ACC_PUBLIC | ClassFlags::ACC_FINAL | ClassFlags::ACC_SUPER,
        8,
        2,
        vec![],
        vec![
            FieldInfo::new(FieldFlags::ACC_PRIVATE | FieldFlags::ACC_FINAL, 11, 12, vec![])
        ],
        vec![
            MethodInfo::new(MethodFlags::ACC_PUBLIC, 5, 25, vec![
                Code {
                    max_stack: 2,
                    max_locals: 2,
                    code: vec![0x2a, 0xb7, 0x0, 0x1, 0x2a, 0x1b, 0xb5, 0x0, 0x7, 0xb1],
                    exception_table: vec![],
                    attributes: vec![
                        LineNumberTable {
                            line_number_table: vec![LineNumberRecord::new(0, 4)],
                        },
                        LocalVariableTable {
                            local_variable_table: vec![
                                LocalVariableTableRecord::new(0, 10, 29, 30, 0),
                                LocalVariableTableRecord::new(0, 10, 11, 12, 1),
                            ]
                        },
                    ],
                },
                MethodParameters {
                    parameters: vec![MethodParameterRecord::new(11, MethodParameterFlags::empty())],
                },
            ]),
            MethodInfo::new(MethodFlags::ACC_PUBLIC | MethodFlags::ACC_FINAL, 15, 32, vec![
                Code {
                    max_stack: 1,
                    max_locals: 1,
                    code: vec![0x2a, 0xba, 0x0, 0xd, 0x0, 0x0, 0xb0],
                    exception_table: vec![],
                    attributes: vec![
                        LineNumberTable {
                            line_number_table: vec![LineNumberRecord::new(0, 4)]
                        },
                        LocalVariableTable {
                            local_variable_table: vec![
                                LocalVariableTableRecord::new(0, 7, 29, 30, 0)
                            ]
                        },
                    ],
                }
            ]),
            MethodInfo::new(MethodFlags::ACC_PUBLIC | MethodFlags::ACC_FINAL, 19, 33, vec![
                Code {
                    max_stack: 1,
                    max_locals: 1,
                    code: vec![0x2a, 0xba, 0x0, 0x11, 0x0, 0x0, 0xac],
                    exception_table: vec![],
                    attributes: vec![
                        LineNumberTable {
                            line_number_table: vec![LineNumberRecord::new(0, 4)]
                        },
                        LocalVariableTable {
                            local_variable_table: vec![
                                LocalVariableTableRecord::new(0, 7, 29, 30, 0)
                            ]
                        },
                    ],
                }
            ]),
            MethodInfo::new(MethodFlags::ACC_PUBLIC | MethodFlags::ACC_FINAL, 23, 34, vec![
                Code {
                    max_stack: 2,
                    max_locals: 2,
                    code: vec![0x2a, 0x2b, 0xba, 0x0, 0x15, 0x0, 0x0, 0xac],
                    exception_table: vec![],
                    attributes: vec![
                        LineNumberTable {
                            line_number_table: vec![LineNumberRecord::new(0, 4)]
                        },
                        LocalVariableTable {
                            local_variable_table: vec![
                                LocalVariableTableRecord::new(0, 8, 29, 30, 0),
                                LocalVariableTableRecord::new(0, 8, 35, 36, 1),
                            ]
                        },
                    ],
                },
                MethodParameters {
                    parameters: vec![MethodParameterRecord::new(35, MethodParameterFlags::empty())],
                },
            ]),
            MethodInfo::new(MethodFlags::ACC_PUBLIC, 11, 33, vec![
                Code {
                    max_stack: 1,
                    max_locals: 1,
                    code: vec![0x2a, 0xb4, 0x0, 0x7, 0xac],
                    exception_table: vec![],
                    attributes: vec![
                        LineNumberTable {
                            line_number_table: vec![LineNumberRecord::new(0, 4)]
                        },
                        LocalVariableTable {
                            local_variable_table: vec![
                                LocalVariableTableRecord::new(0, 5, 29, 30, 0),
                            ]
                        },
                    ],
                }
            ]),
        ],
        vec![
            SourceFile {
                sourcefile_index: 38,
            },
            Record {
                components: vec![RecordComponentInfo::new(11, 12, vec![])],
            },
            BootstrapMethods {
                bootstrap_methods: vec![BootstrapMethodRecord::new(43, vec![8, 41, 42])],
            },
            InnerClasses {
                classes: vec![
                    InnerClassRecord::new(51, 53, 55, NestedClassFlags::ACC_PUBLIC | NestedClassFlags::ACC_STATIC | NestedClassFlags::ACC_FINAL),
                ]
            },
        ],
    );

    assert_eq!(actual_class_file, expected_class_file)
}

// sould be able to parse https://github.com/google/guava/blob/0bf87046267ce281b6335430679fbd59135a1303/guava/src/com/google/common/base/CharMatcher.java#L1487-L1489   
#[test]
fn should_load_and_parse_invalid_cesu8_strings() {
    let bytes = include_bytes!("test_data/InvalidCESU8.class");
    parse(bytes).unwrap();
}

#[test]
fn should_load_and_parse_mutf8_strings() {
    let bytes = include_bytes!("test_data/Mutf8.class");
    let actual_class_file = parse(bytes).unwrap();

    let expected_class_file = ClassFile::new(
        0xCAFEBABE,
        0,
        69,
        vec![
            Empty, //                                               0
            Class {
                //                                                  1
                name_index: 2,
            },
            Utf8 {
                //                                                  2
                value: "Mutf8".into(),
            },
            Class {
                //                                                  3
                name_index: 4,
            },
            Utf8 {
                //                                                  4
                value: "java/lang/Object".into(),
            },
            Utf8 {
                //                                                  5
                value: "withZero".into(),
            },
            Utf8 {
                //                                                  6
                value: "Ljava/lang/String;".into(),
            },
            Utf8 {
                //                                                  7
                value: "ConstantValue".into(),
            },
            String {
                //                                                  8
                string_index: 9,
            },
            Utf8 {
                //                                                  9
                value: "\0abc".into(),
            },
            Utf8 {
                //                                                  10
                value: "singleByteLatin".into(),
            },
            String {
                //                                                  11
                string_index: 12,
            },
            Utf8 {
                //                                                  12
                value: "A".into(),
            },
            Utf8 {
                //                                                  13
                value: "twoByteUkrainian".into(),
            },
            String {
                //                                                  14
                string_index: 15,
            },
            Utf8 {
                //                                                  15
                value: "ї".into(),
            },
            Utf8 {
                //                                                  16
                value: "threeByteSnowman".into(),
            },
            String {
                //                                                  17
                string_index: 18,
            },
            Utf8 {
                //                                                  18
                value: "☃".into(),
            },
            Utf8 {
                //                                                  19
                value: "fourByteGothicLetterHwair".into(),
            },
            String {
                //                                                  20
                string_index: 21,
            },
            Utf8 {
                //                                                  21
                value: "𐍈".into(),
            },
            Utf8 {
                //                                                  22
                value: "fourByteEmoji".into(),
            },
            String {
                //                                                  23
                string_index: 24,
            },
            Utf8 {
                //                                                  24
                value: "😂".into(),
            },
            Utf8 {
                //                                                  25
                value: "withNonValidSequences".into(),
            },
            String {
                //                                                  26
                string_index: 27,
            },
            Utf8 {
                //                                                  27
                value: "a??💔?b".into(),
            },
            Utf8 {
                //                                                  28
                value: "SourceFile".into(),
            },
            Utf8 {
                //                                                  29
                value: "Mutf8.java".into(),
            },
        ],
        ClassFlags::ACC_INTERFACE | ClassFlags::ACC_ABSTRACT,
        1,
        3,
        vec![],
        vec![
            FieldInfo::new(
                FieldFlags::ACC_PUBLIC | FieldFlags::ACC_STATIC | FieldFlags::ACC_FINAL,
                5,
                6,
                vec![ConstantValue {
                    constantvalue_index: 8,
                }],
            ),
            FieldInfo::new(
                FieldFlags::ACC_PUBLIC | FieldFlags::ACC_STATIC | FieldFlags::ACC_FINAL,
                10,
                6,
                vec![ConstantValue {
                    constantvalue_index: 11,
                }],
            ),
            FieldInfo::new(
                FieldFlags::ACC_PUBLIC | FieldFlags::ACC_STATIC | FieldFlags::ACC_FINAL,
                13,
                6,
                vec![ConstantValue {
                    constantvalue_index: 14,
                }],
            ),
            FieldInfo::new(
                FieldFlags::ACC_PUBLIC | FieldFlags::ACC_STATIC | FieldFlags::ACC_FINAL,
                16,
                6,
                vec![ConstantValue {
                    constantvalue_index: 17,
                }],
            ),
            FieldInfo::new(
                FieldFlags::ACC_PUBLIC | FieldFlags::ACC_STATIC | FieldFlags::ACC_FINAL,
                19,
                6,
                vec![ConstantValue {
                    constantvalue_index: 20,
                }],
            ),
            FieldInfo::new(
                FieldFlags::ACC_PUBLIC | FieldFlags::ACC_STATIC | FieldFlags::ACC_FINAL,
                22,
                6,
                vec![ConstantValue {
                    constantvalue_index: 23,
                }],
            ),
            FieldInfo::new(
                FieldFlags::ACC_PUBLIC | FieldFlags::ACC_STATIC | FieldFlags::ACC_FINAL,
                25,
                6,
                vec![ConstantValue {
                    constantvalue_index: 26,
                }],
            ),
        ],
        vec![],
        vec![SourceFile {
            sourcefile_index: 29,
        }],
    );

    assert_eq!(actual_class_file, expected_class_file)
}

#[test]
fn should_load_and_parse_runtime_visible_parameter_annotations() {
    let bytes = include_bytes!("test_data/RuntimeVisibleParameterAnnotations.class");
    let actual_class_file = parse(bytes).unwrap();

    let expected_class_file = ClassFile::new(
        0xCAFEBABE,
        0,
        67,
        vec![
            Empty, //                                               0
            Methodref {
                //                                                  1
                class_index: 2,
                name_and_type_index: 3,
            },
            Class {
                //                                                  2
                name_index: 4,
            },
            NameAndType {
                //                                                  3
                name_index: 5,
                descriptor_index: 6,
            },
            Utf8 {
                //                                                  4
                value: "java/lang/Object".into(),
            },
            Utf8 {
                //                                                  5
                value: "<init>".into(),
            },
            Utf8 {
                //                                                  6
                value: "()V".into(),
            },
            Class {
                //                                                  7
                name_index: 8,
            },
            Utf8 {
                //                                                  8
                value: "RuntimeVisibleParameterAnnotations".into(),
            },
            Utf8 {
                //                                                  9
                value: "Code".into(),
            },
            Utf8 {
                //                                                  10
                value: "LineNumberTable".into(),
            },
            Utf8 {
                //                                                  11
                value: "LocalVariableTable".into(),
            },
            Utf8 {
                //                                                  12
                value: "this".into(),
            },
            Utf8 {
                //                                                  13
                value: "LRuntimeVisibleParameterAnnotations;".into(),
            },
            Utf8 {
                //                                                  14
                value: "someMethod".into(),
            },
            Utf8 {
                //                                                  15
                value: "(Ljava/lang/String;I)V".into(),
            },
            Utf8 {
                //                                                  16
                value: "msg".into(),
            },
            Utf8 {
                //                                                  17
                value: "Ljava/lang/String;".into(),
            },
            Utf8 {
                //                                                  18
                value: "count".into(),
            },
            Utf8 {
                //                                                  19
                value: "I".into(),
            },
            Utf8 {
                //                                                  20
                value: "MethodParameters".into(),
            },
            Utf8 {
                //                                                  21
                value: "Ljava/lang/Deprecated;".into(),
            },
            Utf8 {
                //                                                  22
                value: "SourceFile".into(),
            },
            Utf8 {
                //                                                  23
                value: "RuntimeVisibleParameterAnnotations.java".into(),
            },
        ],
        ClassFlags::ACC_PUBLIC | ClassFlags::ACC_SUPER,
        7,
        2,
        vec![],
        vec![],
        vec![
            MethodInfo::new(
                MethodFlags::ACC_PUBLIC,
                5,
                6,
                vec![Code {
                    max_stack: 1,
                    max_locals: 1,
                    code: vec![0x2a, 0xb7, 0x0, 0x1, 0xb1],
                    exception_table: vec![],
                    attributes: vec![
                        LineNumberTable {
                            line_number_table: vec![LineNumberRecord::new(0, 4)],
                        },
                        LocalVariableTable {
                            local_variable_table: vec![LocalVariableTableRecord::new(
                                0, 5, 12, 13, 0,
                            )],
                        },
                    ],
                }],
            ),
            MethodInfo::new(
                MethodFlags::ACC_PUBLIC,
                14,
                15,
                vec![
                    Code {
                        max_stack: 0,
                        max_locals: 3,
                        code: vec![0xb1],
                        exception_table: vec![],
                        attributes: vec![
                            LineNumberTable {
                                line_number_table: vec![LineNumberRecord::new(0, 6)],
                            },
                            LocalVariableTable {
                                local_variable_table: vec![
                                    LocalVariableTableRecord::new(0, 1, 12, 13, 0),
                                    LocalVariableTableRecord::new(0, 1, 16, 17, 1),
                                    LocalVariableTableRecord::new(0, 1, 18, 19, 2),
                                ],
                            },
                        ],
                    },
                    MethodParameters {
                        parameters: vec![
                            MethodParameterRecord::new(16, MethodParameterFlags::ACC_FINAL),
                            MethodParameterRecord::new(18, MethodParameterFlags::empty()),
                        ],
                    },
                    RuntimeVisibleParameterAnnotations {
                        parameter_annotations: vec![
                            vec![Annotation::new(21, vec![])],
                            vec![Annotation::new(21, vec![])],
                        ],
                    },
                ],
            ),
        ],
        vec![SourceFile {
            sourcefile_index: 23,
        }],
    );

    assert_eq!(actual_class_file, expected_class_file)
}

#[test]
fn should_load_and_parse_runtime_visible_type_annotations() {
    let bytes = include_bytes!("test_data/RuntimeVisibleTypeAnnotations.class");
    let actual_class_file = parse(bytes).unwrap();

    let expected_class_file = ClassFile::new(
        0xCAFEBABE,
        0,
        67,
        vec![
            Empty, //                                               0
            Class {
                //                                                  1
                name_index: 2,
            },
            Utf8 {
                //                                                  2
                value: "RuntimeVisibleTypeAnnotations".into(),
            },
            Class {
                //                                                  3
                name_index: 4,
            },
            Utf8 {
                //                                                  4
                value: "java/lang/Object".into(),
            },
            Utf8 {
                //                                                  5
                value: "someMethod".into(),
            },
            Utf8 {
                //                                                  6
                value: "()Ljava/lang/String;".into(),
            },
            Utf8 {
                //                                                  7
                value: "LInfo;".into(),
            },
            Utf8 {
                //                                                  8
                value: "SourceFile".into(),
            },
            Utf8 {
                //                                                  9
                value: "RuntimeVisibleTypeAnnotations.java".into(),
            },
        ],
        ClassFlags::ACC_INTERFACE | ClassFlags::ACC_ABSTRACT,
        1,
        3,
        vec![],
        vec![],
        vec![MethodInfo::new(
            MethodFlags::ACC_PUBLIC | MethodFlags::ACC_ABSTRACT,
            5,
            6,
            vec![RuntimeVisibleTypeAnnotations {
                type_annotations: vec![TypeAnnotation::new(
                    TargetType::METHOD_RETURN,
                    TargetInfo::EmptyTarget,
                    vec![],
                    Annotation::new(7, vec![]),
                )],
            }],
        )],
        vec![SourceFile {
            sourcefile_index: 9,
        }],
    );

    assert_eq!(actual_class_file, expected_class_file)
}

#[test]
#[cfg(feature = "serde")]
fn should_serialize_to_string() {
    let bytes = include_bytes!("test_data/Mutf8.class");
    let actual_class_file = parse(bytes).unwrap();
    let serialized = serde_json::to_string(&actual_class_file).unwrap();

    assert_eq!(
        serialized,
        r#"{"magic":3405691582,"minor_version":0,"major_version":66,"constant_pool":["Empty",{"Class":{"name_index":2}},{"Utf8":{"value":"Mutf8"}},{"Class":{"name_index":4}},{"Utf8":{"value":"java/lang/Object"}},{"Utf8":{"value":"withZero"}},{"Utf8":{"value":"Ljava/lang/String;"}},{"Utf8":{"value":"ConstantValue"}},{"String":{"string_index":9}},{"Utf8":{"value":"\u0000abc"}},{"Utf8":{"value":"singleByteLatin"}},{"String":{"string_index":12}},{"Utf8":{"value":"A"}},{"Utf8":{"value":"twoByteUkrainian"}},{"String":{"string_index":15}},{"Utf8":{"value":"ї"}},{"Utf8":{"value":"threeByteSnowman"}},{"String":{"string_index":18}},{"Utf8":{"value":"☃"}},{"Utf8":{"value":"fourByteGothicLetterHwair"}},{"String":{"string_index":21}},{"Utf8":{"value":"𐍈"}},{"Utf8":{"value":"fourByteEmoji"}},{"String":{"string_index":24}},{"Utf8":{"value":"😂"}},{"Utf8":{"value":"SourceFile"}},{"Utf8":{"value":"Mutf8.java"}}],"access_flags":"ACC_INTERFACE | ACC_ABSTRACT","this_class":1,"super_class":3,"interfaces":[],"fields":[{"access_flags":"ACC_PUBLIC | ACC_STATIC | ACC_FINAL","name_index":5,"descriptor_index":6,"attributes":[{"ConstantValue":{"constantvalue_index":8}}]},{"access_flags":"ACC_PUBLIC | ACC_STATIC | ACC_FINAL","name_index":10,"descriptor_index":6,"attributes":[{"ConstantValue":{"constantvalue_index":11}}]},{"access_flags":"ACC_PUBLIC | ACC_STATIC | ACC_FINAL","name_index":13,"descriptor_index":6,"attributes":[{"ConstantValue":{"constantvalue_index":14}}]},{"access_flags":"ACC_PUBLIC | ACC_STATIC | ACC_FINAL","name_index":16,"descriptor_index":6,"attributes":[{"ConstantValue":{"constantvalue_index":17}}]},{"access_flags":"ACC_PUBLIC | ACC_STATIC | ACC_FINAL","name_index":19,"descriptor_index":6,"attributes":[{"ConstantValue":{"constantvalue_index":20}}]},{"access_flags":"ACC_PUBLIC | ACC_STATIC | ACC_FINAL","name_index":22,"descriptor_index":6,"attributes":[{"ConstantValue":{"constantvalue_index":23}}]}],"methods":[],"attributes":[{"SourceFile":{"sourcefile_index":26}}]}"#
    )
}
