#[derive(Debug, PartialEq)]
pub struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<ConstantPool>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
    attributes: Vec<Attribute>,
}

impl ClassFile {
    pub fn new(
        magic: u32,
        minor_version: u16,
        major_version: u16,
        constant_pool: Vec<ConstantPool>,
        access_flags: u16,
        this_class: u16,
        super_class: u16,
        interfaces: Vec<u16>,
        fields: Vec<FieldInfo>,
        methods: Vec<MethodInfo>,
        attributes: Vec<Attribute>) -> Self {
        Self {
            magic,
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<Attribute>,
}

impl FieldInfo {
    pub fn new(
        access_flags: u16,
        name_index: u16,
        descriptor_index: u16,
        attributes: Vec<Attribute>) -> Self {
        Self { access_flags, name_index, descriptor_index, attributes }
    }
}

#[derive(Debug, PartialEq)]
pub struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<Attribute>,
}

impl MethodInfo {
    pub fn new(
        access_flags: u16,
        name_index: u16,
        descriptor_index: u16,
        attributes: Vec<Attribute>) -> Self {
        Self { access_flags, name_index, descriptor_index, attributes }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum ConstantPool {
    Empty = 0,
    Uint8 {
        value: String,
    } = 1,
    Integer {
        value: i32,
    } = 3,
    Float {
        value: f32,
    } = 4,
    Long {
        value: i64,
    } = 5,
    Double {
        value: f64,
    } = 6,
    Class {
        name_index: u16,
    } = 7,
    String {
        string_index: u16,
    } = 8,
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    } = 9,
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    } = 10,
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    } = 11,
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    } = 12,
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    } = 15,
    MethodType {
        descriptor_index: u16,
    } = 16,
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    } = 17,
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    } = 18,
    Module {
        name_index: u16
    } = 19,
    Package {
        name_index: u16
    } = 20,
}

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
    LocalVariableTable,
    InnerClasses,
    Synthetic,
    Deprecated,
    EnclosingMethod,
    Signature,
    SourceDebugExtension,
    LocalVariableTypeTable,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault,
    StackMapTable,
    BootstrapMethods,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    MethodParameters,
    Module,
    ModulePackages,
    ModuleMainClass,
    NestHost,
    NestMembers,
    Record,
    PermittedSubclasses,
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
        Self { start_pc, end_pc, handler_pc, catch_type }
    }
}

#[derive(Debug, PartialEq)]
pub struct LineNumberRecord {
    start_pc: u16,
    line_number: u16,
}

impl LineNumberRecord {
    pub fn new(start_pc: u16, line_number: u16) -> Self {
        Self { start_pc, line_number }
    }
}
