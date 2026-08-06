use crate::constant_pool::{ConstantPool, ConstantPoolLookup};
use crate::error::{Error, Result};
use crate::exception_table::{CatchType, ExceptionTableRecord};
use derive_new::new;
use getset::{CopyGetters, Getters};
use jclassfile::attributes::{
    Attribute, InnerClassRecord, LineNumberRecord, MethodParameterRecord,
};
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Debug)]
pub struct Attributes {
    data: HashMap<AttributeType, Attribute>,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub(crate) enum AttributeType {
    ConstantValue,
    Code,
    Exceptions,
    SourceFile,
    LineNumberTable,
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

/// A resolved `LocalVariableTable` entry: the slot's live bytecode range plus its source name.
/// Built from the class file's `LocalVariableTable` with the `name_index` already resolved against
/// the constant pool.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Getters, CopyGetters)]
pub struct LocalVariableInfo {
    #[get_copy = "pub"]
    pub start_pc: u16,
    #[get_copy = "pub"]
    pub length: u16,
    #[get_copy = "pub"]
    pub slot: u16,
    #[get = "pub"]
    pub name: String,
}

/// A resolved `Code` attribute: bytecode plus the tables that describe it, with constant-pool
/// indexes already resolved and line numbers collapsed into a lookup keyed by bytecode index.
#[derive(Debug, PartialEq)]
pub(crate) struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub bytecode: Vec<u8>,
    pub line_numbers: BTreeMap<u16, u16>,
    pub exception_table: Vec<ExceptionTableRecord>,
    pub local_variable_table: Vec<LocalVariableInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, new, Getters, CopyGetters)]
pub struct BootstrapMethodInfo {
    #[get_copy = "pub"]
    pub ref_kind: u8,
    #[get = "pub"]
    pub class_name: String,
    #[get = "pub"]
    pub method_name: String,
    #[get = "pub"]
    pub method_descriptor: String,
    #[get = "pub"]
    pub bootstrap_arguments_cpool_indexes: Vec<u16>,
    #[get = "pub"]
    pub invoke_dynamic_method_name: String,
    #[get = "pub"]
    pub invoke_dynamic_method_descriptor: String,
}

impl From<&Attribute> for AttributeType {
    fn from(value: &Attribute) -> Self {
        match value {
            Attribute::ConstantValue { .. } => AttributeType::ConstantValue,
            Attribute::Code { .. } => AttributeType::Code,
            Attribute::Exceptions { .. } => AttributeType::Exceptions,
            Attribute::SourceFile { .. } => AttributeType::SourceFile,
            Attribute::LineNumberTable { .. } => AttributeType::LineNumberTable,
            Attribute::LocalVariableTable { .. } => AttributeType::LocalVariableTable,
            Attribute::InnerClasses { .. } => AttributeType::InnerClasses,
            Attribute::Synthetic => AttributeType::Synthetic,
            Attribute::Deprecated => AttributeType::Deprecated,
            Attribute::EnclosingMethod { .. } => AttributeType::EnclosingMethod,
            Attribute::Signature { .. } => AttributeType::Signature,
            Attribute::SourceDebugExtension { .. } => AttributeType::SourceDebugExtension,
            Attribute::LocalVariableTypeTable { .. } => AttributeType::LocalVariableTypeTable,
            Attribute::RuntimeVisibleAnnotations { .. } => {
                AttributeType::RuntimeVisibleAnnotations
            }
            Attribute::RuntimeInvisibleAnnotations { .. } => {
                AttributeType::RuntimeInvisibleAnnotations
            }
            Attribute::RuntimeVisibleParameterAnnotations { .. } => {
                AttributeType::RuntimeVisibleParameterAnnotations
            }
            Attribute::RuntimeInvisibleParameterAnnotations { .. } => {
                AttributeType::RuntimeInvisibleParameterAnnotations
            }
            Attribute::AnnotationDefault { .. } => AttributeType::AnnotationDefault,
            Attribute::StackMapTable { .. } => AttributeType::StackMapTable,
            Attribute::BootstrapMethods { .. } => AttributeType::BootstrapMethods,
            Attribute::RuntimeVisibleTypeAnnotations { .. } => {
                AttributeType::RuntimeVisibleTypeAnnotations
            }
            Attribute::RuntimeInvisibleTypeAnnotations { .. } => {
                AttributeType::RuntimeInvisibleTypeAnnotations
            }
            Attribute::MethodParameters { .. } => AttributeType::MethodParameters,
            Attribute::Module => AttributeType::Module,
            Attribute::ModulePackages => AttributeType::ModulePackages,
            Attribute::ModuleMainClass => AttributeType::ModuleMainClass,
            Attribute::NestHost { .. } => AttributeType::NestHost,
            Attribute::NestMembers { .. } => AttributeType::NestMembers,
            Attribute::Record { .. } => AttributeType::Record,
            Attribute::PermittedSubclasses { .. } => AttributeType::PermittedSubclasses,
        }
    }
}

