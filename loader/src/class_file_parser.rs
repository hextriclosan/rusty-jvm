use std::io;
use std::io::ErrorKind::{InvalidData, InvalidInput};
use std::mem::size_of;
use num_traits::Num;
use model::class_file::{Attribute, ClassFile, ConstantPool, ExceptionRecord, FieldInfo, LineNumberRecord, LocalVariableTableRecord, LocalVariableTypeTableRecord, MethodInfo, MethodParameterRecord};
use model::class_file::ConstantPool::*;
use model::class_file::Attribute::{Code, ConstantValue, Deprecated, Exceptions, LineNumberTable, LocalVariableTable, LocalVariableTypeTable, MethodParameters, Signature, SourceFile, Synthetic};

pub struct Parser {}

impl Parser {
    const MAGIC: u32 = 0xCAFEBABE;

    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse(&mut self, data: &[u8]) -> Result<ClassFile, io::Error> {
        let mut start_from = 0;
        let magic: u32 = convert(&data, &mut start_from)?;

        if magic != Self::MAGIC {
            return Err(io::Error::new(InvalidInput, "Not a valid class-file"));
        }

        let minor_version: u16 = convert(&data, &mut start_from)?;
        let major_version: u16 = convert(&data, &mut start_from)?;
        let constant_pool_count: u16 = convert(&data, &mut start_from)?;

        let mut constant_pool_vec = Vec::new();
        for _ in 0..constant_pool_count {
            match constant_pool_vec.last() {
                Some(val) => match val {
                    Double { .. } | Long { .. } => {
                        constant_pool_vec.push(Empty);
                        continue;
                    }
                    _ => {}
                }
                None => {
                    constant_pool_vec.push(Empty);
                    continue;
                }
            }

            let tag: u8 = convert(&data, &mut start_from)?;

            let constant_pool_entry = match tag {
                1 => {
                    let length: u16 = convert(&data, &mut start_from)?;
                    let bytes: &[u8] = convert_bytes(&data, &mut start_from, length as usize)?;

                    Uint8 {
                        value: std::string::String::from_utf8(bytes.to_vec())
                            .map_err(|e| io::Error::new(InvalidData, e))?
                    }
                }
                3 => {
                    Integer {
                        value: convert(&data, &mut start_from)?
                    }
                }
                4 => {
                    let bytes: u32 = convert(&data, &mut start_from)?;

                    Float {
                        value: match bytes {
                            0x7f800000 => f32::INFINITY,
                            0xff800000 => f32::NEG_INFINITY,
                            0x7f800001..=0x7fffffff | 0xff800001..=0xffffffff => f32::NAN,
                            _ => {
                                let s = if (bytes >> 31) == 0 { 1 } else { -1 };
                                let e = ((bytes >> 23) & 0xff) as i32;
                                let m = (if e == 0 { (bytes & 0x7fffff) << 1 } else { (bytes & 0x7fffff) | 0x800000 }) as i32;

                                s as f32 * m as f32 * 2_f32.powi(e - 150)
                            }
                        }
                    }
                }
                5 => {
                    Long {
                        value: convert(&data, &mut start_from)?
                    }
                }
                6 => {
                    let bytes: u64 = convert(&data, &mut start_from)?;
                    Double {
                        value: match bytes {
                            0x7ff0000000000000_u64 => f64::INFINITY,
                            0xfff0000000000000_u64 => f64::NEG_INFINITY,
                            0x7ff0000000000001_u64..=0x7fffffffffffffff_u64 | 0xfff0000000000001_u64..=0xffffffffffffffff_u64 => f64::NAN,
                            _ => {
                                let s = if (bytes >> 63) == 0 { 1 } else { -1 };
                                let e = ((bytes >> 52) & 0x7ff_u64) as i32;
                                let m = (if e == 0 { (bytes & 0xfffffffffffff_u64) << 1 } else { (bytes & 0xfffffffffffff_u64) | 0x10000000000000_u64 }) as i64;

                                s as f64 * m as f64 * 2_f64.powi(e - 1075)
                            }
                        },
                    }
                }
                7 => Class {
                    name_index: convert(&data, &mut start_from)?,
                },
                8 => String {
                    string_index: convert(&data, &mut start_from)?,
                },
                9 => Fieldref {
                    class_index: convert(&data, &mut start_from)?,
                    name_and_type_index: convert(&data, &mut start_from)?,
                },
                10 => Methodref {
                    class_index: convert(&data, &mut start_from)?,
                    name_and_type_index: convert(&data, &mut start_from)?,
                },
                11 => InterfaceMethodref {
                    class_index: convert(&data, &mut start_from)?,
                    name_and_type_index: convert(&data, &mut start_from)?,
                },
                12 => NameAndType {
                    name_index: convert(&data, &mut start_from)?,
                    descriptor_index: convert(&data, &mut start_from)?,
                },
                15 => MethodHandle {
                    reference_kind: convert(&data, &mut start_from)?,
                    reference_index: convert(&data, &mut start_from)?,
                },
                16 => MethodType {
                    descriptor_index: convert(&data, &mut start_from)?,
                },
                17 => Dynamic {
                    bootstrap_method_attr_index: convert(&data, &mut start_from)?,
                    name_and_type_index: convert(&data, &mut start_from)?,
                },
                18 => InvokeDynamic {
                    bootstrap_method_attr_index: convert(&data, &mut start_from)?,
                    name_and_type_index: convert(&data, &mut start_from)?,
                },
                19 => Module {
                    name_index: convert(&data, &mut start_from)?,
                },
                20 => Package {
                    name_index: convert(&data, &mut start_from)?,
                },
                _ => {
                    return Err(io::Error::new(InvalidInput, format!("unmatched tag: {:?}", tag)));
                }
            };

            constant_pool_vec.push(constant_pool_entry);
        }

        let access_flags: u16 = convert(&data, &mut start_from)?;
        let this_class: u16 = convert(&data, &mut start_from)?;
        let super_class: u16 = convert(&data, &mut start_from)?;
        let interfaces_count: u16 = convert(&data, &mut start_from)?;

        let mut interfaces = Vec::new();
        for _ in 0..interfaces_count {
            interfaces.push(convert(&data, &mut start_from)?);
        }

        let fields_count: u16 = convert(&data, &mut start_from)?;
        let mut fields = Vec::new();
        for _ in 0..fields_count {
            let access_flags = convert(&data, &mut start_from)?;
            let name_index = convert(&data, &mut start_from)?;
            let descriptor_index = convert(&data, &mut start_from)?;
            let attributes = Self::parse_attributes(&data, &mut start_from, &constant_pool_vec)?;

            fields.push(FieldInfo::new(access_flags, name_index, descriptor_index, attributes))
        }

        let methods_count: u16 = convert(&data, &mut start_from)?;
        let mut methods = Vec::new();
        for _ in 0..methods_count {
            let access_flags = convert(&data, &mut start_from)?;
            let name_index = convert(&data, &mut start_from)?;
            let descriptor_index = convert(&data, &mut start_from)?;
            let attributes = Self::parse_attributes(&data, &mut start_from, &constant_pool_vec)?;

            methods.push(MethodInfo::new(access_flags, name_index, descriptor_index, attributes))
        }

        let attributes = Self::parse_attributes(&data, &mut start_from, &constant_pool_vec)?;

        if data.len() != start_from {
            return Err(io::Error::new(InvalidInput, format!("Not all was read : data.len() is {}, start_from is {}", data.len(), start_from)));
        }

        Ok(
            ClassFile::new(
                magic,
                minor_version,
                major_version,
                constant_pool_vec,
                access_flags,
                this_class,
                super_class,
                interfaces,
                fields,
                methods,
                attributes,
            )
        )
    }

