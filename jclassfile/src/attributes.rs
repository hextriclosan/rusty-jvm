use crate::attributes::Attribute::*;
use crate::attributes::ElementValue::{
    AnnotationValue, ArrayValue, ClassInfoIndex, ConstValueIndex, EnumConstValue,
};
use crate::attributes::StackMapFrame::{
    AppendFrame, ChopFrame, FullFrame, SameFrame, SameFrameExtended, SameLocals1StackItemFrame,
    SameLocals1StackItemFrameExtended,
};
use crate::constant_pool::ConstantPool;
use crate::constant_pool::ConstantPool::*;
use crate::error::{Error, Result};
use crate::extractors::{get_bitfield, get_bytes, get_int, read_byte_block};
use bitflags::bitflags;
use derive_new::new;
use getset::{CopyGetters, Getters};
use std::io::ErrorKind::InvalidData;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `attribute_info` structure (JVMS §4.7).
pub enum Attribute {
    ConstantValue {
        constantvalue_index: u16,
    },
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<ExceptionRecord>,
        attributes: Vec<Attribute>,
    },
    Exceptions {
        exception_index_table: Vec<u16>,
    },
    SourceFile {
        sourcefile_index: u16,
    },
    LineNumberTable {
        line_number_table: Vec<LineNumberRecord>,
    },
    LocalVariableTable {
        local_variable_table: Vec<LocalVariableTableRecord>,
    },
    InnerClasses {
        classes: Vec<InnerClassRecord>,
    },
    Synthetic,
    Deprecated,
    EnclosingMethod {
        class_index: u16,
        method_index: u16,
    },
    Signature {
        signature_index: u16,
    },
    SourceDebugExtension,
    LocalVariableTypeTable {
        local_variable_type_table: Vec<LocalVariableTypeTableRecord>,
    },
    RuntimeVisibleAnnotations {
        annotations: Vec<Annotation>,
        raw: Vec<u8>,
    },
    RuntimeInvisibleAnnotations {
        annotations: Vec<Annotation>,
    },
    RuntimeVisibleParameterAnnotations {
        parameter_annotations: Vec<Vec<Annotation>>,
    },
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault {
        default_value: ElementValue,
        raw: Vec<u8>,
    },
    StackMapTable {
        entries: Vec<StackMapFrame>,
    },
    BootstrapMethods {
        bootstrap_methods: Vec<BootstrapMethodRecord>,
    },
    RuntimeVisibleTypeAnnotations {
        type_annotations: Vec<TypeAnnotation>,
    },
    RuntimeInvisibleTypeAnnotations,
    MethodParameters {
        parameters: Vec<MethodParameterRecord>,
    },
    Module,
    ModulePackages,
    ModuleMainClass,
    NestHost {
        host_class_index: u16,
    },
    NestMembers {
        classes: Vec<u16>,
    },
    Record {
        components: Vec<RecordComponentInfo>,
    },
    PermittedSubclasses {
        classes: Vec<u16>,
    },
}