impl Attributes {
    pub(crate) fn new(attributes: &[Attribute]) -> Self {
        Self {
            data: attributes
                .iter()
                .map(|attribute| (attribute.into(), attribute.clone()))
                .collect(),
        }
    }

    /// No attributes, for classes the VM invents rather than loads (primitives, arrays).
    pub fn empty() -> Self {
        Self::new(&[])
    }

    /// `Ok(None)` when the method carries no `Code` attribute; an error when it carries one this
    /// constant pool cannot resolve.
    pub(crate) fn get_code<T: ConstantPoolLookup>(
        &self,
        constant_pool: &T,
    ) -> Result<Option<CodeAttribute>> {
        let Some(Attribute::Code {
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        }) = self.data.get(&AttributeType::Code)
        else {
            return Ok(None);
        };

        let nested_attributes = Attributes::new(attributes);
        let local_variable_table = nested_attributes.get_local_variable_table(constant_pool);

        let line_numbers = nested_attributes
            .get_line_number_table()
            .iter()
            .map(|record| (record.start_pc(), record.line_number()))
            .collect();

        let exception_table = exception_table
            .iter()
            .map(|rec| {
                let catch_type = rec.catch_type();

                // JVMS §4.7.3: a zero catch_type is the catch-all that a `finally` block compiles
                // to, so only a non-zero index has to resolve.
                let catch_type = if catch_type == 0 {
                    CatchType::Any
                } else {
                    CatchType::Class(constant_pool.get_class_name(catch_type).ok_or_else(
                        || {
                            Error::constant_pool(format!(
                                "Error getting exception handler catch type by index={catch_type}"
                            ))
                        },
                    )?)
                };

                Ok(ExceptionTableRecord::new(
                    rec.start_pc(),
                    rec.end_pc(),
                    rec.handler_pc(),
                    catch_type,
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Some(CodeAttribute {
            max_stack: *max_stack,
            max_locals: *max_locals,
            bytecode: code.clone(),
            line_numbers,
            exception_table,
            local_variable_table,
        }))
    }

    fn get_line_number_table(&self) -> Vec<LineNumberRecord> {
        match self.data.get(&AttributeType::LineNumberTable) {
            Some(Attribute::LineNumberTable { line_number_table }) => line_number_table.clone(),
            _ => vec![],
        }
    }

    /// Resolves the `LocalVariableTable` (present only when compiled with `javac -g`) into records
    /// carrying the variable's source name, used to render helpful `NullPointerException` messages
    /// (JEP 358). Returns an empty vector when the attribute is absent.
    fn get_local_variable_table<T: ConstantPoolLookup>(
        &self,
        constant_pool: &T,
    ) -> Vec<LocalVariableInfo> {
        match self.data.get(&AttributeType::LocalVariableTable) {
            Some(Attribute::LocalVariableTable {
                local_variable_table,
            }) => local_variable_table
                .iter()
                .filter_map(|record| {
                    let name = constant_pool.get_utf8(record.name_index())?;
                    Some(LocalVariableInfo {
                        start_pc: record.start_pc(),
                        length: record.length(),
                        slot: record.index(),
                        name,
                    })
                })
                .collect(),
            _ => vec![],
        }
    }

    pub(crate) fn get_inner_class_records(&self) -> Option<Vec<InnerClassRecord>> {
        match self.data.get(&AttributeType::InnerClasses)? {
            Attribute::InnerClasses { classes } => Some(classes.clone()),
            _ => None,
        }
    }

    pub(crate) fn get_method_parameter_records(&self) -> Option<Vec<MethodParameterRecord>> {
        match self.data.get(&AttributeType::MethodParameters)? {
            Attribute::MethodParameters { parameters } => Some(parameters.clone()),
            _ => None,
        }
    }

    pub fn get_enclosing_method(&self) -> Option<(u16, u16)> {
        match self.data.get(&AttributeType::EnclosingMethod)? {
            Attribute::EnclosingMethod {
                class_index,
                method_index,
            } => Some((*class_index, *method_index)),
            _ => None,
        }
    }

    pub fn get_source_file<T: ConstantPoolLookup>(&self, constant_pool: &T) -> Option<String> {
        match self.data.get(&AttributeType::SourceFile)? {
            Attribute::SourceFile { sourcefile_index } => {
                constant_pool.get_utf8(*sourcefile_index)
            }
            _ => None,
        }
    }

    pub fn get_bootstrap_method<T: ConstantPoolLookup>(
        &self,
        constant_pool: &T,
        cpool_index: u16,
    ) -> Option<BootstrapMethodInfo> {
        let (
            bootstrap_methods_index,
            invoke_dynamic_method_name,
            invoke_dynamic_method_descriptor,
        ) = constant_pool.get_invoke_dynamic(cpool_index)?;
        let bootstrap_record = match self.data.get(&AttributeType::BootstrapMethods)? {
            // Indexed, not panicking: the index comes from an `InvokeDynamic` constant-pool entry
            // in an untrusted class file and nothing guarantees it is within the table.
            Attribute::BootstrapMethods { bootstrap_methods } => {
                bootstrap_methods.get(bootstrap_methods_index as usize)
            }
            _ => None,
        }?;

        let bootstrap_method_ref = bootstrap_record.bootstrap_method_ref();
        let (ref_kind, class_name, method_name, method_descriptor) =
            constant_pool.get_method_handle(bootstrap_method_ref)?;

        let bootstrap_arguments_cpool_indexes = bootstrap_record.bootstrap_arguments();
        Some(BootstrapMethodInfo::new(
            ref_kind,
            class_name,
            method_name,
            method_descriptor,
            bootstrap_arguments_cpool_indexes.clone(),
            invoke_dynamic_method_name,
            invoke_dynamic_method_descriptor,
        ))
    }

    pub fn get_exception_indexes(&self) -> Option<Vec<u16>> {
        match self.data.get(&AttributeType::Exceptions)? {
            Attribute::Exceptions {
                exception_index_table,
            } => Some(exception_index_table.clone()),
            _ => None,
        }
    }

    pub fn get_annotation_default_raw(&self) -> Option<Vec<u8>> {
        match self.data.get(&AttributeType::AnnotationDefault)? {
            Attribute::AnnotationDefault {
                default_value: _,
                raw,
            } => Some(raw.to_vec()),
            _ => None,
        }
    }

    pub fn get_annotations(
        &self,
        constant_pool: &ConstantPool,
    ) -> Option<(HashSet<String>, Vec<u8>)> {
        match self.data.get(&AttributeType::RuntimeVisibleAnnotations)? {
            Attribute::RuntimeVisibleAnnotations { annotations, raw } => {
                let annotations_name = annotations
                    .iter()
                    .flat_map(|annotation| constant_pool.get_utf8(annotation.type_index()))
                    .collect();

                Some((annotations_name, raw.to_vec()))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constant_pool::MockConstantPoolLookup;
    use crate::error::ErrorKind;
    use jclassfile::attributes::Attribute::{
        Code, LineNumberTable, LocalVariableTable, MethodParameters,
    };
    use jclassfile::attributes::{
        BootstrapMethodRecord, ExceptionRecord, LineNumberRecord, LocalVariableTableRecord,
        MethodParameterFlags, MethodParameterRecord,
    };
    use std::collections::HashMap;

    #[test]
    fn should_create_attribute_map() {
        let code = Code {
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
                    ],
                },
            ],
        };
        let method_parameters = MethodParameters {
            parameters: vec![MethodParameterRecord::new(
                11,
                MethodParameterFlags::empty(),
            )],
        };
        let attributes = vec![code.clone(), method_parameters.clone()];
        let actual = Attributes::new(&attributes);

        let mut expected = HashMap::new();
        expected.insert(AttributeType::Code, code);
        expected.insert(AttributeType::MethodParameters, method_parameters);

        assert_eq!(expected, actual.data);
    }

    #[test]
    fn should_return_code_attribute() {
        let code = Code {
            max_stack: 2,
            max_locals: 4,
            code: vec![0x2a, 0xb7, 0x0, 0x1],
            exception_table: vec![ExceptionRecord::new(1, 2, 3, 4)],
            attributes: vec![LineNumberTable {
                line_number_table: vec![LineNumberRecord::new(0, 4)],
            }],
        };
        let method_parameters = MethodParameters {
            parameters: vec![MethodParameterRecord::new(
                11,
                MethodParameterFlags::empty(),
            )],
        };
        let attributes = vec![code.clone(), method_parameters.clone()];

        let nested_attributes = Attributes::new(&attributes);

        let mut mock = MockConstantPoolLookup::new();

        mock.expect_get_class_name()
            .withf(|index| *index == 4)
            .return_const(Some("java/lang/Exception".to_string()));

        assert_eq!(
            Some(CodeAttribute {
                max_stack: 2,
                max_locals: 4,
                bytecode: vec![0x2a, 0xb7, 0x0, 0x1],
                line_numbers: BTreeMap::from([(0, 4)]),
                exception_table: vec![ExceptionTableRecord::new(
                    1,
                    2,
                    3,
                    CatchType::Class("java/lang/Exception".to_string())
                )],
                local_variable_table: vec![],
            }),
            nested_attributes
                .get_code(&mock)
                .expect("the code attribute should resolve")
        );
    }

    /// A class file is untrusted input, so an exception handler naming a catch type the constant
    /// pool cannot resolve has to be an error rather than a panic.
    #[test]
    fn should_fail_on_an_unresolvable_catch_type() {
        let attributes = vec![Code {
            max_stack: 2,
            max_locals: 4,
            code: vec![0x2a, 0xb7, 0x0, 0x1],
            exception_table: vec![ExceptionRecord::new(1, 2, 3, 99)],
            attributes: vec![],
        }];

        let mut mock = MockConstantPoolLookup::new();
        mock.expect_get_class_name().return_const(None);

        let actual = Attributes::new(&attributes).get_code(&mock);

        assert_eq!(
            ErrorKind::ConstantPool,
            actual
                .expect_err("an absent catch type should not resolve")
                .kind()
        );
    }

    /// A zero `catch_type` is the catch-all a `finally` block compiles to, and must not be looked
    /// up in the constant pool at all.
    #[test]
    fn should_resolve_a_catch_all_handler_without_a_lookup() {
        let attributes = vec![Code {
            max_stack: 2,
            max_locals: 4,
            code: vec![0x2a, 0xb7, 0x0, 0x1],
            exception_table: vec![ExceptionRecord::new(1, 2, 3, 0)],
            attributes: vec![],
        }];

        let mut mock = MockConstantPoolLookup::new();
        mock.expect_get_class_name().never();

        let actual = Attributes::new(&attributes)
            .get_code(&mock)
            .expect("the code attribute should resolve")
            .expect("a code attribute is present");

        assert_eq!(
            vec![ExceptionTableRecord::new(1, 2, 3, CatchType::Any)],
            actual.exception_table
        );
    }

    #[test]
    fn should_return_source_file() {
        let source_file_attribute = Attribute::SourceFile {
            sourcefile_index: 42,
        };
        let attributes = vec![source_file_attribute.clone()];
        let nested_attributes = Attributes::new(&attributes);

        let mut mock = MockConstantPoolLookup::new();
        mock.expect_get_utf8()
            .withf(|index| *index == 42)
            .return_const(Some("TestSourceFile.java".to_string()));

        assert_eq!(
            Some("TestSourceFile.java".to_string()),
            nested_attributes.get_source_file(&mock)
        );
    }

    #[test]
    fn should_return_bootstrap_method() {
        let bootstrapmethod_ref = 42u16;
        let bootstrap_arguments = vec![1000, 1001];
        let bootstrapmethods_attribute = Attribute::BootstrapMethods {
            bootstrap_methods: vec![
                BootstrapMethodRecord::new(1, vec![2, 3]),
                BootstrapMethodRecord::new(bootstrapmethod_ref, bootstrap_arguments.clone()),
            ],
        };
        let attributes = vec![bootstrapmethods_attribute.clone()];
        let nested_attributes = Attributes::new(&attributes);

        let invoke_dynamic_cpool_index = 1337u16;
        let bootstrap_methods_index = 1;
        let invoke_dynamic_method_name = "bootstrapMethod".to_string();
        let invoke_dynamic_method_descriptor = "([Ljava/lang/Object;)V".to_string();
        let mut mock = MockConstantPoolLookup::new();
        mock.expect_get_invoke_dynamic()
            .withf(move |index| *index == invoke_dynamic_cpool_index)
            .return_const(Some((
                bootstrap_methods_index,
                invoke_dynamic_method_name.clone(),
                invoke_dynamic_method_descriptor.clone(),
            )));

        let reference_kind = 6;
        let class_name = "class_name".to_string();
        let name = "method_name".to_string();
        let descriptor = "()V".to_string();
        mock.expect_get_method_handle()
            .withf(move |index| *index == bootstrapmethod_ref)
            .return_const(Some((
                reference_kind,
                class_name.clone(),
                name.clone(),
                descriptor.clone(),
            )));

        let actual = nested_attributes.get_bootstrap_method(&mock, invoke_dynamic_cpool_index);
        let expected = Some(BootstrapMethodInfo::new(
            reference_kind,
            class_name,
            name,
            descriptor,
            bootstrap_arguments,
            invoke_dynamic_method_name,
            invoke_dynamic_method_descriptor,
        ));
        assert_eq!(expected, actual);
    }

    /// The bootstrap method index comes from an `InvokeDynamic` entry in an untrusted class file,
    /// so one pointing past the end of the `BootstrapMethods` table must not index out of bounds.
    #[test]
    fn should_return_none_for_an_out_of_range_bootstrap_method_index() {
        let attributes = vec![Attribute::BootstrapMethods {
            bootstrap_methods: vec![BootstrapMethodRecord::new(1, vec![2, 3])],
        }];
        let nested_attributes = Attributes::new(&attributes);

        let mut mock = MockConstantPoolLookup::new();
        mock.expect_get_invoke_dynamic().return_const(Some((
            99,
            "bootstrapMethod".to_string(),
            "([Ljava/lang/Object;)V".to_string(),
        )));
        mock.expect_get_method_handle().never();

        assert_eq!(None, nested_attributes.get_bootstrap_method(&mock, 1337));
    }
}