    fn parse_attributes(data: &[u8], mut start_from: &mut usize, constant_pool_vec: &Vec<ConstantPool>) -> Result<Vec<Attribute>, io::Error> {
        let attributes_count: u16 = convert(&data, &mut start_from)?;
        let mut attributes = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(Self::parse_attribute(&data, &mut start_from, constant_pool_vec)?);
        }

        Ok(attributes)
    }

    fn parse_attribute(data: &[u8], mut start_from: &mut usize, constant_pool_vec: &Vec<ConstantPool>) -> Result<Attribute, io::Error> {
        let attribute_name_index: u16 = convert(&data, &mut start_from)?;


        let attribute_name = match constant_pool_vec.get(attribute_name_index as usize) {
            Some(item) => match item {
                Uint8 { value } => value,
                _ => return Err(io::Error::new(InvalidData, format!("element type is not Uint8 but {:?}", item)))
            },
            None => return Err(io::Error::new(InvalidData, format!("element not found at index {}", attribute_name_index)))
        };

        let _attribute_length: u32 = convert(&data, &mut start_from)?;

        let attribute = match attribute_name.as_str() {
            "ConstantValue" => ConstantValue {
                constantvalue_index: convert(&data, &mut start_from)?,
            },
            "Code" => {
                let max_stack = convert(&data, &mut start_from)?;
                let max_locals = convert(&data, &mut start_from)?;
                let code_length: u32 = convert(&data, &mut start_from)?;
                let code = convert_bytes(&data, &mut start_from, code_length as usize)?.to_vec();
                let exception_table_length: u16 = convert(&data, &mut start_from)?;

                let mut exception_table = Vec::new();
                for _ in 0..exception_table_length {
                    exception_table.push(ExceptionRecord::new(
                        convert(&data, &mut start_from)?,
                        convert(&data, &mut start_from)?,
                        convert(&data, &mut start_from)?,
                        convert(&data, &mut start_from)?,
                    ));
                }

                let attributes = Self::parse_attributes(&data, &mut start_from, constant_pool_vec)?;

                Code {
                    max_stack,
                    max_locals,
                    code,
                    exception_table,
                    attributes,
                }
            }
            "Exceptions" => {
                let number_of_exceptions: u16 = convert(&data, &mut start_from)?;
                let mut exception_index_table = Vec::new();
                for _ in 0..number_of_exceptions {
                    exception_index_table.push(convert(&data, &mut start_from)?);
                }
                Exceptions {
                    exception_index_table
                }
            }
            "Synthetic" => Synthetic,
            "Deprecated" => Deprecated,
            "SourceFile" => SourceFile {
                sourcefile_index: convert(&data, &mut start_from)?,
            },
            "LineNumberTable" => {
                let line_number_table_length: u16 = convert(&data, &mut start_from)?;
                let mut line_number_table = Vec::new();
                for _ in 0..line_number_table_length {
                    line_number_table.push(LineNumberRecord::new(convert(&data, &mut start_from)?, convert(&data, &mut start_from)?));
                }
                LineNumberTable {
                    line_number_table,
                }
            }
            "LocalVariableTable" => {
                let local_variable_table_length: u16 = convert(&data, &mut start_from)?;
                let mut local_variable_table = Vec::new();
                for _ in 0..local_variable_table_length {
                    local_variable_table.push(LocalVariableTableRecord::new(
                        convert(&data, &mut start_from)?,
                        convert(&data, &mut start_from)?,
                        convert(&data, &mut start_from)?,
                        convert(&data, &mut start_from)?,
                        convert(&data, &mut start_from)?,
                    ))
                }

                LocalVariableTable {
                    local_variable_table
                }
            }
            "Signature" => Signature {
                signature_index: convert(&data, &mut start_from)?,
            },
            "LocalVariableTypeTable" => {
                let local_variable_type_table_length: u16 = convert(&data, &mut start_from)?;
                let mut local_variable_type_table = Vec::new();
                for _ in 0..local_variable_type_table_length {
                    local_variable_type_table.push(LocalVariableTypeTableRecord::new(
                        convert(&data, &mut start_from)?,
                        convert(&data, &mut start_from)?,
                        convert(&data, &mut start_from)?,
                        convert(&data, &mut start_from)?,
                        convert(&data, &mut start_from)?,
                    ))
                }

                LocalVariableTypeTable {
                    local_variable_type_table
                }
            }
            "MethodParameters" => {
                let parameters_count: u8 = convert(&data, &mut start_from)?;
                let mut parameters = Vec::new();
                for _ in 0..parameters_count {
                    parameters.push(MethodParameterRecord::new(
                        convert(&data, &mut start_from)?,
                        convert(&data, &mut start_from)?,
                    ))
                }
                MethodParameters {
                    parameters
                }
            }
            _ => {
                return Err(io::Error::new(InvalidInput, format!("unmatched attribute: {:?}", attribute_name)));
            }
        };

        Ok(attribute)
    }
}