#[derive(Debug, PartialEq, Clone, CopyGetters, new)]
#[get_copy = "pub"]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `exception_table` entry (JVMS §4.7.3).
pub struct ExceptionRecord {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

#[derive(Debug, PartialEq, Clone, CopyGetters, new)]
#[get_copy = "pub"]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `LineNumberTable` entry (JVMS §4.7.12).
pub struct LineNumberRecord {
    start_pc: u16,
    line_number: u16,
}

#[derive(Debug, PartialEq, Clone, CopyGetters, new)]
#[get_copy = "pub"]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `LocalVariableTable` entry (JVMS §4.7.13).
pub struct LocalVariableTableRecord {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

#[derive(Debug, PartialEq, Clone, CopyGetters, new)]
#[get_copy = "pub"]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `LocalVariableTypeTable` entry (JVMS §4.7.14).
pub struct LocalVariableTypeTableRecord {
    start_pc: u16,
    length: u16,
    name_index: u16,
    signature_index: u16,
    index: u16,
}

bitflags! {
    #[derive(Debug, PartialEq, Clone)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    /// Access and property modifiers from `MethodParameters` (JVMS §4.7.24).
    pub struct MethodParameterFlags: u16 {
        /// Indicates that the formal parameter was declared `final`.
        const ACC_FINAL = 0x0010;
        /// Indicates that the formal parameter was not explicitly or implicitly declared in source code,
        /// according to the specification of the language in which the source code was written (JLS §13.1).
        /// (The formal parameter is an implementation artifact of the compiler which produced this class file.)
        const ACC_SYNTHETIC = 0x1000;
        /// Indicates that the formal parameter was implicitly declared in source code,
        /// according to the specification of the language in which the source code was written (JLS §13.1).
        /// (The formal parameter is mandated by a language specification, so all compilers for the language must emit it.)
        const ACC_MANDATED = 0x8000;
    }
}

#[derive(Debug, PartialEq, Clone, Getters, CopyGetters, new)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `MethodParameters` entry (JVMS §4.7.24).
pub struct MethodParameterRecord {
    #[get_copy = "pub"]
    name_index: u16,
    #[get = "pub"]
    access_flags: MethodParameterFlags,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `stack_map_frame` structure (JVMS §4.7.4).
pub enum StackMapFrame {
    SameFrame {
        frame_type: u8,
        offset_delta: u16,
    },
    SameLocals1StackItemFrame {
        frame_type: u8,
        offset_delta: u16,
        stack: VerificationTypeInfo,
    },
    SameLocals1StackItemFrameExtended {
        frame_type: u8,
        offset_delta: u16,
        stack: VerificationTypeInfo,
    },
    ChopFrame {
        frame_type: u8,
        offset_delta: u16,
    },
    SameFrameExtended {
        frame_type: u8,
        offset_delta: u16,
    },
    AppendFrame {
        frame_type: u8,
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },
    FullFrame {
        frame_type: u8,
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
        stack: Vec<VerificationTypeInfo>,
    },
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `verification_type_info` structure (JVMS §4.7.4).
pub enum VerificationTypeInfo {
    TopVariableInfo,
    IntegerVariableInfo,
    FloatVariableInfo,
    LongVariableInfo,
    DoubleVariableInfo,
    NullVariableInfo,
    UninitializedThisVariableInfo,
    ObjectVariableInfo { cpool_index: u16 },
    UninitializedVariableInfo { offset: u16 },
}

#[derive(Debug, PartialEq, Clone, Getters, CopyGetters, new)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `annotation` structure (JVMS §4.7.16).
pub struct Annotation {
    #[get_copy = "pub"]
    /// Type index in the constant pool
    type_index: u16,
    #[get = "pub"]
    /// Element value pairs
    element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Debug, PartialEq, Clone, Getters, CopyGetters, new)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `element_value_pairs` structure (JVMS §4.7.16).
pub struct ElementValuePair {
    #[get_copy = "pub"]
    /// Element name index in the constant pool
    element_name_index: u16,
    #[get = "pub"]
    /// Element value
    value: ElementValue,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `element_value` structure (JVMS §4.7.16.1).
pub enum ElementValue {
    ConstValueIndex {
        tag: u8,
        const_value_index: u16,
    },
    EnumConstValue {
        tag: u8,
        type_name_index: u16,
        const_name_index: u16,
    },
    ClassInfoIndex {
        tag: u8,
        class_info_index: u16,
    },
    AnnotationValue {
        tag: u8,
        annotation_value: Annotation,
    },
    ArrayValue {
        tag: u8,
        values: Vec<ElementValue>,
    },
}

bitflags! {
    #[derive(Debug, PartialEq, Clone)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    /// Access and property modifiers of `inner_class_access_flags` (JVMS §4.7.6).
    pub struct NestedClassFlags: u16 {
        /// Marked or implicitly public in source.
        const ACC_PUBLIC = 0x0001;
        /// Marked private in source.
        const ACC_PRIVATE = 0x0002;
        /// Marked protected in source.
        const ACC_PROTECTED = 0x0004;
        /// Marked or implicitly static in source.
        const ACC_STATIC = 0x0008;
        /// Marked or implicitly final in source.
        const ACC_FINAL = 0x0010;
        /// Was an interface in source.
        const ACC_INTERFACE = 0x0200;
        /// Marked or implicitly abstract in source.
        const ACC_ABSTRACT = 0x0400;
        /// Declared synthetic; not present in the source code.
        const ACC_SYNTHETIC = 0x1000;
        /// Declared as an annotation interface.
        const ACC_ANNOTATION = 0x2000;
        /// Declared as an enum class.
        const ACC_ENUM = 0x4000;
    }
}

#[derive(Debug, PartialEq, Clone, Getters, CopyGetters, new)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `InnerClasses` entry (JVMS §4.7.6).
pub struct InnerClassRecord {
    #[get_copy = "pub"]
    /// Inner class info index in the constant pool
    inner_class_info_index: u16,
    #[get_copy = "pub"]
    /// Outer class info index in the constant pool
    outer_class_info_index: u16,
    #[get_copy = "pub"]
    /// Inner name index in the constant pool
    inner_name_index: u16,
    #[get = "pub"]
    /// Access and property flags for the inner class
    inner_class_access_flags: NestedClassFlags,
}

#[derive(Debug, PartialEq, Clone, Getters, CopyGetters, new)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `BootstrapMethods` entry (JVMS §4.7.23).
pub struct BootstrapMethodRecord {
    #[get_copy = "pub"]
    /// Bootstrap method reference index in the constant pool
    bootstrap_method_ref: u16,
    #[get = "pub"]
    /// Bootstrap method arguments
    bootstrap_arguments: Vec<u16>,
}

#[derive(Debug, PartialEq, Clone, Getters, CopyGetters, new)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `record_component_info` entry (JVMS §4.7.30).
pub struct RecordComponentInfo {
    #[get_copy = "pub"]
    /// Name index in the constant pool
    name_index: u16,
    #[get_copy = "pub"]
    /// Descriptor index in the constant pool
    descriptor_index: u16,
    #[get = "pub"]
    /// Attributes associated with the record component
    attributes: Vec<Attribute>,
}

#[derive(Debug, PartialEq, Clone, Getters, new)]
#[get = "pub"]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `type_annotation` entry (JVMS §4.7.20).
pub struct TypeAnnotation {
    target_type: TargetType,
    target_info: TargetInfo,
    type_path: Vec<TypePathEntry>,
    annotation: Annotation,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `target_info` entry (JVMS §4.7.20.1).
pub enum TargetInfo {
    TypeParameterTarget {
        type_parameter_index: u8,
    },
    SupertypeTarget {
        supertype_index: u16,
    },
    TypeParameterBoundTarget {
        type_parameter_index: u8,
        bound_index: u8,
    },
    EmptyTarget,
    FormalParameterTarget {
        formal_parameter_index: u8,
    },
    ThrowsTarget {
        throws_type_index: u16,
    },
    LocalvarTarget {
        table: Vec<LocalvarTargetTableEntry>,
    },
    CatchTarget {
        exception_table_index: u16,
    },
    OffsetTarget {
        offset: u16,
    },
    TypeArgumentTarget {
        offset: u16,
        type_argument_index: u8,
    },
}

#[derive(Debug, PartialEq, Clone, CopyGetters, new)]
#[get_copy = "pub"]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `table` entry (JVMS §4.7.20.1).
pub struct LocalvarTargetTableEntry {
    start_pc: u16,
    length: u16,
    index: u16,
}

#[derive(Debug, PartialEq, Clone, CopyGetters, new)]
#[get_copy = "pub"]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `path` entry (JVMS §4.7.20.2).
pub struct TypePathEntry {
    path_kind: u8,
    path_index: u8,
}

pub(crate) fn get_attributes(
    data: &[u8],
    mut start_from: &mut usize,
    constant_pool_vec: &Vec<ConstantPool>,
) -> Result<Vec<Attribute>> {
    let attributes_count: u16 = get_int(&data, &mut start_from)?;
    let mut attributes = Vec::with_capacity(attributes_count as usize);
    for _ in 0..attributes_count {
        attributes.push(get_attribute(&data, &mut start_from, constant_pool_vec)?);
    }

    Ok(attributes)
}

fn get_attribute(
    data: &[u8],
    mut start_from: &mut usize,
    constant_pool_vec: &Vec<ConstantPool>,
) -> Result<Attribute> {
    let attribute_name_index: u16 = get_int(&data, &mut start_from)?;
    let attribute_name = match constant_pool_vec.get(attribute_name_index as usize) {
        Some(item) => match item {
            Utf8 { value } => value,
            _ => {
                return Err(Error::new_io(
                    InvalidData,
                    format!("element type is not Uint8 but {:?}", item).as_str(),
                ));
            }
        },
        None => {
            return Err(Error::new_io(
                InvalidData,
                format!("element not found at index {}", attribute_name_index).as_str(),
            ));
        }
    };

    let attribute_length: u32 = get_int(&data, &mut start_from)?;

    let attribute = match attribute_name.as_str() {
        "ConstantValue" => ConstantValue {
            constantvalue_index: get_int(&data, &mut start_from)?,
        },
        "Code" => {
            let max_stack = get_int(&data, &mut start_from)?;
            let max_locals = get_int(&data, &mut start_from)?;
            let code_length: u32 = get_int(&data, &mut start_from)?;
            let code = get_bytes(&data, &mut start_from, code_length as usize)?.to_vec();
            let exception_table_length: u16 = get_int(&data, &mut start_from)?;

            let mut exception_table = Vec::with_capacity(exception_table_length as usize);
            for _ in 0..exception_table_length {
                exception_table.push(ExceptionRecord::new(
                    get_int(&data, &mut start_from)?,
                    get_int(&data, &mut start_from)?,
                    get_int(&data, &mut start_from)?,
                    get_int(&data, &mut start_from)?,
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
            let number_of_exceptions: u16 = get_int(&data, &mut start_from)?;
            let mut exception_index_table = Vec::with_capacity(number_of_exceptions as usize);
            for _ in 0..number_of_exceptions {
                exception_index_table.push(get_int(&data, &mut start_from)?);
            }
            Exceptions {
                exception_index_table,
            }
        }
        "Synthetic" => Synthetic,
        "Deprecated" => Deprecated,
        "SourceFile" => SourceFile {
            sourcefile_index: get_int(&data, &mut start_from)?,
        },
        "LineNumberTable" => {
            let line_number_table_length: u16 = get_int(&data, &mut start_from)?;
            let mut line_number_table = Vec::with_capacity(line_number_table_length as usize);
            for _ in 0..line_number_table_length {
                line_number_table.push(LineNumberRecord::new(
                    get_int(&data, &mut start_from)?,
                    get_int(&data, &mut start_from)?,
                ));
            }
            LineNumberTable { line_number_table }
        }
        "LocalVariableTable" => {
            let local_variable_table_length: u16 = get_int(&data, &mut start_from)?;
            let mut local_variable_table =
                Vec::with_capacity(local_variable_table_length as usize);
            for _ in 0..local_variable_table_length {
                local_variable_table.push(LocalVariableTableRecord::new(
                    get_int(&data, &mut start_from)?,
                    get_int(&data, &mut start_from)?,
                    get_int(&data, &mut start_from)?,
                    get_int(&data, &mut start_from)?,
                    get_int(&data, &mut start_from)?,
                ))
            }

            LocalVariableTable {
                local_variable_table,
            }
        }
        "InnerClasses" => {
            let number_of_classes: u16 = get_int(&data, &mut start_from)?;
            let mut classes = Vec::with_capacity(number_of_classes as usize);
            for _ in 0..number_of_classes {
                classes.push(InnerClassRecord::new(
                    get_int(&data, &mut start_from)?,
                    get_int(&data, &mut start_from)?,
                    get_int(&data, &mut start_from)?,
                    get_bitfield(&data, &mut start_from)?,
                ));
            }

            InnerClasses { classes }
        }
        "EnclosingMethod" => EnclosingMethod {
            class_index: get_int(&data, &mut start_from)?,
            method_index: get_int(&data, &mut start_from)?,
        },
        "Signature" => Signature {
            signature_index: get_int(&data, &mut start_from)?,
        },
        "LocalVariableTypeTable" => {
            let local_variable_type_table_length: u16 = get_int(&data, &mut start_from)?;
            let mut local_variable_type_table =
                Vec::with_capacity(local_variable_type_table_length as usize);
            for _ in 0..local_variable_type_table_length {
                local_variable_type_table.push(LocalVariableTypeTableRecord::new(
                    get_int(&data, &mut start_from)?,
                    get_int(&data, &mut start_from)?,
                    get_int(&data, &mut start_from)?,
                    get_int(&data, &mut start_from)?,
                    get_int(&data, &mut start_from)?,
                ))
            }

            LocalVariableTypeTable {
                local_variable_type_table,
            }
        }
        "RuntimeVisibleAnnotations" => {
            let raw = read_byte_block(&data, *start_from, attribute_length as usize)?.to_vec();
            let num_annotations: u16 = get_int(&data, &mut start_from)?;
            let mut annotations = Vec::with_capacity(num_annotations as usize);
            for _ in 0..num_annotations {
                annotations.push(get_annotation(&data, &mut start_from)?);
            }

            RuntimeVisibleAnnotations { annotations, raw }
        }
        "RuntimeInvisibleAnnotations" => {
            let num_annotations: u16 = get_int(&data, &mut start_from)?;
            let mut annotations = Vec::with_capacity(num_annotations as usize);
            for _ in 0..num_annotations {
                annotations.push(get_annotation(&data, &mut start_from)?);
            }

            RuntimeInvisibleAnnotations { annotations }
        }
        "RuntimeVisibleParameterAnnotations" => {
            let num_parameters: u8 = get_int(&data, &mut start_from)?;
            let mut parameter_annotations = Vec::with_capacity(num_parameters as usize);
            for _ in 0..num_parameters {
                let num_annotations: u16 = get_int(&data, &mut start_from)?;
                let mut annotations = Vec::with_capacity(num_annotations as usize);
                for _ in 0..num_annotations {
                    annotations.push(get_annotation(&data, &mut start_from)?);
                }
                parameter_annotations.push(annotations);
            }

            RuntimeVisibleParameterAnnotations {
                parameter_annotations,
            }
        }
        "RuntimeVisibleTypeAnnotations" => {
            let num_annotations: u16 = get_int(&data, &mut start_from)?;
            let mut type_annotations = Vec::with_capacity(num_annotations as usize);
            for _ in 0..num_annotations {
                type_annotations.push(get_type_annotation(&data, &mut start_from)?);
            }

            RuntimeVisibleTypeAnnotations { type_annotations }
        }
        "AnnotationDefault" => {
            let raw = read_byte_block(&data, *start_from, attribute_length as usize)?.to_vec();
            AnnotationDefault {
                default_value: get_element_value(&data, &mut start_from)?,
                raw,
            }
        }
        "StackMapTable" => {
            let number_of_entries: u16 = get_int(&data, &mut start_from)?;
            let mut entries = Vec::with_capacity(number_of_entries as usize);
            for _ in 0..number_of_entries {
                let frame_type = get_int(&data, &mut start_from)?;
                let stack_map_frame = match frame_type {
                    0..=63 => SameFrame {
                        frame_type,
                        offset_delta: frame_type as u16,
                    },
                    64..=127 => {
                        let tag: u8 = get_int(&data, &mut start_from)?;

                        SameLocals1StackItemFrame {
                            frame_type,
                            offset_delta: frame_type as u16 - 64,
                            stack: get_verification_type_info(tag, &data, &mut start_from)?,
                        }
                    }
                    247 => {
                        let offset_delta = get_int(&data, &mut start_from)?;
                        let tag = get_int(&data, &mut start_from)?;

                        SameLocals1StackItemFrameExtended {
                            frame_type,
                            offset_delta,
                            stack: get_verification_type_info(tag, &data, &mut start_from)?,
                        }
                    }
                    248..=250 => ChopFrame {
                        frame_type,
                        offset_delta: get_int(&data, &mut start_from)?,
                    },
                    251 => SameFrameExtended {
                        frame_type,
                        offset_delta: get_int(&data, &mut start_from)?,
                    },
                    252..=254 => {
                        let offset_delta = get_int(&data, &mut start_from)?;
                        let size = frame_type - 251;
                        let mut locals = Vec::with_capacity(size as usize);
                        for _ in 0..size {
                            let tag: u8 = get_int(&data, &mut start_from)?;
                            locals.push(get_verification_type_info(tag, &data, &mut start_from)?);
                        }
                        AppendFrame {
                            frame_type,
                            offset_delta,
                            locals,
                        }
                    }
                    255 => {
                        let offset_delta = get_int(&data, &mut start_from)?;

                        let number_of_locals: u16 = get_int(&data, &mut start_from)?;
                        let mut locals = Vec::with_capacity(number_of_locals as usize);
                        for _ in 0..number_of_locals {
                            let tag: u8 = get_int(&data, &mut start_from)?;
                            locals.push(get_verification_type_info(tag, &data, &mut start_from)?);
                        }

                        let number_of_stack_items: u16 = get_int(&data, &mut start_from)?;
                        let mut stack = Vec::with_capacity(number_of_stack_items as usize);
                        for _ in 0..number_of_stack_items {
                            let tag: u8 = get_int(&data, &mut start_from)?;
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
                        return Err(Error::new_io(
                            InvalidData,
                            format!("Unsupported frame_type: {}", frame_type).as_str(),
                        ));
                    }
                };

                entries.push(stack_map_frame);
            }

            StackMapTable { entries }
        }
        "BootstrapMethods" => {
            let num_bootstrap_methods: u16 = get_int(&data, &mut start_from)?;
            let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods as usize);
            for _ in 0..num_bootstrap_methods {
                let bootstrap_method_ref = get_int(&data, &mut start_from)?;
                let num_bootstrap_arguments: u16 = get_int(&data, &mut start_from)?;
                let mut bootstrap_arguments = Vec::with_capacity(num_bootstrap_arguments as usize);
                for _ in 0..num_bootstrap_arguments {
                    bootstrap_arguments.push(get_int(&data, &mut start_from)?);
                }
                bootstrap_methods.push(BootstrapMethodRecord::new(
                    bootstrap_method_ref,
                    bootstrap_arguments,
                ))
            }

            BootstrapMethods { bootstrap_methods }
        }
        "MethodParameters" => {
            let parameters_count: u8 = get_int(&data, &mut start_from)?;
            let mut parameters = Vec::with_capacity(parameters_count as usize);
            for _ in 0..parameters_count {
                parameters.push(MethodParameterRecord::new(
                    get_int(&data, &mut start_from)?,
                    get_bitfield(&data, &mut start_from)?,
                ))
            }
            MethodParameters { parameters }
        }
        "NestHost" => NestHost {
            host_class_index: get_int(&data, &mut start_from)?,
        },
        "NestMembers" => {
            let number_of_classes: u16 = get_int(&data, &mut start_from)?;
            let mut classes = Vec::with_capacity(number_of_classes as usize);
            for _ in 0..number_of_classes {
                classes.push(get_int(&data, &mut start_from)?);
            }

            NestMembers { classes }
        }
        "Record" => {
            let components_count: u16 = get_int(&data, &mut start_from)?;
            let mut components = Vec::with_capacity(components_count as usize);
            for _ in 0..components_count {
                components.push(RecordComponentInfo::new(
                    get_int(&data, &mut start_from)?,
                    get_int(&data, &mut start_from)?,
                    get_attributes(&data, &mut start_from, &constant_pool_vec)?,
                ))
            }

            Record { components }
        }
        "PermittedSubclasses" => {
            let number_of_classes: u16 = get_int(&data, &mut start_from)?;
            let mut classes = Vec::with_capacity(number_of_classes as usize);
            for _ in 0..number_of_classes {
                classes.push(get_int(&data, &mut start_from)?);
            }

            PermittedSubclasses { classes }
        }
        _ => {
            return Err(Error::new_io(
                InvalidData,
                format!("unmatched attribute: {}", attribute_name).as_str(),
            ));
        }
    };

    Ok(attribute)
}

fn get_verification_type_info(
    tag: u8,
    data: &[u8],
    start_from: &mut usize,
) -> Result<VerificationTypeInfo> {
    match tag {
        0 => Ok(VerificationTypeInfo::TopVariableInfo),
        1 => Ok(VerificationTypeInfo::IntegerVariableInfo),
        2 => Ok(VerificationTypeInfo::FloatVariableInfo),
        3 => Ok(VerificationTypeInfo::LongVariableInfo),
        4 => Ok(VerificationTypeInfo::DoubleVariableInfo),
        5 => Ok(VerificationTypeInfo::NullVariableInfo),
        6 => Ok(VerificationTypeInfo::UninitializedThisVariableInfo),
        7 => Ok(VerificationTypeInfo::ObjectVariableInfo {
            cpool_index: get_int(&data, start_from)?,
        }),
        8 => Ok(VerificationTypeInfo::UninitializedVariableInfo {
            offset: get_int(&data, start_from)?,
        }),
        _ => Err(Error::new_io(
            InvalidData,
            format!("tag {} is not valid", tag).as_str(),
        )),
    }
}

fn get_annotation(data: &[u8], start_from: &mut usize) -> Result<Annotation> {
    let type_index = get_int(&data, start_from)?;
    let num_element_value_pairs: u16 = get_int(&data, start_from)?;
    let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
    for _ in 0..num_element_value_pairs {
        let element_name_index: u16 = get_int(&data, start_from)?;
        let value = get_element_value(&data, start_from)?;
        element_value_pairs.push(ElementValuePair::new(element_name_index, value));
    }

    Ok(Annotation::new(type_index, element_value_pairs))
}

fn get_element_value(data: &[u8], start_from: &mut usize) -> Result<ElementValue> {
    let tag: u8 = get_int(&data, start_from)?;
    match tag {
        b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' | b's' => Ok(ConstValueIndex {
            tag,
            const_value_index: get_int(&data, start_from)?,
        }),
        b'e' => Ok(EnumConstValue {
            tag,
            type_name_index: get_int(&data, start_from)?,
            const_name_index: get_int(&data, start_from)?,
        }),
        b'c' => Ok(ClassInfoIndex {
            tag,
            class_info_index: get_int(&data, start_from)?,
        }),
        b'@' => Ok(AnnotationValue {
            tag,
            annotation_value: get_annotation(&data, start_from)?,
        }),
        b'[' => {
            let num_values: u16 = get_int(&data, start_from)?;
            let mut values = Vec::with_capacity(num_values as usize);
            for _ in 0..num_values {
                values.push(get_element_value(&data, start_from)?);
            }

            Ok(ArrayValue { tag, values })
        }
        _ => Err(Error::new_io(
            InvalidData,
            format!("Unsupported element tag: {}", tag).as_str(),
        )),
    }
}

fn get_type_annotation(data: &[u8], start_from: &mut usize) -> Result<TypeAnnotation> {
    let target_type: u8 = get_int(&data, start_from)?;
    let target_type = TargetType::try_from(target_type)?;
    let target_info = get_target_info(data, start_from, target_type)?;
    let type_path = get_type_path(data, start_from)?;
    let annotation = get_annotation(data, start_from)?;
    Ok(TypeAnnotation::new(
        target_type,
        target_info,
        type_path,
        annotation,
    ))
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TargetType {
    // Regular type parameter annotations
    CLASS_TYPE_PARAMETER = 0x00,
    METHOD_TYPE_PARAMETER = 0x01,

    // Type Annotations outside method bodies
    CLASS_EXTENDS = 0x10,
    CLASS_TYPE_PARAMETER_BOUND = 0x11,
    METHOD_TYPE_PARAMETER_BOUND = 0x12,
    FIELD = 0x13,
    METHOD_RETURN = 0x14,
    METHOD_RECEIVER = 0x15,
    METHOD_FORMAL_PARAMETER = 0x16,
    THROWS = 0x17,

    // Type Annotations inside method bodies
    LOCAL_VARIABLE = 0x40,
    RESOURCE_VARIABLE = 0x41,
    EXCEPTION_PARAMETER = 0x42,
    INSTANCEOF = 0x43,
    NEW = 0x44,
    CONSTRUCTOR_REFERENCE = 0x45,
    METHOD_REFERENCE = 0x46,
    CAST = 0x47,
    CONSTRUCTOR_INVOCATION_TYPE_ARGUMENT = 0x48,
    METHOD_INVOCATION_TYPE_ARGUMENT = 0x49,
    CONSTRUCTOR_REFERENCE_TYPE_ARGUMENT = 0x4A,
    METHOD_REFERENCE_TYPE_ARGUMENT = 0x4B,
}

impl TryFrom<u8> for TargetType {
    type Error = Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x00 => Ok(TargetType::CLASS_TYPE_PARAMETER),
            0x01 => Ok(TargetType::METHOD_TYPE_PARAMETER),
            0x10 => Ok(TargetType::CLASS_EXTENDS),
            0x11 => Ok(TargetType::CLASS_TYPE_PARAMETER_BOUND),
            0x12 => Ok(TargetType::METHOD_TYPE_PARAMETER_BOUND),
            0x13 => Ok(TargetType::FIELD),
            0x14 => Ok(TargetType::METHOD_RETURN),
            0x15 => Ok(TargetType::METHOD_RECEIVER),
            0x16 => Ok(TargetType::METHOD_FORMAL_PARAMETER),
            0x17 => Ok(TargetType::THROWS),
            0x40 => Ok(TargetType::LOCAL_VARIABLE),
            0x41 => Ok(TargetType::RESOURCE_VARIABLE),
            0x42 => Ok(TargetType::EXCEPTION_PARAMETER),
            0x43 => Ok(TargetType::INSTANCEOF),
            0x44 => Ok(TargetType::NEW),
            0x45 => Ok(TargetType::CONSTRUCTOR_REFERENCE),
            0x46 => Ok(TargetType::METHOD_REFERENCE),
            0x47 => Ok(TargetType::CAST),
            0x48 => Ok(TargetType::CONSTRUCTOR_INVOCATION_TYPE_ARGUMENT),
            0x49 => Ok(TargetType::METHOD_INVOCATION_TYPE_ARGUMENT),
            0x4A => Ok(TargetType::CONSTRUCTOR_REFERENCE_TYPE_ARGUMENT),
            0x4B => Ok(TargetType::METHOD_REFERENCE_TYPE_ARGUMENT),
            _ => Err(Error::new_io(
                InvalidData,
                &format!("Invalid target_type: {}", value),
            )),
        }
    }
}

fn get_target_info(
    data: &[u8],
    start_from: &mut usize,
    target_type: TargetType,
) -> Result<TargetInfo> {
    match target_type {
        TargetType::CLASS_TYPE_PARAMETER | TargetType::METHOD_TYPE_PARAMETER => {
            Ok(TargetInfo::TypeParameterTarget {
                type_parameter_index: get_int(&data, start_from)?,
            })
        }
        TargetType::CLASS_EXTENDS => Ok(TargetInfo::SupertypeTarget {
            supertype_index: get_int(&data, start_from)?,
        }),
        TargetType::CLASS_TYPE_PARAMETER_BOUND | TargetType::METHOD_TYPE_PARAMETER_BOUND => {
            Ok(TargetInfo::TypeParameterBoundTarget {
                type_parameter_index: get_int(&data, start_from)?,
                bound_index: get_int(&data, start_from)?,
            })
        }
        TargetType::FIELD | TargetType::METHOD_RETURN | TargetType::METHOD_RECEIVER => {
            Ok(TargetInfo::EmptyTarget)
        }
        TargetType::METHOD_FORMAL_PARAMETER => Ok(TargetInfo::FormalParameterTarget {
            formal_parameter_index: get_int(&data, start_from)?,
        }),
        TargetType::THROWS => Ok(TargetInfo::ThrowsTarget {
            throws_type_index: get_int(&data, start_from)?,
        }),
        TargetType::LOCAL_VARIABLE | TargetType::RESOURCE_VARIABLE => {
            let table_length: u16 = get_int(&data, start_from)?;
            let mut table = Vec::with_capacity(table_length as usize);
            for _ in 0..table_length {
                table.push(LocalvarTargetTableEntry::new(
                    get_int(&data, start_from)?,
                    get_int(&data, start_from)?,
                    get_int(&data, start_from)?,
                ));
            }

            Ok(TargetInfo::LocalvarTarget { table })
        }
        TargetType::EXCEPTION_PARAMETER => Ok(TargetInfo::CatchTarget {
            exception_table_index: get_int(&data, start_from)?,
        }),
        TargetType::INSTANCEOF
        | TargetType::NEW
        | TargetType::CONSTRUCTOR_REFERENCE
        | TargetType::METHOD_REFERENCE => Ok(TargetInfo::OffsetTarget {
            offset: get_int(&data, start_from)?,
        }),
        TargetType::CAST
        | TargetType::CONSTRUCTOR_INVOCATION_TYPE_ARGUMENT
        | TargetType::METHOD_INVOCATION_TYPE_ARGUMENT
        | TargetType::CONSTRUCTOR_REFERENCE_TYPE_ARGUMENT
        | TargetType::METHOD_REFERENCE_TYPE_ARGUMENT => Ok(TargetInfo::TypeArgumentTarget {
            offset: get_int(&data, start_from)?,
            type_argument_index: get_int(&data, start_from)?,
        }),
    }
}

fn get_type_path(data: &[u8], start_from: &mut usize) -> Result<Vec<TypePathEntry>> {
    let path_length: u8 = get_int(&data, start_from)?;
    let mut path = Vec::with_capacity(path_length as usize);
    for _ in 0..path_length {
        path.push(TypePathEntry::new(
            get_int(&data, start_from)?,
            get_int(&data, start_from)?,
        ));
    }

    Ok(path)
}
