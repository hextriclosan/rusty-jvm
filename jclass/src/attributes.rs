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
use crate::extractors::{get_bitfield, get_bytes, get_int};
use bitflags::bitflags;
use std::io::ErrorKind::InvalidData;

#[derive(Debug, PartialEq)]
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
    },
    RuntimeInvisibleAnnotations {
        annotations: Vec<Annotation>,
    },
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault {
        default_value: ElementValue,
    },
    StackMapTable {
        entries: Vec<StackMapFrame>,
    },
    BootstrapMethods {
        bootstrap_methods: Vec<BootstrapMethodRecord>,
    },
    RuntimeVisibleTypeAnnotations,
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

#[derive(Debug, PartialEq)]
pub struct ExceptionRecord {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl ExceptionRecord {
    pub fn new(start_pc: u16, end_pc: u16, handler_pc: u16, catch_type: u16) -> Self {
        Self {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        }
    }
    pub fn start_pc(&self) -> u16 {
        self.start_pc
    }
    pub fn end_pc(&self) -> u16 {
        self.end_pc
    }
    pub fn handler_pc(&self) -> u16 {
        self.handler_pc
    }
    pub fn catch_type(&self) -> u16 {
        self.catch_type
    }
}

#[derive(Debug, PartialEq)]
pub struct LineNumberRecord {
    start_pc: u16,
    line_number: u16,
}

impl LineNumberRecord {
    pub fn new(start_pc: u16, line_number: u16) -> Self {
        Self {
            start_pc,
            line_number,
        }
    }
    pub fn start_pc(&self) -> u16 {
        self.start_pc
    }
    pub fn line_number(&self) -> u16 {
        self.line_number
    }
}

#[derive(Debug, PartialEq)]
pub struct LocalVariableTableRecord {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

impl LocalVariableTableRecord {
    pub fn new(
        start_pc: u16,
        length: u16,
        name_index: u16,
        descriptor_index: u16,
        index: u16,
    ) -> Self {
        Self {
            start_pc,
            length,
            name_index,
            descriptor_index,
            index,
        }
    }
    pub fn start_pc(&self) -> u16 {
        self.start_pc
    }
    pub fn length(&self) -> u16 {
        self.length
    }
    pub fn name_index(&self) -> u16 {
        self.name_index
    }
    pub fn descriptor_index(&self) -> u16 {
        self.descriptor_index
    }
    pub fn index(&self) -> u16 {
        self.index
    }
}

#[derive(Debug, PartialEq)]
pub struct LocalVariableTypeTableRecord {
    start_pc: u16,
    length: u16,
    name_index: u16,
    signature_index: u16,
    index: u16,
}

impl LocalVariableTypeTableRecord {
    pub fn new(
        start_pc: u16,
        length: u16,
        name_index: u16,
        signature_index: u16,
        index: u16,
    ) -> Self {
        Self {
            start_pc,
            length,
            name_index,
            signature_index,
            index,
        }
    }
    pub fn start_pc(&self) -> u16 {
        self.start_pc
    }
    pub fn length(&self) -> u16 {
        self.length
    }
    pub fn name_index(&self) -> u16 {
        self.name_index
    }
    pub fn signature_index(&self) -> u16 {
        self.signature_index
    }
    pub fn index(&self) -> u16 {
        self.index
    }
}

bitflags! {
    #[derive(Debug, PartialEq)]
    pub struct MethodParameterFlags: u16 {
        const ACC_FINAL = 0x0010;       // Indicates that the formal parameter was declared final.
        const ACC_SYNTHETIC = 0x1000;   // Indicates that the formal parameter was not explicitly or implicitly declared in source code, according to the specification of the language in which the source code was written (JLS §13.1). (The formal parameter is an implementation artifact of the compiler which produced this class file.)
        const ACC_MANDATED = 0x8000;    // Indicates that the formal parameter was implicitly declared in source code, according to the specification of the language in which the source code was written (JLS §13.1). (The formal parameter is mandated by a language specification, so all compilers for the language must emit it.)
    }
}

#[derive(Debug, PartialEq)]
pub struct MethodParameterRecord {
    name_index: u16,
    access_flags: MethodParameterFlags,
}

impl MethodParameterRecord {
    pub fn new(name_index: u16, access_flags: MethodParameterFlags) -> Self {
        Self {
            name_index,
            access_flags,
        }
    }
    pub fn name_index(&self) -> u16 {
        self.name_index
    }
    pub fn access_flags(&self) -> &MethodParameterFlags {
        &self.access_flags
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Annotation {
    type_index: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

impl Annotation {
    pub fn new(type_index: u16, element_value_pairs: Vec<ElementValuePair>) -> Self {
        Self {
            type_index,
            element_value_pairs,
        }
    }
    pub fn type_index(&self) -> u16 {
        self.type_index
    }
    pub fn element_value_pairs(&self) -> &Vec<ElementValuePair> {
        &self.element_value_pairs
    }
}

#[derive(Debug, PartialEq)]
pub struct ElementValuePair {
    element_name_index: u16,
    value: ElementValue,
}

impl ElementValuePair {
    pub fn new(element_name_index: u16, value: ElementValue) -> Self {
        Self {
            element_name_index,
            value,
        }
    }
    pub fn element_name_index(&self) -> u16 {
        self.element_name_index
    }
    pub fn value(&self) -> &ElementValue {
        &self.value
    }
}

#[derive(Debug, PartialEq)]
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
    #[derive(Debug, PartialEq)]
    pub struct NestedClassFlags: u16 {
        const ACC_PUBLIC = 0x0001;      // Marked or implicitly public in source.
        const ACC_PRIVATE = 0x0002;     // Marked private in source.
        const ACC_PROTECTED = 0x0004;   // Marked protected in source.
        const ACC_STATIC = 0x0008;      // Marked or implicitly static in source.
        const ACC_FINAL = 0x0010;       // Marked or implicitly final in source.
        const ACC_INTERFACE = 0x0200;   // Was an interface in source.
        const ACC_ABSTRACT = 0x0400;    // Marked or implicitly abstract in source.
        const ACC_SYNTHETIC = 0x1000;   // Declared synthetic; not present in the source code.
        const ACC_ANNOTATION = 0x2000;  // Declared as an annotation interface.
        const ACC_ENUM = 0x4000;        // Declared as an enum class.
    }
}

#[derive(Debug, PartialEq)]
pub struct InnerClassRecord {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: NestedClassFlags,
}

impl InnerClassRecord {
    pub fn new(
        inner_class_info_index: u16,
        outer_class_info_index: u16,
        inner_name_index: u16,
        inner_class_access_flags: NestedClassFlags,
    ) -> Self {
        Self {
            inner_class_info_index,
            outer_class_info_index,
            inner_name_index,
            inner_class_access_flags,
        }
    }
    pub fn inner_class_info_index(&self) -> u16 {
        self.inner_class_info_index
    }
    pub fn outer_class_info_index(&self) -> u16 {
        self.outer_class_info_index
    }
    pub fn inner_name_index(&self) -> u16 {
        self.inner_name_index
    }
    pub fn inner_class_access_flags(&self) -> &NestedClassFlags {
        &self.inner_class_access_flags
    }
}

#[derive(Debug, PartialEq)]
pub struct BootstrapMethodRecord {
    bootstrap_method_ref: u16,
    bootstrap_arguments: Vec<u16>,
}

impl BootstrapMethodRecord {
    pub fn new(bootstrap_method_ref: u16, bootstrap_arguments: Vec<u16>) -> Self {
        Self {
            bootstrap_method_ref,
            bootstrap_arguments,
        }
    }
    pub fn bootstrap_method_ref(&self) -> u16 {
        self.bootstrap_method_ref
    }
    pub fn bootstrap_arguments(&self) -> &Vec<u16> {
        &self.bootstrap_arguments
    }
}

#[derive(Debug, PartialEq)]
pub struct RecordComponentInfo {
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<Attribute>,
}

impl RecordComponentInfo {
    pub fn new(name_index: u16, descriptor_index: u16, attributes: Vec<Attribute>) -> Self {
        Self {
            name_index,
            descriptor_index,
            attributes,
        }
    }
    pub fn name_index(&self) -> u16 {
        self.name_index
    }
    pub fn descriptor_index(&self) -> u16 {
        self.descriptor_index
    }
    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }
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

    let _attribute_length: u32 = get_int(&data, &mut start_from)?;

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
            let num_annotations: u16 = get_int(&data, &mut start_from)?;
            let mut annotations = Vec::with_capacity(num_annotations as usize);
            for _ in 0..num_annotations {
                annotations.push(get_annotation(&data, &mut start_from)?);
            }

            RuntimeVisibleAnnotations { annotations }
        }
        "RuntimeInvisibleAnnotations" => {
            let num_annotations: u16 = get_int(&data, &mut start_from)?;
            let mut annotations = Vec::with_capacity(num_annotations as usize);
            for _ in 0..num_annotations {
                annotations.push(get_annotation(&data, &mut start_from)?);
            }

            RuntimeInvisibleAnnotations { annotations }
        }
        "AnnotationDefault" => AnnotationDefault {
            default_value: get_element_value(&data, &mut start_from)?,
        },
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