fn convert<T>(slice: &[u8], start_from: &mut usize) -> Result<T, io::Error>
    where
        T: Num + From<u8> + std::ops::Shl<Output=T> + std::ops::BitOr<Output=T> + Copy, {
    if *start_from >= slice.len() {
        io::Error::new(
            InvalidInput,
            format!("overflow : attempt to read from {} whereas len is {}", *start_from, slice.len()));
    }

    let size = size_of::<T>();
    let mut value = T::zero();
    if size == 1 {
        value = T::from(slice[*start_from]);
    } else {
        let sub_slice = &slice[*start_from..*start_from + size];
        for &byte in sub_slice {
            value = (value << 8.into()) | T::from(byte);
        }
    }
    *start_from += size;

    Ok(value)
}

fn convert_bytes<'a>(slice: &'a [u8], start_from: &mut usize, size: usize) -> Result<&'a [u8], io::Error> {
    if *start_from + size > slice.len() {
        return Err(io::Error::new(
            InvalidInput,
            format!("Index out of bounds: {} of {}", *start_from + size, slice.len())));
    }

    let sub_slice = &slice[*start_from..*start_from + size];

    *start_from += size;

    Ok(sub_slice)
}

#[cfg(test)]
mod tests {
    use model::class_file::{ClassFile, FieldInfo, LineNumberRecord, LocalVariableTableRecord, LocalVariableTypeTableRecord, MethodInfo, MethodParameterRecord};
    use model::class_file::Attribute::{Code, ConstantValue, Exceptions, LineNumberTable, LocalVariableTable, LocalVariableTypeTable, MethodParameters, Signature, SourceFile};
    use model::class_file::ConstantPool::{Class, Double, Empty, Fieldref, Float, Integer, Long, Methodref, NameAndType, Uint8};
    use crate::loader::load;

