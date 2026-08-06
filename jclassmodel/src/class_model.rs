use crate::attributes::{Attributes, LocalVariableInfo};
use crate::constant_pool::{ConstantPool, ConstantPoolLookup};
use crate::error::{Error, ErrorKind, Result};
use crate::exception_table::ExceptionTableRecord;
use crate::modifiers::{
    ClassModifier, FieldModifier, MethodModifier, NestedClassModifier, ParameterModifier,
};
use jclassfile::attributes::InnerClassRecord;
use jclassfile::class_file::ClassFile;
use jclassfile::fields::FieldInfo;
use jclassfile::methods::{MethodFlags, MethodInfo};
use jdescriptor::{MethodDescriptor, TypeDescriptor};
use std::collections::{BTreeMap, HashSet};
use std::fmt::{Display, Formatter};

/// Marks a native method whose descriptor is not its real signature, so callers must dispatch it by
/// name rather than by name and descriptor (`MethodHandle::invoke` and friends).
const POLYMORPHIC_SIGNATURE: &str = "Ljava/lang/invoke/MethodHandle$PolymorphicSignature;";

/// A class file that has been parsed but not yet interpreted.
///
/// This intermediate step exists because the name a class is *loaded* under is not always the name
/// recorded in its own constant pool, and the caller is the one that knows the difference. Read the
/// recorded name with [`ParsedClass::this_class_name`], decide on the final names, then call
/// [`ParsedClass::into_model`].
#[derive(Debug)]
pub struct ParsedClass {
    class_file: ClassFile,
}

/// Parses a class file into a [`ParsedClass`].
pub fn parse(bytecode: &[u8]) -> Result<ParsedClass> {
    let class_file = jclassfile::class_file::parse(bytecode)
        .map_err(|e| Error::with_source(ErrorKind::ClassFile, "error parsing class file", e))?;

    Ok(ParsedClass { class_file })
}

impl ParsedClass {
    /// The class name as recorded in this class file's own constant pool.
    pub fn this_class_name(&self) -> Option<String> {
        ConstantPool::new(self.class_file.constant_pool())
            .get_class_name(self.class_file.this_class())
    }

    /// The class file format version, readable before committing to [`ParsedClass::into_model`]
    /// so an unsupported version can be rejected without resolving the whole class.
    pub fn version(&self) -> ClassFileVersion {
        ClassFileVersion {
            major: self.class_file.major_version(),
            minor: self.class_file.minor_version(),
        }
    }

    /// Resolves every constant-pool reference, naming the class the way its own constant pool
    /// does.
    ///
    /// This is what an ordinary consumer wants: for a class read from a `.class` file the
    /// recorded name *is* the loaded name, and the external name is it with `/` replaced by `.`.
    ///
    /// Reach for [`ParsedClass::into_model`] instead when the class is being loaded under a name
    /// the file does not record, which happens when a JVM defines a hidden class
    /// (`Lookup.defineHiddenClass`, and so every lambda) and appends a suffix to the recorded
    /// name. Only the caller knows that suffix, and the dotted form of such a name is not a plain
    /// `/`-to-`.` replacement.
    pub fn into_model_with_recorded_name(self) -> Result<ClassModel> {
        let internal_name = self.this_class_name().ok_or_else(|| {
            Error::constant_pool("Error getting the class name recorded in the constant pool")
        })?;
        let external_name = internal_name.replace('/', ".");

        self.into_model(internal_name, external_name)
    }

