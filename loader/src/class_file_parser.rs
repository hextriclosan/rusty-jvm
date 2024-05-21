use std::io;
use std::io::ErrorKind::{InvalidData, InvalidInput};
use std::mem::size_of;
use num_traits::Num;
use model::class_file::{Annotation, Attribute, BootstrapMethodRecord, ClassFile, ConstantPool, ElementValue, ElementValuePair, ExceptionRecord, FieldInfo, InnerClassRecord, LineNumberRecord, LocalVariableTableRecord, LocalVariableTypeTableRecord, MethodInfo, MethodParameterRecord, RecordComponentInfo, VerificationTypeInfo};
use model::class_file::ConstantPool::*;
use model::class_file::Attribute::{AnnotationDefault, BootstrapMethods, Code, ConstantValue, Deprecated, EnclosingMethod, Exceptions, InnerClasses, LineNumberTable, LocalVariableTable, LocalVariableTypeTable, MethodParameters, NestHost, NestMembers, PermittedSubclasses, Record, RuntimeInvisibleAnnotations, RuntimeVisibleAnnotations, Signature, SourceFile, StackMapTable, Synthetic};
use model::class_file::ElementValue::{AnnotationValue, ArrayValue, ClassInfoIndex, ConstValueIndex, EnumConstValue};
use model::class_file::StackMapFrame::{AppendFrame, ChopFrame, FullFrame, SameFrame, SameFrameExtended, SameLocals1StackItemFrame, SameLocals1StackItemFrameExtended};

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

                    Utf8 {
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
                Utf8 { value } => value,
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
            "InnerClasses" => {
                let number_of_classes: u16 = convert(&data, &mut start_from)?;
                let mut classes = Vec::new();
                for _ in 0..number_of_classes {
                    classes.push(InnerClassRecord::new(
                        convert(&data, &mut start_from)?,
                        convert(&data, &mut start_from)?,
                        convert(&data, &mut start_from)?,
                        convert(&data, &mut start_from)?,
                    ));
                }

                InnerClasses {
                    classes
                }
            }
            "EnclosingMethod" => EnclosingMethod {
                class_index: convert(&data, &mut start_from)?,
                method_index: convert(&data, &mut start_from)?,
            },
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
            "RuntimeVisibleAnnotations" => {
                let num_annotations: u16 = convert(&data, &mut start_from)?;
                let mut annotations = Vec::new();
                for _ in 0..num_annotations {
                    annotations.push(get_annotation(&data, &mut start_from)?);
                };

                RuntimeVisibleAnnotations {
                    annotations
                }
            }
            "RuntimeInvisibleAnnotations" => {
                let num_annotations: u16 = convert(&data, &mut start_from)?;
                let mut annotations = Vec::new();
                for _ in 0..num_annotations {
                    annotations.push(get_annotation(&data, &mut start_from)?);
                };

                RuntimeInvisibleAnnotations {
                    annotations
                }
            }
            "AnnotationDefault" => AnnotationDefault {
                default_value: get_element_value(&data, &mut start_from)?
            },
            "StackMapTable" => {
                let number_of_entries: u16 = convert(&data, &mut start_from)?;
                let mut entries = Vec::new();
                for _ in 0..number_of_entries {
                    let frame_type: u8 = convert(&data, &mut start_from)?;
                    let stack_map_frame = match frame_type {
                        0..=63 => SameFrame {
                            frame_type,
                            offset_delta: frame_type as u16,
                        },
                        64..=127 => {
                            let tag: u8 = convert(&data, &mut start_from)?;

                            SameLocals1StackItemFrame {
                                frame_type,
                                offset_delta: frame_type as u16 - 64,
                                stack: get_verification_type_info(tag, &data, &mut start_from)?,
                            }
                        }
                        247 => {
                            let offset_delta: u16 = convert(&data, &mut start_from)?;
                            let tag: u8 = convert(&data, &mut start_from)?;

                            SameLocals1StackItemFrameExtended {
                                frame_type,
                                offset_delta,
                                stack: get_verification_type_info(tag, &data, &mut start_from)?,
                            }
                        }
                        248..=250 => ChopFrame {
                            frame_type,
                            offset_delta: convert(&data, &mut start_from)?,
                        },
                        251 => SameFrameExtended {
                            frame_type,
                            offset_delta: convert(&data, &mut start_from)?,
                        },
                        252..=254 => {
                            let offset_delta: u16 = convert(&data, &mut start_from)?;
                            let mut locals = Vec::new();
                            let size = frame_type - 251;
                            for _ in 0..size {
                                let tag: u8 = convert(&data, &mut start_from)?;
                                locals.push(get_verification_type_info(tag, &data, &mut start_from)?);
                            }
                            AppendFrame {
                                frame_type,
                                offset_delta,
                                locals,
                            }
                        }
                        255 => {
                            let offset_delta: u16 = convert(&data, &mut start_from)?;

                            let number_of_locals: u16 = convert(&data, &mut start_from)?;
                            let mut locals = Vec::new();
                            for _ in 0..number_of_locals {
                                let tag: u8 = convert(&data, &mut start_from)?;
                                locals.push(get_verification_type_info(tag, &data, &mut start_from)?);
                            }

                            let number_of_stack_items: u16 = convert(&data, &mut start_from)?;
                            let mut stack = Vec::new();
                            for _ in 0..number_of_stack_items {
                                let tag: u8 = convert(&data, &mut start_from)?;
                                stack.push(get_verification_type_info(tag, &data, &mut start_from)?);
                            }

                            FullFrame {
                                frame_type,
                                offset_delta,
                                locals,
                                stack,
                            }
                        }
                        _ => {
                            return Err(io::Error::new(InvalidInput, format!("Unsupported frame_type: {}", frame_type)));
                        }
                    };

                    entries.push(stack_map_frame);
                }

                StackMapTable {
                    entries
                }
            }
            "BootstrapMethods" => {
                let num_bootstrap_methods: u16 = convert(&data, &mut start_from)?;
                let mut bootstrap_methods = Vec::new();
                for _ in 0..num_bootstrap_methods {
                    let bootstrap_method_ref = convert(&data, &mut start_from)?;
                    let num_bootstrap_arguments: u16 = convert(&data, &mut start_from)?;
                    let mut bootstrap_arguments = Vec::new();
                    for _ in 0..num_bootstrap_arguments {
                        bootstrap_arguments.push(convert(&data, &mut start_from)?);
                    }
                    bootstrap_methods.push(BootstrapMethodRecord::new(bootstrap_method_ref, bootstrap_arguments))
                }

                BootstrapMethods { bootstrap_methods }
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
            "NestHost" => NestHost {
                host_class_index: convert(&data, &mut start_from)?,
            },
            "NestMembers" => {
                let number_of_classes: u16 = convert(&data, &mut start_from)?;
                let mut classes = Vec::new();
                for _ in 0..number_of_classes {
                    classes.push(convert(&data, &mut start_from)?);
                }

                NestMembers {
                    classes
                }
            }
            "Record" => {
                let components_count: u16 = convert(&data, &mut start_from)?;
                let mut components = Vec::new();
                for _ in 0..components_count {
                    components.push(RecordComponentInfo::new(
                        convert(&data, &mut start_from)?,
                        convert(&data, &mut start_from)?,
                        Self::parse_attributes(&data, &mut start_from, &constant_pool_vec)?),
                    )
                }

                Record {
                    components
                }
            }
            "PermittedSubclasses" => {
                let number_of_classes: u16 = convert(&data, &mut start_from)?;
                let mut classes = Vec::new();
                for _ in 0..number_of_classes {
                    classes.push(convert(&data, &mut start_from)?);
                }

                PermittedSubclasses {
                    classes
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

fn get_verification_type_info(tag: u8, data: &[u8], start_from: &mut usize) -> Result<VerificationTypeInfo, io::Error> {
    match tag {
        0 => Ok(VerificationTypeInfo::TopVariableInfo),
        1 => Ok(VerificationTypeInfo::IntegerVariableInfo),
        2 => Ok(VerificationTypeInfo::FloatVariableInfo),
        3 => Ok(VerificationTypeInfo::LongVariableInfo),
        4 => Ok(VerificationTypeInfo::DoubleVariableInfo),
        5 => Ok(VerificationTypeInfo::NullVariableInfo),
        6 => Ok(VerificationTypeInfo::UninitializedThisVariableInfo),
        7 => Ok(VerificationTypeInfo::ObjectVariableInfo {
            cpool_index: convert(&data, start_from)?,
        }),
        8 => Ok(VerificationTypeInfo::UninitializedVariableInfo {
            offset: convert(&data, start_from)?,
        }),
        _ => Err(io::Error::new(InvalidInput, format!("tag {} is not valid", tag)))
    }
}

fn get_annotation(data: &[u8], start_from: &mut usize) -> Result<Annotation, io::Error> {
    let type_index: u16 = convert(&data, start_from)?;
    let num_element_value_pairs: u16 = convert(&data, start_from)?;
    let mut element_value_pairs = Vec::new();
    for _ in 0..num_element_value_pairs {
        let element_name_index: u16 = convert(&data, start_from)?;
        let value = get_element_value(&data, start_from)?;
        element_value_pairs.push(ElementValuePair::new(element_name_index, value));
    }

    Ok(Annotation::new(type_index, element_value_pairs))
}

fn get_element_value(data: &[u8], start_from: &mut usize) -> Result<ElementValue, io::Error> {
    let tag: u8 = convert(&data, start_from)?;
    match tag {
        b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' | b's' => Ok(ConstValueIndex {
            tag,
            const_value_index: convert(&data, start_from)?,
        }),
        b'e' => Ok(EnumConstValue {
            tag,
            type_name_index: convert(&data, start_from)?,
            const_name_index: convert(&data, start_from)?,
        }),
        b'c' => Ok(ClassInfoIndex {
            tag,
            class_info_index: convert(&data, start_from)?,
        }),
        b'@' => Ok(AnnotationValue {
            tag,
            annotation_value: get_annotation(&data, start_from)?,
        }),
        b'[' => {
            let num_values: u16 = convert(&data, start_from)?;
            let mut values = Vec::new();
            for _ in 0..num_values {
                values.push(get_element_value(&data, start_from)?);
            }

            Ok(ArrayValue {
                tag,
                values,
            })
        }
        _ => Err(io::Error::new(InvalidInput, format!("Unsupported tag: {}", tag)))
    }
}

#[cfg(test)]
mod tests {
    use model::class_file::{Annotation, BootstrapMethodRecord, ClassFile, ConstantPool, ElementValuePair, FieldInfo, InnerClassRecord, LineNumberRecord, LocalVariableTableRecord, LocalVariableTypeTableRecord, MethodInfo, MethodParameterRecord, RecordComponentInfo};
    use model::class_file::Attribute::{AnnotationDefault, BootstrapMethods, Code, ConstantValue, EnclosingMethod, Exceptions, InnerClasses, LineNumberTable, LocalVariableTable, LocalVariableTypeTable, MethodParameters, NestHost, NestMembers, PermittedSubclasses, Record, RuntimeInvisibleAnnotations, RuntimeVisibleAnnotations, Signature, SourceFile, StackMapTable};
    use model::class_file::ConstantPool::{Class, Double, Empty, Fieldref, Float, Integer, InvokeDynamic, Long, MethodHandle, Methodref, MethodType, NameAndType, Utf8};
    use model::class_file::ElementValue::{ArrayValue, ConstValueIndex, EnumConstValue};
    use model::class_file::StackMapFrame::{AppendFrame, SameLocals1StackItemFrame};
    use model::class_file::VerificationTypeInfo::IntegerVariableInfo;
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
                    value: -1.23456789E-290,
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
            0x0021,
            8,
            2,
            vec![20],
            vec![
                FieldInfo::new(0x0019, 22, 23, vec![ConstantValue { constantvalue_index: 25 }]),
                FieldInfo::new(0x001c, 26, 27, vec![ConstantValue { constantvalue_index: 28 }]),
                FieldInfo::new(0x001a, 29, 27, vec![ConstantValue { constantvalue_index: 30 }]),
                FieldInfo::new(0x001a, 31, 32, vec![ConstantValue { constantvalue_index: 33 }]),
                FieldInfo::new(0x001a, 35, 32, vec![ConstantValue { constantvalue_index: 36 }]),
                FieldInfo::new(0x001a, 38, 39, vec![ConstantValue { constantvalue_index: 40 }]),
                FieldInfo::new(0x001a, 42, 39, vec![ConstantValue { constantvalue_index: 43 }]),
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
                MethodInfo::new(0x0001, 53, 54, vec![
                    Code {
                        max_stack: 2,
                        max_locals: 4,
                        code: vec![0x1b, 0x1c, 0x60, 0x3e, 0x1d, 0x9e, 0x00, 0x07, 0x1d, 0xa7, 0x00, 0x04, 0x03, 0xac],
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
                            MethodParameterRecord::new(55, 0),
                            MethodParameterRecord::new(56, 0x0010),
                        ]
                    },
                ]),
                MethodInfo::new(0x0001, 18, 6, vec![
                    Code {
                        max_stack: 1,
                        max_locals: 2,
                        code: vec![0xba, 0x00, 0x10, 0x00, 0x00, 0x4c, 0xb1],
                        exception_table: Vec::new(),
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
                MethodInfo::new(0x100A, 64, 6, vec![
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
                        InnerClassRecord::new(70, 8, 87, 0),
                        InnerClassRecord::new(72, 0, 88, 0x0608),
                        InnerClassRecord::new(89, 91, 93, 0x19),
                    ]
                },
            ],
        );

        assert_eq!(actual_class_file, expected_class_file)
    }

    #[test]
    fn should_load_and_parse_complex_runtime_visible_annotations() {
        let actual_class_file = load("../test_data/FunctionalInterface.class").unwrap();

        let expected_class_file = ClassFile::new(
            0xCAFEBABE,
            0,
            61,
            vec![
                Empty, //                                               0
                Class { //                                              1
                    name_index: 2,
                },
                Utf8 { //                                               2
                    value: "fake/java/lang/FunctionalInterface".into(),
                },
                Class { //                                              3
                    name_index: 4,
                },
                Utf8 { //                                               4
                    value: "java/lang/Object".into(),
                },
                Class { //                                              5
                    name_index: 6,
                },
                Utf8 { //                                               6
                    value: "java/lang/annotation/Annotation".into(),
                },
                Utf8 { //                                               7
                    value: "SourceFile".into(),
                },
                Utf8 { //                                               8
                    value: "FunctionalInterface.java".into(),
                },
                Utf8 { //                                               9
                    value: "RuntimeVisibleAnnotations".into(),
                },
                Utf8 { //                                               10
                    value: "Ljava/lang/annotation/Documented;".into(),
                },
                Utf8 { //                                               11
                    value: "Ljava/lang/annotation/Retention;".into(),
                },
                Utf8 { //                                               12
                    value: "value".into(),
                },
                Utf8 { //                                               13
                    value: "Ljava/lang/annotation/RetentionPolicy;".into(),
                },
                Utf8 { //                                               14
                    value: "RUNTIME".into(),
                },
                Utf8 { //                                               15
                    value: "Ljava/lang/annotation/Target;".into(),
                },
                Utf8 { //                                               16
                    value: "Ljava/lang/annotation/ElementType;".into(),
                },
                Utf8 { //                                               17
                    value: "TYPE".into(),
                },
            ],
            0x2601, // ACC_PUBLIC, ACC_INTERFACE, ACC_ABSTRACT, ACC_ANNOTATION
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
                        Annotation::new(11, vec![
                            ElementValuePair::new(12, EnumConstValue {
                                tag: 'e' as u8,
                                type_name_index: 13,
                                const_name_index: 14,
                            })
                        ]),
                        Annotation::new(15, vec![
                            ElementValuePair::new(12, ArrayValue {
                                tag: '[' as u8,
                                values: vec![EnumConstValue {
                                    tag: 'e' as u8,
                                    type_name_index: 16,
                                    const_name_index: 17,
                                }],
                            })
                        ]),
                    ]
                },
            ],
        );

        assert_eq!(actual_class_file, expected_class_file)
    }

    #[test]
    fn should_load_and_parse_complex_runtime_invisible_annotations() {
        let actual_class_file = load("../test_data/RuntimeInvisibleAnnotationUsage.class").unwrap();

        let expected_class_file = ClassFile::new(
            0xCAFEBABE,
            0,
            61,
            vec![
                Empty, //                                               0
                Class { //                                              1
                    name_index: 2,
                },
                Utf8 { //                                               2
                    value: "RuntimeInvisibleAnnotationUsage".into(),
                },
                Class { //                                              3
                    name_index: 4,
                },
                Utf8 { //                                               4
                    value: "java/lang/Object".into(),
                },
                Utf8 { //                                               5
                    value: "SourceFile".into(),
                },
                Utf8 { //                                               6
                    value: "RuntimeInvisibleAnnotationUsage.java".into(),
                },
                Utf8 { //                                               7
                    value: "RuntimeInvisibleAnnotations".into(),
                },
                Utf8 { //                                               8
                    value: "LRuntimeInvisibleAnnotation;".into(),
                },
                Utf8 { //                                               9
                    value: "value".into(),
                },
                Utf8 { //                                               10
                    value: "This is a runtime invisible annotation".into(),
                },
            ],
            0x0601, // ACC_PUBLIC, ACC_INTERFACE, ACC_ABSTRACT
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
                    annotations: vec![
                        Annotation::new(8, vec![
                            ElementValuePair::new(9, ConstValueIndex {
                                tag: 's' as u8,
                                const_value_index: 10,
                            })
                        ]),
                    ]
                },
            ],
        );

        assert_eq!(actual_class_file, expected_class_file)
    }

    #[test]
    fn should_load_and_parse_permitted_subclasses_annotation() {
        let actual_class_file = load("../test_data/Shape.class").unwrap();

        let expected_class_file = ClassFile::new(
            0xCAFEBABE,
            0,
            61,
            vec![
                Empty, //                                               0
                Class { //                                              1
                    name_index: 2,
                },
                Utf8 { //                                               2
                    value: "Shape".into(),
                },
                Class { //                                              3
                    name_index: 4,
                },
                Utf8 { //                                               4
                    value: "java/lang/Object".into(),
                },
                Utf8 { //                                               5
                    value: "SourceFile".into(),
                },
                Utf8 { //                                               6
                    value: "Shape.java".into(),
                },
                Utf8 { //                                               7
                    value: "PermittedSubclasses".into(),
                },
                Class { //                                              8
                    name_index: 9,
                },
                Utf8 { //                                               9
                    value: "Circle".into(),
                },
            ],
            0x0601, // ACC_PUBLIC, ACC_INTERFACE, ACC_ABSTRACT
            1,
            3,
            vec![],
            vec![],
            vec![],
            vec![
                SourceFile {
                    sourcefile_index: 6,
                },
                PermittedSubclasses {
                    classes: vec![8],
                },
            ],
        );

        assert_eq!(actual_class_file, expected_class_file)
    }

    #[test]
    fn should_load_and_parse_annotation_default_annotation() {
        let actual_class_file = load("../test_data/RuntimeInvisibleAnnotation.class").unwrap();

        let expected_class_file = ClassFile::new(
            0xCAFEBABE,
            0,
            61,
            vec![
                Empty, //                                               0
                Class { //                                              1
                    name_index: 2,
                },
                Utf8 { //                                               2
                    value: "RuntimeInvisibleAnnotation".into(),
                },
                Class { //                                              3
                    name_index: 4,
                },
                Utf8 { //                                               4
                    value: "java/lang/Object".into(),
                },
                Class { //                                              5
                    name_index: 6,
                },
                Utf8 { //                                               6
                    value: "java/lang/annotation/Annotation".into(),
                },
                Utf8 { //                                               7
                    value: "value".into(),
                },
                Utf8 { //                                               8
                    value: "()Ljava/lang/String;".into(),
                },
                Utf8 { //                                               9
                    value: "AnnotationDefault".into(),
                },
                Utf8 { //                                               10
                    value: "I'm a default value".into(),
                },
                Utf8 { //                                               11
                    value: "SourceFile".into(),
                },
                Utf8 { //                                               12
                    value: "RuntimeInvisibleAnnotationUsage.java".into(),
                },
                Utf8 { //                                               13
                    value: "RuntimeVisibleAnnotations".into(),
                },
                Utf8 { //                                               14
                    value: "Ljava/lang/annotation/Retention;".into(),
                },
                Utf8 { //                                               15
                    value: "Ljava/lang/annotation/RetentionPolicy;".into(),
                },
                Utf8 { //                                               16
                    value: "CLASS".into(),
                },
            ],
            0x2600, // ACC_INTERFACE, ACC_ABSTRACT, ACC_ANNOTATION
            1,
            3,
            vec![5],
            vec![],
            vec![
                MethodInfo::new(0x0401, 7, 8, vec![
                    AnnotationDefault {
                        default_value: ConstValueIndex {
                            tag: 's' as u8,
                            const_value_index: 10,
                        }
                    }
                ])
            ],
            vec![
                SourceFile {
                    sourcefile_index: 12,
                },
                RuntimeVisibleAnnotations {
                    annotations: vec![
                        Annotation::new(14, vec![
                            ElementValuePair::new(7, EnumConstValue {
                                tag: 'e' as u8,
                                type_name_index: 15,
                                const_name_index: 16,
                            })
                        ]),
                    ]
                },
            ],
        );

        assert_eq!(actual_class_file, expected_class_file)
    }

    #[test]
    fn should_load_and_parse_local_class() {
        let actual_class_file = load("../test_data/Trivial$1LocalCls.class").unwrap();

        let expected_class_file = ClassFile::new(
            0xCAFEBABE,
            0,
            61,
            vec![
                Empty, //                                               0
                Class { //                                              1
                    name_index: 2,
                },
                Utf8 { //                                               2
                    value: "Trivial$1LocalCls".into(),
                },
                Class { //                                              3
                    name_index: 4,
                },
                Utf8 { //                                               4
                    value: "java/lang/Object".into(),
                },
                Utf8 { //                                               5
                    value: "SourceFile".into(),
                },
                Utf8 { //                                               6
                    value: "Trivial.java".into(),
                },
                Utf8 { //                                               7
                    value: "EnclosingMethod".into(),
                },
                Class { //                                              8
                    name_index: 9,
                },
                Utf8 { //                                               9
                    value: "Trivial".into(),
                },
                NameAndType { //                                        10
                    name_index: 11,
                    descriptor_index: 12,
                },
                Utf8 { //                                               11
                    value: "run".into(),
                },
                Utf8 { //                                               12
                    value: "()V".into(),
                },
                Utf8 { //                                               13
                    value: "NestHost".into(),
                },
                Utf8 { //                                               14
                    value: "InnerClasses".into(),
                },
                Utf8 { //                                               15
                    value: "LocalCls".into(),
                },
            ],
            0x0600, // ACC_INTERFACE, ACC_ABSTRACT
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
                    classes: vec![
                        InnerClassRecord::new(1, 0, 15, 0x0608),
                    ]
                },
            ],
        );

        assert_eq!(actual_class_file, expected_class_file)
    }

    #[test]
    fn should_load_and_parse_record() {
        let actual_class_file = load("../test_data/Rcrd.class").unwrap();

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
                ConstantPool::String { //                               41
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
            0x0031, // ACC_PUBLIC, ACC_FINAL, ACC_SUPER
            8,
            2,
            vec![],
            vec![
                FieldInfo::new(0x0012, 11, 12, vec![])
            ],
            vec![
                MethodInfo::new(0x0001, 5, 25, vec![
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
                        parameters: vec![MethodParameterRecord::new(11, 0)],
                    },
                ]),
                MethodInfo::new(0x0011, 15, 32, vec![
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
                MethodInfo::new(0x0011, 19, 33, vec![
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
                MethodInfo::new(0x0011, 23, 34, vec![
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
                        parameters: vec![MethodParameterRecord::new(35, 0)],
                    },
                ]),
                MethodInfo::new(0x0001, 11, 33, vec![
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
                        InnerClassRecord::new(51, 53, 55, 0x0019),
                    ]
                },
            ],
        );

        assert_eq!(actual_class_file, expected_class_file)
    }
}