    #[test]
    fn should_load_and_parse() {
        let actual_class_file = load("../test_data/Trivial.class").unwrap();

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
                Uint8 { //                              4
                    value: "java/lang/Object".into()
                },
                Uint8 { //                              5
                    value: "<init>".into()
                },
                Uint8 { //                              6
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
                Uint8 { //                              10
                    value: "Trivial".into()
                },
                Uint8 { //                              11
                    value: "someText".into()
                },
                Uint8 { //                              12
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
                Uint8 { //                              15
                    value: "(Ljava/lang/String;)V".into()
                },
                Class { //                              16
                    name_index: 17,
                },
                Uint8 { //                              17
                    value: "java/lang/Runnable".into()
                },
                Uint8 { //                              18
                    value: "PI".into()
                },
                Uint8 { //                              19
                    value: "F".into()
                },
                Uint8 { //                              20
                    value: "ConstantValue".into()
                },
                Float { //                              21
                    value: 3.1415927,
                },
                Uint8 { //                              22
                    value: "SPEED_OF_LIGHT".into()
                },
                Uint8 { //                              23
                    value: "I".into()
                },
                Integer { //                            24
                    value: 299792458,
                },
                Uint8 { //                              25
                    value: "MIN_INT".into()
                },
                Integer { //                            26
                    value: -2147483648,
                },
                Uint8 { //                              27
                    value: "MIN_LONG".into()
                },
                Uint8 { //                              28
                    value: "J".into()
                },
                Long { //                               29
                    value: -9223372036854775808,
                },
                Empty, //                               30
                Uint8 { //                              31
                    value: "MAX_LONG".into()
                },
                Long { //                               32
                    value: 9223372036854775807,
                },
                Empty, //                               33
                Uint8 { //                              34
                    value: "MAX_DOUBLE".into()
                },
                Uint8 { //                              35
                    value: "D".into()
                },
                Double { //                             36
                    value: 1.7976931348623157E308,
                },
                Empty, //                               37
                Uint8 { //                              38
                    value: "MIN_DOUBLE".into()
                },
                Double { //                             39
                    value: -1.23456789E-290,
                },
                Empty, //                               40
                Uint8 { //                              41
                    value: "Code".into()
                },
                Uint8 { //                              42
                    value: "LineNumberTable".into()
                },
                Uint8 { //                              43
                    value: "LocalVariableTable".into()
                },
                Uint8 { //                              44
                    value: "this".into()
                },
                Uint8 { //                              45
                    value: "LTrivial;".into()
                },
                Uint8 { //                              46
                    value: "LocalVariableTypeTable".into()
                },
                Uint8 { //                              47
                    value: "LTrivial<TT;>;".into()
                },
                Uint8 { //                              48
                    value: "MethodParameters".into()
                },
                Uint8 { //                              49
                    value: "add".into()
                },
                Uint8 { //                              50
                    value: "(II)I".into()
                },
                Uint8 { //                              51
                    value: "first".into()
                },
                Uint8 { //                              52
                    value: "second".into()
                },
                Uint8 { //                              53
                    value: "result".into()
                },
                Uint8 { //                              54
                    value: "Exceptions".into()
                },
                Class { //                              55
                    name_index: 56
                },
                Uint8 { //                              56
                    value: "java/lang/ClassNotFoundException".into()
                },
                Uint8 { //                              57
                    value: "run".into()
                },
                Uint8 { //                              58
                    value: "Signature".into()
                },
                Uint8 { //                              59
                    value: "<T:Ljava/lang/Object;>Ljava/lang/Object;Ljava/lang/Runnable;".into()
                },
                Uint8 { //                              60
                    value: "SourceFile".into()
                },
                Uint8 { //                              61
                    value: "Trivial.java".into()
                },
            ],
            0x0021,
            8,
            2,
            vec![16],
            vec![
                FieldInfo::new(0x0019, 18, 19, vec![ConstantValue { constantvalue_index: 21 }]),
                FieldInfo::new(0x001c, 22, 23, vec![ConstantValue { constantvalue_index: 24 }]),
                FieldInfo::new(0x001a, 25, 23, vec![ConstantValue { constantvalue_index: 26 }]),
                FieldInfo::new(0x001a, 27, 28, vec![ConstantValue { constantvalue_index: 29 }]),
                FieldInfo::new(0x001a, 31, 28, vec![ConstantValue { constantvalue_index: 32 }]),
                FieldInfo::new(0x001a, 34, 35, vec![ConstantValue { constantvalue_index: 36 }]),
                FieldInfo::new(0x001a, 38, 35, vec![ConstantValue { constantvalue_index: 39 }]),
                FieldInfo::new(0x0004, 11, 12, Vec::new()),
            ],
            vec![
                MethodInfo::new(0x0001, 5, 15, vec![
                    Code {
                        max_stack: 2,
                        max_locals: 2,
                        code: vec![0x2a, 0xb7, 0x00, 0x01, 0x2a, 0x2b, 0xb5, 0x00, 0x07, 0xb1],
                        exception_table: Vec::new(),
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
                                    LocalVariableTableRecord::new(0, 10, 44, 45, 0),
                                    LocalVariableTableRecord::new(0, 10, 11, 12, 1),
                                ]
                            },
                            LocalVariableTypeTable {
                                local_variable_type_table: vec![
                                    LocalVariableTypeTableRecord::new(0, 10, 44, 47, 0),
                                ]
                            },
                        ],
                    },
                    MethodParameters {
                        parameters: vec![
                            MethodParameterRecord::new(11, 0),
                        ]
                    },
                ],
                ),
                MethodInfo::new(0x0001, 5, 6, vec![
                    Code {
                        max_stack: 2,
                        max_locals: 1,
                        code: vec![0x2a, 0x01, 0xb7, 0x00, 0x0d, 0xb1],
                        exception_table: Vec::new(),
                        attributes: vec![
                            LineNumberTable {
                                line_number_table: vec![
                                    LineNumberRecord::new(0, 20),
                                    LineNumberRecord::new(5, 21),
                                ]
                            },
                            LocalVariableTable {
                                local_variable_table: vec![
                                    LocalVariableTableRecord::new(0, 6, 44, 45, 0)
                                ]
                            },
                            LocalVariableTypeTable {
                                local_variable_type_table: vec![
                                    LocalVariableTypeTableRecord::new(0, 6, 44, 47, 0),
                                ]
                            },
                        ],
                    }
                ],
                ),
                MethodInfo::new(0x0001, 49, 50, vec![
                    Code {
                        max_stack: 2,
                        max_locals: 4,
                        code: vec![0x1b, 0x1c, 0x60, 0x3e, 0x1d, 0xac],
                        exception_table: Vec::new(),
                        attributes: vec![
                            LineNumberTable {
                                line_number_table: vec![
                                    LineNumberRecord::new(0, 24),
                                    LineNumberRecord::new(4, 26),
                                ]
                            },
                            LocalVariableTable {
                                local_variable_table: vec![
                                    LocalVariableTableRecord::new(0, 6, 44, 45, 0),
                                    LocalVariableTableRecord::new(0, 6, 51, 23, 1),
                                    LocalVariableTableRecord::new(0, 6, 52, 23, 2),
                                    LocalVariableTableRecord::new(4, 2, 53, 23, 3),
                                ]
                            },
                            LocalVariableTypeTable {
                                local_variable_type_table: vec![
                                    LocalVariableTypeTableRecord::new(0, 6, 44, 47, 0),
                                ]
                            },
                        ],
                    },
                    Exceptions { exception_index_table: vec![55] },
                    MethodParameters {
                        parameters: vec![
                            MethodParameterRecord::new(51, 0),
                            MethodParameterRecord::new(52, 0x0010),
                        ]
                    },
                ]),
                MethodInfo::new(0x0001, 57, 6, vec![
                    Code {
                        max_stack: 0,
                        max_locals: 1,
                        code: vec![0xb1],
                        exception_table: Vec::new(),
                        attributes: vec![
                            LineNumberTable {
                                line_number_table: vec![
                                    LineNumberRecord::new(0, 32)]
                            },
                            LocalVariableTable {
                                local_variable_table: vec![
                                    LocalVariableTableRecord::new(0, 1, 44, 45, 0)]
                            },
                            LocalVariableTypeTable {
                                local_variable_type_table: vec![
                                    LocalVariableTypeTableRecord::new(0, 1, 44, 47, 0),
                                ]
                            },
                        ],
                    }
                ],
                ),
            ],
            vec![
                Signature { signature_index: 59 },
                SourceFile { sourcefile_index: 61 },
            ],
        );

        assert_eq!(actual_class_file, expected_class_file)
    }
}