    /// Resolves every constant-pool reference and builds the runtime-friendly view, under names
    /// the caller chooses.
    ///
    /// `internal_name` and `external_name` are the names the class is being loaded under, and
    /// they win over whatever the constant pool records for this class. Read the recorded name
    /// first with [`ParsedClass::this_class_name`] if you need to base them on it.
    ///
    /// Most consumers want [`ParsedClass::into_model_with_recorded_name`] instead; this overload
    /// is for callers that mint their own names, such as a JVM naming a hidden class or one it
    /// invents outright for a primitive or an array.
    pub fn into_model(self, internal_name: String, external_name: String) -> Result<ClassModel> {
        let version = self.version();
        let class_file = self.class_file;
        let constant_pool = ConstantPool::new_with_classname(
            class_file.constant_pool(),
            class_file.this_class(),
            internal_name.clone(),
        );

        let super_class_index = class_file.super_class();
        let super_class_name = if super_class_index > 0 {
            Some(
                constant_pool
                    .get_class_name(super_class_index)
                    .ok_or_else(|| {
                        Error::constant_pool(format!(
                            "Error getting super_class_name by index={super_class_index}"
                        ))
                    })?,
            )
        } else {
            None
        };

        let interfaces = class_file
            .interfaces()
            .iter()
            .map(|index| {
                constant_pool.get_class_name(*index).ok_or_else(|| {
                    Error::constant_pool(format!("Error getting interface by index={index}"))
                })
            })
            .collect::<Result<Vec<String>>>()?;

        let methods = class_file
            .methods()
            .iter()
            .map(|method_info| MethodModel::new(method_info, &constant_pool))
            .collect::<Result<Vec<_>>>()?;

        let fields = class_file
            .fields()
            .iter()
            .map(|field_info| FieldModel::new(field_info, &constant_pool))
            .collect::<Result<Vec<_>>>()?;

        let attributes = Attributes::new(class_file.attributes());
        let nested_class_record = nested_class_record(&attributes, &constant_pool, &internal_name);
        let declaring_class = nested_class_record
            .as_ref()
            .and_then(|record| constant_pool.get_class_name(record.outer_class_info_index()));
        let nested_class_modifiers = nested_class_record.as_ref().map(|record| {
            NestedClassModifier::from_bits_truncate(record.inner_class_access_flags().bits())
        });
        let annotations_raw = attributes
            .get_annotations(&constant_pool)
            .map(|(_annotations, annotations_raw)| annotations_raw);
        let enclosing_method = enclosing_method(&attributes, &constant_pool);
        let source_file = attributes.get_source_file(&constant_pool);

        Ok(ClassModel {
            version,
            name: internal_name,
            external_name,
            super_class_name,
            interfaces,
            modifiers: ClassModifier::from_bits_truncate(class_file.access_flags().bits()),
            methods,
            fields,
            declaring_class,
            nested_class_modifiers,
            annotations_raw,
            enclosing_method,
            source_file,
            constant_pool,
            attributes,
        })
    }
}

/// A class file with every constant-pool reference resolved.
#[derive(Debug)]
pub struct ClassModel {
    /// The class file format version this class was compiled to.
    pub version: ClassFileVersion,
    /// Internal (slash-separated) name the class is loaded under.
    pub name: String,
    /// External (dot-separated) name the class is loaded under.
    pub external_name: String,
    /// Superclass name, absent only for `java/lang/Object`.
    pub super_class_name: Option<String>,
    /// Directly implemented interfaces, in declaration order.
    pub interfaces: Vec<String>,
    pub modifiers: ClassModifier,
    /// Methods in declaration order. How to key them is left to the caller.
    pub methods: Vec<MethodModel>,
    /// Fields in declaration order.
    pub fields: Vec<FieldModel>,
    /// Name of the class this one is declared in, if it is an inner class.
    pub declaring_class: Option<String>,
    /// Modifiers from this class's own `InnerClasses` record, present only when it is nested.
    ///
    /// These are what `Class.getModifiers()` reports for a nested class, and they carry
    /// `private`, `protected` and `static`, which [`ClassModifier`] structurally cannot. Present
    /// even for anonymous and local classes, where [`ClassModel::declaring_class`] is `None`
    /// because the record names no outer class.
    pub nested_class_modifiers: Option<NestedClassModifier>,
    /// Raw `RuntimeVisibleAnnotations` bytes, as reflection hands them back to Java unchanged.
    pub annotations_raw: Option<Vec<u8>>,
    /// What lexically encloses this class, for local and anonymous classes.
    pub enclosing_method: Option<EnclosingMethodInfo>,
    pub source_file: Option<String>,
    pub constant_pool: ConstantPool,
    pub attributes: Attributes,
}

/// The class file format version (JVMS §4.1, `major_version` and `minor_version`).
///
/// Ordering is lexicographic on `major` then `minor`, which is the ordering the spec uses to
/// decide whether a JVM supports a class file, so the derive depends on the field order below.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassFileVersion {
    pub major: u16,
    pub minor: u16,
}

impl ClassFileVersion {
    /// Whether this class file depends on preview features. JVMS §4.1 marks those with a
    /// `minor_version` of `65535`, and they are valid only on the exact JDK release that emitted
    /// them.
    pub fn is_preview(&self) -> bool {
        self.minor == 0xFFFF
    }
}

