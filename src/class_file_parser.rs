use std::io;
use std::io::ErrorKind::{InvalidData, InvalidInput};
use mutf8::mutf8_to_utf8;
use crate::class_file::*;
use crate::class_file::ConstantPool::*;
use crate::class_file::Attribute::*;
use crate::class_file::ElementValue::*;
use crate::class_file::StackMapFrame::*;
use crate::extractors::{read_int, read_bytes};


const MAGIC: u32 = 0xCAFEBABE;


pub fn parse(data: &[u8]) -> Result<ClassFile, io::Error> {
    let mut start_from = 0;
    let magic = read_int(&data, &mut start_from)?;

    if magic != MAGIC {
        return Err(io::Error::new(InvalidInput, "Not a valid class-file"));
    }

    let minor_version = read_int(&data, &mut start_from)?;
    let major_version = read_int(&data, &mut start_from)?;
    let constant_pool_vec = get_constant_pool(&data, &mut start_from)?;
    let access_flags = read_int(&data, &mut start_from)?;
    let this_class = read_int(&data, &mut start_from)?;
    let super_class = read_int(&data, &mut start_from)?;
    let interfaces = get_interfaces(&data, &mut start_from)?;
    let fields = get_fields(&data, &mut start_from, &constant_pool_vec)?;
    let methods = get_methods(&data, &mut start_from, &constant_pool_vec)?;
    let attributes = get_attributes(&data, &mut start_from, &constant_pool_vec)?;

    if data.len() != start_from {
        return Err(
            io::Error::new(
                InvalidInput,
                format!("Not all was read : data.len() is {}, start_from is {}", data.len(), start_from)),
        );
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

fn get_methods(data: &&[u8], mut start_from: &mut usize, constant_pool_vec: &Vec<ConstantPool>) -> Result<Vec<MethodInfo>, io::Error> {
    let methods_count: u16 = read_int(&data, &mut start_from)?;
    let mut methods = Vec::with_capacity(methods_count as usize);
    for _ in 0..methods_count {
        methods.push(MethodInfo::new(
            read_int(&data, &mut start_from)?,
            read_int(&data, &mut start_from)?,
            read_int(&data, &mut start_from)?,
            get_attributes(&data, &mut start_from, &constant_pool_vec)?,
        ))
    }

    Ok(methods)
}

fn get_fields(data: &&[u8], mut start_from: &mut usize, constant_pool_vec: &Vec<ConstantPool>) -> Result<Vec<FieldInfo>, io::Error> {
    let fields_count: u16 = read_int(&data, &mut start_from)?;

    let mut fields = Vec::with_capacity(fields_count as usize);
    for _ in 0..fields_count {
        fields.push(FieldInfo::new(
            read_int(&data, &mut start_from)?,
            read_int(&data, &mut start_from)?,
            read_int(&data, &mut start_from)?,
            get_attributes(&data, &mut start_from, &constant_pool_vec)?,
        ))
    }

    Ok(fields)
}

fn get_interfaces(data: &&[u8], mut start_from: &mut usize) -> Result<Vec<u16>, io::Error> {
    let interfaces_count: u16 = read_int(&data, &mut start_from)?;

    let mut interfaces = Vec::with_capacity(interfaces_count as usize);
    for _ in 0..interfaces_count {
        interfaces.push(read_int(&data, &mut start_from)?);
    }

    Ok(interfaces)
}

fn get_constant_pool(data: &&[u8], mut start_from: &mut usize) -> Result<Vec<ConstantPool>, io::Error> {
    let constant_pool_count: u16 = read_int(&data, &mut start_from)?;
    let mut constant_pool_vec = Vec::with_capacity(constant_pool_count as usize);
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

        let tag: u8 = read_int(&data, &mut start_from)?;

        let constant_pool_entry = match tag {
            1 => Utf8 {
                value: get_string(&data, &mut start_from)?
            },
            3 => Integer {
                value: read_int(&data, &mut start_from)?
            },
            4 => Float {
                value: get_float(&data, &mut start_from)?
            },
            5 => Long {
                value: read_int(&data, &mut start_from)?
            },
            6 => Double {
                value: get_double(&data, &mut start_from)?
            },
            7 => Class {
                name_index: read_int(&data, &mut start_from)?,
            },
            8 => String {
                string_index: read_int(&data, &mut start_from)?,
            },
            9 => Fieldref {
                class_index: read_int(&data, &mut start_from)?,
                name_and_type_index: read_int(&data, &mut start_from)?,
            },
            10 => Methodref {
                class_index: read_int(&data, &mut start_from)?,
                name_and_type_index: read_int(&data, &mut start_from)?,
            },
            11 => InterfaceMethodref {
                class_index: read_int(&data, &mut start_from)?,
                name_and_type_index: read_int(&data, &mut start_from)?,
            },
            12 => NameAndType {
                name_index: read_int(&data, &mut start_from)?,
                descriptor_index: read_int(&data, &mut start_from)?,
            },
            15 => MethodHandle {
                reference_kind: read_int(&data, &mut start_from)?,
                reference_index: read_int(&data, &mut start_from)?,
            },
            16 => MethodType {
                descriptor_index: read_int(&data, &mut start_from)?,
            },
            17 => Dynamic {
                bootstrap_method_attr_index: read_int(&data, &mut start_from)?,
                name_and_type_index: read_int(&data, &mut start_from)?,
            },
            18 => InvokeDynamic {
                bootstrap_method_attr_index: read_int(&data, &mut start_from)?,
                name_and_type_index: read_int(&data, &mut start_from)?,
            },
            19 => ConstantPool::Module {
                name_index: read_int(&data, &mut start_from)?,
            },
            20 => Package {
                name_index: read_int(&data, &mut start_from)?,
            },
            _ => {
                return Err(io::Error::new(InvalidInput, format!("unmatched tag: {}", tag)));
            }
        };

        constant_pool_vec.push(constant_pool_entry);
    }

    Ok(constant_pool_vec)
}

fn get_double(data: &&[u8], mut start_from: &mut usize) -> Result<f64, io::Error> {
    let bytes: u64 = read_int(&data, &mut start_from)?;

    Ok(match bytes {
        0x7ff0000000000000_u64 => f64::INFINITY,
        0xfff0000000000000_u64 => f64::NEG_INFINITY,
        0x7ff0000000000001_u64..=0x7fffffffffffffff_u64 | 0xfff0000000000001_u64..=0xffffffffffffffff_u64 => f64::NAN,
        _ => {
            let s = if (bytes >> 63) == 0 { 1 } else { -1 };
            let e = ((bytes >> 52) & 0x7ff_u64) as i32;
            let m = (if e == 0 { (bytes & 0xfffffffffffff_u64) << 1 } else { (bytes & 0xfffffffffffff_u64) | 0x10000000000000_u64 }) as i64;

            s as f64 * m as f64 * 2_f64.powi(e - 1075)
        }
    })
}

fn get_float(data: &&[u8], mut start_from: &mut usize) -> Result<f32, io::Error> {
    let bytes: u32 = read_int(&data, &mut start_from)?;

    Ok(match bytes {
        0x7f800000 => f32::INFINITY,
        0xff800000 => f32::NEG_INFINITY,
        0x7f800001..=0x7fffffff | 0xff800001..=0xffffffff => f32::NAN,
        _ => {
            let s = if (bytes >> 31) == 0 { 1 } else { -1 };
            let e = ((bytes >> 23) & 0xff) as i32;
            let m = (if e == 0 { (bytes & 0x7fffff) << 1 } else { (bytes & 0x7fffff) | 0x800000 }) as i32;

            s as f32 * m as f32 * 2_f32.powi(e - 150)
        }
    })
}

fn get_string(data: &&[u8], mut start_from: &mut usize) -> Result<std::string::String, io::Error> {
    let length: u16 = read_int(&data, &mut start_from)?;
    let mutf8_bytes: &[u8] = read_bytes(&data, &mut start_from, length as usize)?;
    let utf8_bytes = mutf8_to_utf8(mutf8_bytes)
        .map_err(|e| io::Error::new(InvalidData, e))?
        .to_vec();

    std::string::String::from_utf8(utf8_bytes)
        .map_err(|e| io::Error::new(InvalidData, e))
}

fn get_attributes(data: &[u8], mut start_from: &mut usize, constant_pool_vec: &Vec<ConstantPool>) -> Result<Vec<Attribute>, io::Error> {
    let attributes_count: u16 = read_int(&data, &mut start_from)?;
    let mut attributes = Vec::with_capacity(attributes_count as usize);
    for _ in 0..attributes_count {
        attributes.push(get_attribute(&data, &mut start_from, constant_pool_vec)?);
    }

    Ok(attributes)
}

fn get_attribute(data: &[u8], mut start_from: &mut usize, constant_pool_vec: &Vec<ConstantPool>) -> Result<Attribute, io::Error> {
    let attribute_name_index: u16 = read_int(&data, &mut start_from)?;
    let attribute_name = match constant_pool_vec.get(attribute_name_index as usize) {
        Some(item) => match item {
            Utf8 { value } => value,
            _ => return Err(io::Error::new(InvalidData, format!("element type is not Uint8 but {:?}", item)))
        },
        None => return Err(io::Error::new(InvalidData, format!("element not found at index {}", attribute_name_index)))
    };

    let _attribute_length: u32 = read_int(&data, &mut start_from)?;

    let attribute = match attribute_name.as_str() {
        "ConstantValue" => ConstantValue {
            constantvalue_index: read_int(&data, &mut start_from)?,
        },
        "Code" => {
            let max_stack = read_int(&data, &mut start_from)?;
            let max_locals = read_int(&data, &mut start_from)?;
            let code_length: u32 = read_int(&data, &mut start_from)?;
            let code = read_bytes(&data, &mut start_from, code_length as usize)?.to_vec();
            let exception_table_length: u16 = read_int(&data, &mut start_from)?;

            let mut exception_table = Vec::with_capacity(exception_table_length as usize);
            for _ in 0..exception_table_length {
                exception_table.push(ExceptionRecord::new(
                    read_int(&data, &mut start_from)?,
                    read_int(&data, &mut start_from)?,
                    read_int(&data, &mut start_from)?,
                    read_int(&data, &mut start_from)?,
                ));
            }

            let attributes = get_attributes(&data, &mut start_from, constant_pool_vec)?;

            Code {
                max_stack,
                max_locals,
                code,
                exception_table,
                attributes,
            }
        }
        "Exceptions" => {
            let number_of_exceptions: u16 = read_int(&data, &mut start_from)?;
            let mut exception_index_table = Vec::with_capacity(number_of_exceptions as usize);
            for _ in 0..number_of_exceptions {
                exception_index_table.push(read_int(&data, &mut start_from)?);
            }
            Exceptions {
                exception_index_table
            }
        }
        "Synthetic" => Synthetic,
        "Deprecated" => Deprecated,
        "SourceFile" => SourceFile {
            sourcefile_index: read_int(&data, &mut start_from)?,
        },
        "LineNumberTable" => {
            let line_number_table_length: u16 = read_int(&data, &mut start_from)?;
            let mut line_number_table = Vec::with_capacity(line_number_table_length as usize);
            for _ in 0..line_number_table_length {
                line_number_table.push(LineNumberRecord::new(
                    read_int(&data, &mut start_from)?,
                    read_int(&data, &mut start_from)?,
                ));
            }
            LineNumberTable {
                line_number_table,
            }
        }
        "LocalVariableTable" => {
            let local_variable_table_length: u16 = read_int(&data, &mut start_from)?;
            let mut local_variable_table = Vec::with_capacity(local_variable_table_length as usize);
            for _ in 0..local_variable_table_length {
                local_variable_table.push(LocalVariableTableRecord::new(
                    read_int(&data, &mut start_from)?,
                    read_int(&data, &mut start_from)?,
                    read_int(&data, &mut start_from)?,
                    read_int(&data, &mut start_from)?,
                    read_int(&data, &mut start_from)?,
                ))
            }

            LocalVariableTable {
                local_variable_table
            }
        }
        "InnerClasses" => {
            let number_of_classes: u16 = read_int(&data, &mut start_from)?;
            let mut classes = Vec::with_capacity(number_of_classes as usize);
            for _ in 0..number_of_classes {
                classes.push(InnerClassRecord::new(
                    read_int(&data, &mut start_from)?,
                    read_int(&data, &mut start_from)?,
                    read_int(&data, &mut start_from)?,
                    read_int(&data, &mut start_from)?,
                ));
            }

            InnerClasses {
                classes
            }
        }
        "EnclosingMethod" => EnclosingMethod {
            class_index: read_int(&data, &mut start_from)?,
            method_index: read_int(&data, &mut start_from)?,
        },
        "Signature" => Signature {
            signature_index: read_int(&data, &mut start_from)?,
        },
        "LocalVariableTypeTable" => {
            let local_variable_type_table_length: u16 = read_int(&data, &mut start_from)?;
            let mut local_variable_type_table = Vec::with_capacity(local_variable_type_table_length as usize);
            for _ in 0..local_variable_type_table_length {
                local_variable_type_table.push(LocalVariableTypeTableRecord::new(
                    read_int(&data, &mut start_from)?,
                    read_int(&data, &mut start_from)?,
                    read_int(&data, &mut start_from)?,
                    read_int(&data, &mut start_from)?,
                    read_int(&data, &mut start_from)?,
                ))
            }

            LocalVariableTypeTable {
                local_variable_type_table
            }
        }
        "RuntimeVisibleAnnotations" => {
            let num_annotations: u16 = read_int(&data, &mut start_from)?;
            let mut annotations = Vec::with_capacity(num_annotations as usize);
            for _ in 0..num_annotations {
                annotations.push(get_annotation(&data, &mut start_from)?);
            };

            RuntimeVisibleAnnotations {
                annotations
            }
        }
        "RuntimeInvisibleAnnotations" => {
            let num_annotations: u16 = read_int(&data, &mut start_from)?;
            let mut annotations = Vec::with_capacity(num_annotations as usize);
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
            let number_of_entries: u16 = read_int(&data, &mut start_from)?;
            let mut entries = Vec::with_capacity(number_of_entries as usize);
            for _ in 0..number_of_entries {
                let frame_type = read_int(&data, &mut start_from)?;
                let stack_map_frame = match frame_type {
                    0..=63 => SameFrame {
                        frame_type,
                        offset_delta: frame_type as u16,
                    },
                    64..=127 => {
                        let tag: u8 = read_int(&data, &mut start_from)?;

                        SameLocals1StackItemFrame {
                            frame_type,
                            offset_delta: frame_type as u16 - 64,
                            stack: get_verification_type_info(tag, &data, &mut start_from)?,
                        }
                    }
                    247 => {
                        let offset_delta = read_int(&data, &mut start_from)?;
                        let tag = read_int(&data, &mut start_from)?;

                        SameLocals1StackItemFrameExtended {
                            frame_type,
                            offset_delta,
                            stack: get_verification_type_info(tag, &data, &mut start_from)?,
                        }
                    }
                    248..=250 => ChopFrame {
                        frame_type,
                        offset_delta: read_int(&data, &mut start_from)?,
                    },
                    251 => SameFrameExtended {
                        frame_type,
                        offset_delta: read_int(&data, &mut start_from)?,
                    },
                    252..=254 => {
                        let offset_delta = read_int(&data, &mut start_from)?;
                        let size = frame_type - 251;
                        let mut locals = Vec::with_capacity(size as usize);
                        for _ in 0..size {
                            let tag: u8 = read_int(&data, &mut start_from)?;
                            locals.push(get_verification_type_info(tag, &data, &mut start_from)?);
                        }
                        AppendFrame {
                            frame_type,
                            offset_delta,
                            locals,
                        }
                    }
                    255 => {
                        let offset_delta = read_int(&data, &mut start_from)?;

                        let number_of_locals: u16 = read_int(&data, &mut start_from)?;
                        let mut locals = Vec::with_capacity(number_of_locals as usize);
                        for _ in 0..number_of_locals {
                            let tag: u8 = read_int(&data, &mut start_from)?;
                            locals.push(get_verification_type_info(tag, &data, &mut start_from)?);
                        }

                        let number_of_stack_items: u16 = read_int(&data, &mut start_from)?;
                        let mut stack = Vec::with_capacity(number_of_stack_items as usize);
                        for _ in 0..number_of_stack_items {
                            let tag: u8 = read_int(&data, &mut start_from)?;
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
            let num_bootstrap_methods: u16 = read_int(&data, &mut start_from)?;
            let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods as usize);
            for _ in 0..num_bootstrap_methods {
                let bootstrap_method_ref = read_int(&data, &mut start_from)?;
                let num_bootstrap_arguments: u16 = read_int(&data, &mut start_from)?;
                let mut bootstrap_arguments = Vec::with_capacity(num_bootstrap_arguments as usize);
                for _ in 0..num_bootstrap_arguments {
                    bootstrap_arguments.push(read_int(&data, &mut start_from)?);
                }
                bootstrap_methods.push(BootstrapMethodRecord::new(bootstrap_method_ref, bootstrap_arguments))
            }

            BootstrapMethods { bootstrap_methods }
        }
        "MethodParameters" => {
            let parameters_count: u8 = read_int(&data, &mut start_from)?;
            let mut parameters = Vec::with_capacity(parameters_count as usize);
            for _ in 0..parameters_count {
                parameters.push(MethodParameterRecord::new(
                    read_int(&data, &mut start_from)?,
                    read_int(&data, &mut start_from)?,
                ))
            }
            MethodParameters {
                parameters
            }
        }
        "NestHost" => NestHost {
            host_class_index: read_int(&data, &mut start_from)?,
        },
        "NestMembers" => {
            let number_of_classes: u16 = read_int(&data, &mut start_from)?;
            let mut classes = Vec::with_capacity(number_of_classes as usize);
            for _ in 0..number_of_classes {
                classes.push(read_int(&data, &mut start_from)?);
            }

            NestMembers {
                classes
            }
        }
        "Record" => {
            let components_count: u16 = read_int(&data, &mut start_from)?;
            let mut components = Vec::with_capacity(components_count as usize);
            for _ in 0..components_count {
                components.push(RecordComponentInfo::new(
                    read_int(&data, &mut start_from)?,
                    read_int(&data, &mut start_from)?,
                    get_attributes(&data, &mut start_from, &constant_pool_vec)?),
                )
            }

            Record {
                components
            }
        }
        "PermittedSubclasses" => {
            let number_of_classes: u16 = read_int(&data, &mut start_from)?;
            let mut classes = Vec::with_capacity(number_of_classes as usize);
            for _ in 0..number_of_classes {
                classes.push(read_int(&data, &mut start_from)?);
            }

            PermittedSubclasses {
                classes
            }
        }
        _ => {
            return Err(io::Error::new(InvalidInput, format!("unmatched attribute: {}", attribute_name)));
        }
    };

    Ok(attribute)
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
            cpool_index: read_int(&data, start_from)?,
        }),
        8 => Ok(VerificationTypeInfo::UninitializedVariableInfo {
            offset: read_int(&data, start_from)?,
        }),
        _ => Err(io::Error::new(InvalidInput, format!("tag {} is not valid", tag)))
    }
}

fn get_annotation(data: &[u8], start_from: &mut usize) -> Result<Annotation, io::Error> {
    let type_index = read_int(&data, start_from)?;
    let num_element_value_pairs: u16 = read_int(&data, start_from)?;
    let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
    for _ in 0..num_element_value_pairs {
        let element_name_index: u16 = read_int(&data, start_from)?;
        let value = get_element_value(&data, start_from)?;
        element_value_pairs.push(ElementValuePair::new(element_name_index, value));
    }

    Ok(Annotation::new(type_index, element_value_pairs))
}

fn get_element_value(data: &[u8], start_from: &mut usize) -> Result<ElementValue, io::Error> {
    let tag: u8 = read_int(&data, start_from)?;
    match tag {
        b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' | b's' => Ok(ConstValueIndex {
            tag,
            const_value_index: read_int(&data, start_from)?,
        }),
        b'e' => Ok(EnumConstValue {
            tag,
            type_name_index: read_int(&data, start_from)?,
            const_name_index: read_int(&data, start_from)?,
        }),
        b'c' => Ok(ClassInfoIndex {
            tag,
            class_info_index: read_int(&data, start_from)?,
        }),
        b'@' => Ok(AnnotationValue {
            tag,
            annotation_value: get_annotation(&data, start_from)?,
        }),
        b'[' => {
            let num_values: u16 = read_int(&data, start_from)?;
            let mut values = Vec::with_capacity(num_values as usize);
            for _ in 0..num_values {
                values.push(get_element_value(&data, start_from)?);
            }

            Ok(ArrayValue {
                tag,
                values,
            })
        }
        _ => Err(io::Error::new(InvalidInput, format!("Unsupported element tag: {}", tag)))
    }
}
