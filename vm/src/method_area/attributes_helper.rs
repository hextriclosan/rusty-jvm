use crate::method_area::cpool_helper::CPoolHelper;
use jclass::attributes::{Attribute, InnerClassRecord, LineNumberRecord};
use std::collections::{HashMap, HashSet};

pub struct AttributesHelper {
    data: HashMap<AttributeType, Attribute>,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum AttributeType {
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
            Attribute::SourceDebugExtension => AttributeType::SourceDebugExtension,
            Attribute::LocalVariableTypeTable { .. } => AttributeType::LocalVariableTypeTable,
            Attribute::RuntimeVisibleAnnotations { .. } => {
                AttributeType::RuntimeVisibleAnnotations
            }
            Attribute::RuntimeInvisibleAnnotations { .. } => {
                AttributeType::RuntimeInvisibleAnnotations
            }
            Attribute::RuntimeVisibleParameterAnnotations => {
                AttributeType::RuntimeVisibleParameterAnnotations
            }
            Attribute::RuntimeInvisibleParameterAnnotations => {
                AttributeType::RuntimeInvisibleParameterAnnotations
            }
            Attribute::AnnotationDefault { .. } => AttributeType::AnnotationDefault,
            Attribute::StackMapTable { .. } => AttributeType::StackMapTable,
            Attribute::BootstrapMethods { .. } => AttributeType::BootstrapMethods,
            Attribute::RuntimeVisibleTypeAnnotations => {
                AttributeType::RuntimeVisibleTypeAnnotations
            }
            Attribute::RuntimeInvisibleTypeAnnotations => {
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

impl AttributesHelper {
    pub fn new(attributes: &[Attribute]) -> Self {
        Self {
            data: attributes
                .iter()
                .map(|attribute| (attribute.into(), attribute.clone()))
                .collect(),
        }
    }

    pub fn get_code(&self) -> Option<(u16, u16, Vec<u8>, Vec<LineNumberRecord>)> {
        match self.data.get(&AttributeType::Code)? {
            Attribute::Code {
                max_stack,
                max_locals,
                code,
                exception_table: _,
                attributes,
            } => {
                let attributes_helper = AttributesHelper::new(attributes);
                let line_numbers = attributes_helper.get_line_number_table();

                Some((*max_stack, *max_locals, code.clone(), line_numbers))
            }
            _ => None,
        }
    }

    fn get_line_number_table(&self) -> Vec<LineNumberRecord> {
        match self.data.get(&AttributeType::LineNumberTable) {
            Some(Attribute::LineNumberTable { line_number_table }) => line_number_table.clone(),
            _ => vec![],
        }
    }

    pub fn get_inner_class_records(&self) -> Option<Vec<InnerClassRecord>> {
        match self.data.get(&AttributeType::InnerClasses)? {
            Attribute::InnerClasses { classes } => Some(classes.clone()),
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
        cpool_helper: &CPoolHelper,
    ) -> Option<(HashSet<String>, Vec<u8>)> {
        match self.data.get(&AttributeType::RuntimeVisibleAnnotations)? {
            Attribute::RuntimeVisibleAnnotations { annotations, raw } => {
                let annotations_name = annotations
                    .iter()
                    .flat_map(|annotation| {
                        let type_index = annotation.type_index();
                        let type_name = cpool_helper.get_utf8(type_index);
                        type_name
                    })
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
    use jclass::attributes::Attribute::{
        Code, LineNumberTable, LocalVariableTable, MethodParameters,
    };
    use jclass::attributes::{
        LineNumberRecord, LocalVariableTableRecord, MethodParameterFlags, MethodParameterRecord,
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
        let actual = AttributesHelper::new(&attributes);

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
            exception_table: vec![],
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

        let attributes_helper = AttributesHelper::new(&attributes);

        assert_eq!(
            Some((
                2,
                4,
                vec![0x2a, 0xb7, 0x0, 0x1],
                vec![LineNumberRecord::new(0, 4)]
            )),
            attributes_helper.get_code()
        );
    }
}