impl Display for ClassFileVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// A resolved `EnclosingMethod` attribute (JVMS §4.7.7): the class, and where applicable the
/// method, that lexically encloses a local or anonymous class.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnclosingMethodInfo {
    /// The immediately enclosing class.
    pub class_name: String,
    /// The immediately enclosing method.
    ///
    /// Absent when the class appears in an instance initializer, a field initializer or a static
    /// block rather than in a method: JVMS §4.7.7 permits `method_index` to be zero there, and the
    /// attribute still names the enclosing class.
    pub method: Option<EnclosingMethodRef>,
}

/// The name and descriptor of an enclosing method.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnclosingMethodRef {
    pub name: String,
    pub descriptor: String,
}

/// A method with its name, descriptor and code resolved.
#[derive(Debug)]
pub struct MethodModel {
    pub name: String,
    /// `name:descriptor`, e.g. `valueOf:(I)Ljava/lang/Integer;`.
    pub name_signature: String,
    pub descriptor: MethodDescriptor,
    /// Absent for abstract and native methods, which carry no `Code` attribute.
    pub code: Option<CodeModel>,
    pub modifiers: MethodModifier,
    /// Checked exceptions declared by the `throws` clause (the `Exceptions` attribute), in
    /// declaration order. Empty for methods that declare none.
    pub exceptions: Vec<String>,
    /// Formal parameters from the `MethodParameters` attribute, in declaration order. Empty
    /// unless the class was compiled with `javac -parameters`; the descriptor remains the
    /// authority on how many parameters a method actually takes.
    pub parameters: Vec<ParameterInfo>,
    pub annotation_default_raw: Option<Vec<u8>>,
    pub annotations_raw: Option<Vec<u8>>,
    pub runtime_visible_annotations: HashSet<String>,
}

/// A formal parameter from the `MethodParameters` attribute (JVMS §4.7.24).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParameterInfo {
    /// The parameter's name. Absent when `name_index` is zero, which JVMS §4.7.24 uses to record
    /// a parameter whose name the compiler did not emit.
    pub name: Option<String>,
    pub modifiers: ParameterModifier,
}

/// The `Code` attribute with its exception handlers and line numbers resolved.
#[derive(Debug)]
pub struct CodeModel {
    pub max_stack: u16,
    pub max_locals: u16,
    pub bytecode: Vec<u8>,
    /// Maps a bytecode index to the source line it came from.
    pub line_numbers: BTreeMap<u16, u16>,
    pub exception_table: Vec<ExceptionTableRecord>,
    /// Empty unless the class was compiled with `javac -g`.
    pub local_variable_table: Vec<LocalVariableInfo>,
}

/// A field with its descriptor parsed.
#[derive(Debug)]
pub struct FieldModel {
    pub name: String,
    pub descriptor: TypeDescriptor,
    pub modifiers: FieldModifier,
}

impl MethodModel {
    fn new(method_info: &MethodInfo, constant_pool: &ConstantPool) -> Result<Self> {
        let name_index = method_info.name_index();
        let name = constant_pool.get_utf8(name_index).ok_or_else(|| {
            Error::constant_pool(format!("error getting method name by index {name_index}"))
        })?;

        let descriptor_index = method_info.descriptor_index();
        let signature = constant_pool.get_utf8(descriptor_index).ok_or_else(|| {
            Error::constant_pool(format!(
                "error getting method signature by index {descriptor_index}"
            ))
        })?;

        let name_signature = format!("{name}:{signature}");
        let descriptor: MethodDescriptor = signature.parse().map_err(|err| {
            Error::new(
                ErrorKind::Descriptor,
                format!("Error parsing signature {signature}: {err}"),
            )
        })?;

        let attributes = Attributes::new(method_info.attributes());
        let access_flags = method_info.access_flags();

        let code = if access_flags.intersects(MethodFlags::ACC_ABSTRACT | MethodFlags::ACC_NATIVE)
        {
            None
        } else {
            Some(CodeModel::new(&attributes, constant_pool)?.ok_or_else(|| {
                Error::structure(format!(
                    "Error getting code attribute for method {name_signature}"
                ))
            })?)
        };

        let (runtime_visible_annotations, annotations_raw) =
            match attributes.get_annotations(constant_pool) {
                Some((annotations, annotations_raw)) => (annotations, Some(annotations_raw)),
                None => (HashSet::new(), None),
            };

        let exceptions = attributes
            .get_exception_indexes()
            .unwrap_or_default()
            .iter()
            .map(|index| {
                constant_pool.get_class_name(*index).ok_or_else(|| {
                    Error::constant_pool(format!(
                        "Error getting exception of method {name_signature} by index={index}"
                    ))
                })
            })
            .collect::<Result<Vec<String>>>()?;

        let parameters = attributes
            .get_method_parameter_records()
            .unwrap_or_default()
            .iter()
            .map(|record| {
                let name_index = record.name_index();

                // JVMS §4.7.24: a zero name_index records a parameter with no name, so only a
                // non-zero index that fails to resolve is an error.
                let name = if name_index > 0 {
                    Some(constant_pool.get_utf8(name_index).ok_or_else(|| {
                        Error::constant_pool(format!(
                            "Error getting parameter name of method {name_signature} by index={name_index}"
                        ))
                    })?)
                } else {
                    None
                };

                Ok(ParameterInfo {
                    name,
                    modifiers: ParameterModifier::from_bits_truncate(record.access_flags().bits()),
                })
            })
            .collect::<Result<Vec<ParameterInfo>>>()?;

        Ok(Self {
            name,
            name_signature,
            descriptor,
            code,
            modifiers: MethodModifier::from_bits_truncate(access_flags.bits()),
            exceptions,
            parameters,
            annotation_default_raw: attributes.get_annotation_default_raw(),
            annotations_raw,
            runtime_visible_annotations,
        })
    }

    /// Whether this method's descriptor is not its real signature, so callers must dispatch it by
    /// name alone (`MethodHandle::invoke` and friends).
    pub fn is_polymorphic_signature(&self) -> bool {
        self.is_native()
            && self
                .runtime_visible_annotations
                .contains(POLYMORPHIC_SIGNATURE)
    }

    pub fn is_native(&self) -> bool {
        self.modifiers.contains(MethodModifier::Native)
    }
}

impl CodeModel {
    fn new(attributes: &Attributes, constant_pool: &ConstantPool) -> Result<Option<Self>> {
        let Some(code) = attributes.get_code(constant_pool)? else {
            return Ok(None);
        };

        Ok(Some(Self {
            max_stack: code.max_stack,
            max_locals: code.max_locals,
            bytecode: code.bytecode,
            line_numbers: code.line_numbers,
            exception_table: code.exception_table,
            local_variable_table: code.local_variable_table,
        }))
    }
}

impl FieldModel {
    fn new(field_info: &FieldInfo, constant_pool: &ConstantPool) -> Result<Self> {
        let name_index = field_info.name_index();
        let name = constant_pool.get_utf8(name_index).ok_or_else(|| {
            Error::constant_pool(format!("Error getting field name by index {name_index}"))
        })?;

        let descriptor_index = field_info.descriptor_index();
        let raw_descriptor = constant_pool.get_utf8(descriptor_index).ok_or_else(|| {
            Error::constant_pool(format!(
                "Error getting field descriptor by index {descriptor_index}"
            ))
        })?;
        let descriptor: TypeDescriptor = raw_descriptor.parse().map_err(|err| {
            Error::new(
                ErrorKind::Descriptor,
                format!("Error parsing field descriptor {raw_descriptor}: {err}"),
            )
        })?;

        Ok(Self {
            name,
            descriptor,
            modifiers: FieldModifier::from_bits_truncate(field_info.access_flags().bits()),
        })
    }

    pub fn is_static(&self) -> bool {
        self.modifiers.contains(FieldModifier::Static)
    }
}

/// The `InnerClasses` record describing *this* class, if it is nested.
///
/// A class lists every nested class it knows about, including itself when it is nested, so the
/// record for this class has to be picked out by name.
fn nested_class_record(
    attributes: &Attributes,
    constant_pool: &ConstantPool,
    class_name: &str,
) -> Option<InnerClassRecord> {
    let inner_class_records = attributes.get_inner_class_records()?;

    inner_class_records.into_iter().find(|inner_class_record| {
        let inner_class_info_index = inner_class_record.inner_class_info_index();
        constant_pool
            .get_class_name(inner_class_info_index)
            .is_some_and(|inner_class_info| class_name == inner_class_info)
    })
}

fn enclosing_method(
    attributes: &Attributes,
    constant_pool: &ConstantPool,
) -> Option<EnclosingMethodInfo> {
    let (class_index, method_index) = attributes.get_enclosing_method()?;

    let class_name = constant_pool.get_class_name(class_index)?;

    // JVMS §4.7.7: a zero method_index means the class is enclosed by a class but not by any
    // method, so the enclosing class stands on its own.
    let method = if method_index > 0 {
        let (name, descriptor) = constant_pool.get_name_and_type(method_index)?;
        Some(EnclosingMethodRef { name, descriptor })
    } else {
        None
    };

    Some(EnclosingMethodInfo { class_name, method })
}

#[cfg(test)]
mod tests {
    use super::*;
    use jclassfile::attributes::{
        Attribute, MethodParameterFlags, MethodParameterRecord, NestedClassFlags,
    };
    use jclassfile::constant_pool::ConstantPool as Entry;
    use jclassfile::constant_pool::ConstantPool::{Class, Empty, NameAndType, Utf8};

    #[test]
    fn class_file_version_should_recognise_preview() {
        let java25 = ClassFileVersion {
            major: 69,
            minor: 0,
        };
        let java25_preview = ClassFileVersion {
            major: 69,
            minor: 0xFFFF,
        };

        assert!(!java25.is_preview());
        assert!(java25_preview.is_preview());
    }

    /// Ordering is what a JVM uses to reject a class file it is too old to run, so it has to
    /// compare `major` before `minor`.
    #[test]
    fn class_file_version_should_order_by_major_then_minor() {
        let java17 = ClassFileVersion {
            major: 61,
            minor: 0,
        };
        let java21 = ClassFileVersion {
            major: 65,
            minor: 0,
        };
        let java17_preview = ClassFileVersion {
            major: 61,
            minor: 0xFFFF,
        };

        assert!(java17 < java21);
        assert!(java17 < java17_preview);
        assert!(java17_preview < java21);
        assert_eq!("61.0", java17.to_string());
    }

    /// Constant pool shared by the enclosing-method tests:
    /// 1 = `Class` -> 2 = `"Trivial"`, 3 = `NameAndType` -> 4 = `"run"`, 5 = `"()V"`.
    fn cpool() -> Vec<Entry> {
        vec![
            Empty,
            Class { name_index: 2 },
            Utf8 {
                value: "Trivial".into(),
            },
            NameAndType {
                name_index: 4,
                descriptor_index: 5,
            },
            Utf8 {
                value: "run".into(),
            },
            Utf8 {
                value: "()V".into(),
            },
        ]
    }

    #[test]
    fn should_resolve_enclosing_class_and_method() {
        let attributes = Attributes::new(&[Attribute::EnclosingMethod {
            class_index: 1,
            method_index: 3,
        }]);

        let actual = enclosing_method(&attributes, &ConstantPool::new(&cpool()));

        assert_eq!(
            Some(EnclosingMethodInfo {
                class_name: "Trivial".to_string(),
                method: Some(EnclosingMethodRef {
                    name: "run".to_string(),
                    descriptor: "()V".to_string(),
                }),
            }),
            actual
        );
    }

    /// JVMS §4.7.7 allows `method_index` to be zero when the class is enclosed by a class but not
    /// by a method (an instance or field initializer). The enclosing class must survive that.
    #[test]
    fn should_resolve_enclosing_class_when_method_index_is_zero() {
        let attributes = Attributes::new(&[Attribute::EnclosingMethod {
            class_index: 1,
            method_index: 0,
        }]);

        let actual = enclosing_method(&attributes, &ConstantPool::new(&cpool()));

        assert_eq!(
            Some(EnclosingMethodInfo {
                class_name: "Trivial".to_string(),
                method: None,
            }),
            actual
        );
    }

    /// Constant pool for the `throws`-clause tests: 1 = `"run"`, 2 = `"()V"`, then two exception
    /// classes at 3 and 5, deliberately ordered so declaration order differs from index order.
    fn throwing_method_cpool() -> Vec<Entry> {
        vec![
            Empty,
            Utf8 {
                value: "run".into(),
            },
            Utf8 {
                value: "()V".into(),
            },
            Class { name_index: 4 },
            Utf8 {
                value: "java/io/IOException".into(),
            },
            Class { name_index: 6 },
            Utf8 {
                value: "java/lang/InterruptedException".into(),
            },
        ]
    }

    fn abstract_method(attributes: Vec<Attribute>) -> MethodInfo {
        MethodInfo::new(MethodFlags::ACC_ABSTRACT, 1, 2, attributes)
    }

    /// Constant pool for the `MethodParameters` tests: 1 = `"run"`, 2 = `"()V"`, 3 = `"count"`.
    fn parameterized_method_cpool() -> Vec<Entry> {
        vec![
            Empty,
            Utf8 {
                value: "run".into(),
            },
            Utf8 {
                value: "()V".into(),
            },
            Utf8 {
                value: "count".into(),
            },
        ]
    }

    /// JVMS §4.7.24 lets `name_index` be zero for a parameter the compiler emitted no name for,
    /// so an unnamed parameter must still appear in the list, carrying its modifiers.
    #[test]
    fn should_resolve_named_and_unnamed_method_parameters() {
        let method_info = abstract_method(vec![Attribute::MethodParameters {
            parameters: vec![
                MethodParameterRecord::new(3, MethodParameterFlags::ACC_FINAL),
                MethodParameterRecord::new(0, MethodParameterFlags::ACC_MANDATED),
            ],
        }]);

        let actual = MethodModel::new(
            &method_info,
            &ConstantPool::new(&parameterized_method_cpool()),
        )
        .expect("method should resolve");

        assert_eq!(
            vec![
                ParameterInfo {
                    name: Some("count".to_string()),
                    modifiers: ParameterModifier::Final,
                },
                ParameterInfo {
                    name: None,
                    modifiers: ParameterModifier::Mandated,
                },
            ],
            actual.parameters
        );
    }

    #[test]
    fn should_resolve_no_parameters_without_the_attribute() {
        let method_info = abstract_method(vec![]);

        let actual = MethodModel::new(
            &method_info,
            &ConstantPool::new(&parameterized_method_cpool()),
        )
        .expect("method should resolve");

        assert!(actual.parameters.is_empty());
    }

    #[test]
    fn should_fail_on_an_unresolvable_parameter_name_index() {
        let method_info = abstract_method(vec![Attribute::MethodParameters {
            parameters: vec![MethodParameterRecord::new(
                99,
                MethodParameterFlags::empty(),
            )],
        }]);

        let actual = MethodModel::new(
            &method_info,
            &ConstantPool::new(&parameterized_method_cpool()),
        );

        assert_eq!(
            ErrorKind::ConstantPool,
            actual
                .expect_err("an absent index should not resolve")
                .kind()
        );
    }

    /// A class lists every nested class it knows about, so the record for *this* class has to be
    /// picked out by name rather than by position.
    #[test]
    fn should_find_this_classes_own_nested_class_record() {
        let attributes = Attributes::new(&[Attribute::InnerClasses {
            classes: vec![
                InnerClassRecord::new(3, 0, 0, NestedClassFlags::ACC_PUBLIC),
                InnerClassRecord::new(1, 3, 0, NestedClassFlags::ACC_PRIVATE),
            ],
        }]);

        let actual = nested_class_record(&attributes, &ConstantPool::new(&cpool()), "Trivial")
            .expect("the record for this class should be found");

        assert_eq!(
            NestedClassModifier::Private,
            NestedClassModifier::from_bits_truncate(actual.inner_class_access_flags().bits())
        );
    }

    #[test]
    fn should_resolve_thrown_exceptions_in_declaration_order() {
        let method_info = abstract_method(vec![Attribute::Exceptions {
            exception_index_table: vec![5, 3],
        }]);

        let actual = MethodModel::new(&method_info, &ConstantPool::new(&throwing_method_cpool()))
            .expect("method should resolve");

        assert_eq!(
            vec![
                "java/lang/InterruptedException".to_string(),
                "java/io/IOException".to_string(),
            ],
            actual.exceptions
        );
    }

    #[test]
    fn should_resolve_no_exceptions_without_the_attribute() {
        let method_info = abstract_method(vec![]);

        let actual = MethodModel::new(&method_info, &ConstantPool::new(&throwing_method_cpool()))
            .expect("method should resolve");

        assert!(actual.exceptions.is_empty());
    }

    #[test]
    fn should_fail_on_an_unresolvable_exception_index() {
        let method_info = abstract_method(vec![Attribute::Exceptions {
            exception_index_table: vec![99],
        }]);

        let actual = MethodModel::new(&method_info, &ConstantPool::new(&throwing_method_cpool()));

        assert_eq!(
            ErrorKind::ConstantPool,
            actual
                .expect_err("an absent index should not resolve")
                .kind()
        );
    }

    #[test]
    fn should_return_none_without_enclosing_method_attribute() {
        let attributes = Attributes::new(&[]);

        let actual = enclosing_method(&attributes, &ConstantPool::new(&cpool()));

        assert_eq!(None, actual);
    }
}
