//! Format checking (JVMS §4.8), as far as it concerns the constant pool: the constraints
//! documented throughout JVMS §4.4, and the well-formedness of the names, classes and descriptors
//! that field and method references carry.
//!
//! The other checks of JVMS §4.8 (magic number, attribute lengths, truncation and trailing bytes)
//! concern the layout of the file and are made while it is parsed.

use crate::attributes::Attribute;
use crate::class_file::{ClassFile, ClassFlags};
use crate::constant_pool::ConstantPool;
use crate::constant_pool::ConstantPool::*;
use crate::error::{Error, Result};
use crate::names::{
    is_binary_name, is_class_info_name, is_field_descriptor, is_method_descriptor, is_method_name,
    is_module_name, is_unqualified_name,
};
use std::fmt::{Arguments, Display};

pub(crate) fn check_format(class_file: &ClassFile) -> Result<()> {
    let constant_pool = class_file.constant_pool();

    if constant_pool.is_empty() {
        return Err(Error::format("constant_pool_count is zero (JVMS §4.1)"));
    }
    if let Some(Long { .. } | Double { .. }) = constant_pool.last() {
        return Err(Error::format(format!(
            "constant pool entry #{}: an 8-byte constant takes up two entries, but this is the last one (JVMS §4.4.5)",
            constant_pool.len() - 1
        )));
    }

    let checker = FormatChecker {
        constant_pool,
        major_version: class_file.major_version(),
        is_module: class_file.access_flags().contains(ClassFlags::ACC_MODULE),
        bootstrap_methods_count: bootstrap_methods_count(class_file.attributes()),
    };

    // Index zero is not a valid constant pool index.
    (1..constant_pool.len()).try_for_each(|index| checker.check_entry(index))
}

/// The length of the `bootstrap_methods` array of the class file (JVMS §4.7.23).
fn bootstrap_methods_count(attributes: &[Attribute]) -> usize {
    attributes
        .iter()
        .find_map(|attribute| match attribute {
            Attribute::BootstrapMethods { bootstrap_methods } => Some(bootstrap_methods.len()),
            _ => None,
        })
        .unwrap_or(0)
}

/// The name of a constant kind, the JVMS section that documents it, and the major version of the
/// class file format that first defined it (JVMS §4.4, Table 4.4-B).
fn describe(entry: &ConstantPool) -> (&'static str, &'static str, u16) {
    match entry {
        Empty => ("unusable", "4.4.5", 45),
        Utf8 { .. } => ("Utf8", "4.4.7", 45),
        Integer { .. } => ("Integer", "4.4.4", 45),
        Float { .. } => ("Float", "4.4.4", 45),
        Long { .. } => ("Long", "4.4.5", 45),
        Double { .. } => ("Double", "4.4.5", 45),
        Class { .. } => ("Class", "4.4.1", 45),
        String { .. } => ("String", "4.4.3", 45),
        Fieldref { .. } => ("Fieldref", "4.4.2", 45),
        Methodref { .. } => ("Methodref", "4.4.2", 45),
        InterfaceMethodref { .. } => ("InterfaceMethodref", "4.4.2", 45),
        NameAndType { .. } => ("NameAndType", "4.4.6", 45),
        MethodHandle { .. } => ("MethodHandle", "4.4.8", 51),
        MethodType { .. } => ("MethodType", "4.4.9", 51),
        Dynamic { .. } => ("Dynamic", "4.4.10", 55),
        InvokeDynamic { .. } => ("InvokeDynamic", "4.4.10", 51),
        Module { .. } => ("Module", "4.4.11", 53),
        Package { .. } => ("Package", "4.4.12", 53),
    }
}

struct FormatChecker<'a> {
    constant_pool: &'a [ConstantPool],
    major_version: u16,
    is_module: bool,
    bootstrap_methods_count: usize,
}

impl<'a> FormatChecker<'a> {
    fn check_entry(&self, index: usize) -> Result<()> {
        let entry = self.entry(index);

        if self.major_version < entry.since {
            return Err(Error::format(format!(
                "constant pool entry #{index} ({}): the tag was first defined in version {}.0 of the class file format, but the class file has major version {} (JVMS §4.4)",
                entry.kind, entry.since, self.major_version
            )));
        }

        match self.constant_pool[index] {
            Empty | Utf8 { .. } | Integer { .. } | Float { .. } | Long { .. } | Double { .. } => {
                Ok(())
            }
            Class { name_index } => {
                let name = entry.utf8("name_index", name_index)?;
                entry.ensure(
                    is_class_info_name(name),
                    format_args!("{name:?} is not a valid class or interface name"),
                )
            }
            String { string_index } => entry.utf8("string_index", string_index).map(|_| ()),
            Fieldref {
                class_index,
                name_and_type_index,
            } => {
                entry.class("class_index", class_index)?;
                let (_, descriptor) = entry.name_and_type(name_and_type_index)?;
                entry.ensure(
                    !descriptor.starts_with('('),
                    format_args!("{descriptor:?} is not a field descriptor"),
                )
            }
            Methodref {
                class_index,
                name_and_type_index,
            } => {
                entry.class("class_index", class_index)?;
                let (name, descriptor) = entry.name_and_type(name_and_type_index)?;
                entry.ensure(
                    descriptor.starts_with('('),
                    format_args!("{descriptor:?} is not a method descriptor"),
                )?;
                if name.starts_with('<') {
                    entry.ensure(
                        name == "<init>",
                        format_args!("the name {name:?} begins with '<', but is not <init>"),
                    )?;
                    entry.ensure(
                        descriptor.ends_with(")V"),
                        format_args!(
                            "<init> must return void, but its descriptor is {descriptor:?}"
                        ),
                    )?;
                }
                Ok(())
            }
            InterfaceMethodref {
                class_index,
                name_and_type_index,
            } => {
                entry.class("class_index", class_index)?;
                let (_, descriptor) = entry.name_and_type(name_and_type_index)?;
                entry.ensure(
                    descriptor.starts_with('('),
                    format_args!("{descriptor:?} is not a method descriptor"),
                )
            }
            NameAndType {
                name_index,
                descriptor_index,
            } => {
                let name = entry.utf8("name_index", name_index)?;
                let descriptor = entry.utf8("descriptor_index", descriptor_index)?;
                if descriptor.starts_with('(') {
                    entry.ensure(
                        is_method_descriptor(descriptor),
                        format_args!("{descriptor:?} is not a valid method descriptor"),
                    )?;
                    entry.ensure(
                        is_method_name(name),
                        format_args!("{name:?} is not a valid method name"),
                    )
                } else {
                    entry.ensure(
                        is_field_descriptor(descriptor),
                        format_args!("{descriptor:?} is not a valid field descriptor"),
                    )?;
                    entry.ensure(
                        is_unqualified_name(name),
                        format_args!("{name:?} is not a valid field name"),
                    )
                }
            }
            MethodHandle {
                reference_kind,
                reference_index,
            } => self.check_method_handle(&entry, reference_kind, reference_index),
            MethodType { descriptor_index } => {
                let descriptor = entry.utf8("descriptor_index", descriptor_index)?;
                entry.ensure(
                    is_method_descriptor(descriptor),
                    format_args!("{descriptor:?} is not a valid method descriptor"),
                )
            }
            Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                self.check_bootstrap_method(&entry, bootstrap_method_attr_index)?;
                let (_, descriptor) = entry.name_and_type(name_and_type_index)?;
                entry.ensure(
                    !descriptor.starts_with('('),
                    format_args!("{descriptor:?} is not a field descriptor"),
                )
            }
            InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                self.check_bootstrap_method(&entry, bootstrap_method_attr_index)?;
                let (_, descriptor) = entry.name_and_type(name_and_type_index)?;
                entry.ensure(
                    descriptor.starts_with('('),
                    format_args!("{descriptor:?} is not a method descriptor"),
                )
            }
            Module { name_index } => {
                self.check_declares_module(&entry)?;
                let name = entry.utf8("name_index", name_index)?;
                entry.ensure(
                    is_module_name(name),
                    format_args!("{name:?} is not a valid module name"),
                )
            }
            Package { name_index } => {
                self.check_declares_module(&entry)?;
                let name = entry.utf8("name_index", name_index)?;
                entry.ensure(
                    is_binary_name(name),
                    format_args!("{name:?} is not a valid package name"),
                )
            }
        }
    }

    fn check_method_handle(
        &self,
        entry: &Entry<'a>,
        reference_kind: u8,
        reference_index: u16,
    ) -> Result<()> {
        let target = self.constant_pool.get(reference_index as usize);

        let (is_expected, expected) = match reference_kind {
            1..=4 => (matches!(target, Some(Fieldref { .. })), "a Fieldref"),
            5 | 8 => (matches!(target, Some(Methodref { .. })), "a Methodref"),
            6 | 7 if self.major_version < 52 => {
                (matches!(target, Some(Methodref { .. })), "a Methodref")
            }
            6 | 7 => (
                matches!(target, Some(Methodref { .. } | InterfaceMethodref { .. })),
                "a Methodref or an InterfaceMethodref",
            ),
            9 => (
                matches!(target, Some(InterfaceMethodref { .. })),
                "an InterfaceMethodref",
            ),
            _ => {
                return Err(entry.error(format_args!(
                    "reference_kind {reference_kind} is not in the range 1 to 9"
                )))
            }
        };
        entry.ensure(
            is_expected,
            format_args!(
                "reference_index #{reference_index} is not {expected} entry, as reference_kind {reference_kind} requires"
            ),
        )?;

        // A reference that cannot be followed down to its name is reported by its own check.
        let Some(name) = self.method_name(target) else {
            return Ok(());
        };
        match reference_kind {
            5 | 6 | 7 | 9 => entry.ensure(
                name != "<init>" && name != "<clinit>",
                format_args!("reference_kind {reference_kind} must not refer to {name}"),
            ),
            8 => entry.ensure(
                name == "<init>",
                format_args!("reference_kind 8 must refer to <init>, not to {name:?}"),
            ),
            _ => Ok(()),
        }
    }

    /// The name of the method that a `Methodref` or an `InterfaceMethodref` entry represents.
    fn method_name(&self, reference: Option<&ConstantPool>) -> Option<&'a str> {
        let (Methodref {
            name_and_type_index,
            ..
        }
        | InterfaceMethodref {
            name_and_type_index,
            ..
        }) = reference?
        else {
            return None;
        };

        match self.constant_pool.get(*name_and_type_index as usize)? {
            NameAndType { name_index, .. } => {
                match self.constant_pool.get(*name_index as usize)? {
                    Utf8 { value } => Some(value),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn check_bootstrap_method(
        &self,
        entry: &Entry<'a>,
        bootstrap_method_attr_index: u16,
    ) -> Result<()> {
        entry.ensure(
            (bootstrap_method_attr_index as usize) < self.bootstrap_methods_count,
            format_args!(
                "bootstrap_method_attr_index {bootstrap_method_attr_index} is not a valid index into the bootstrap_methods array, whose length is {}",
                self.bootstrap_methods_count
            ),
        )
    }

    fn check_declares_module(&self, entry: &Entry<'a>) -> Result<()> {
        entry.ensure(
            self.is_module,
            format_args!(
                "permitted only in a class file that declares a module, where ACC_MODULE is set"
            ),
        )
    }

    fn entry(&self, index: usize) -> Entry<'a> {
        let (kind, section, since) = describe(&self.constant_pool[index]);

        Entry {
            constant_pool: self.constant_pool,
            index,
            kind,
            section,
            since,
        }
    }
}

/// A constant pool entry under check: follows the indexes it holds and words its failures.
struct Entry<'a> {
    constant_pool: &'a [ConstantPool],
    index: usize,
    kind: &'static str,
    section: &'static str,
    since: u16,
}

impl<'a> Entry<'a> {
    fn error(&self, problem: impl Display) -> Error {
        Error::format(format!(
            "constant pool entry #{} ({}): {} (JVMS §{})",
            self.index, self.kind, problem, self.section
        ))
    }

    fn ensure(&self, condition: bool, problem: Arguments<'_>) -> Result<()> {
        if condition {
            Ok(())
        } else {
            Err(self.error(problem))
        }
    }

    fn utf8(&self, item: &str, index: u16) -> Result<&'a str> {
        match self.constant_pool.get(index as usize) {
            Some(Utf8 { value }) => Ok(value),
            _ => Err(self.error(format_args!("{item} #{index} is not a Utf8 entry"))),
        }
    }

    fn class(&self, item: &str, index: u16) -> Result<()> {
        self.ensure(
            matches!(self.constant_pool.get(index as usize), Some(Class { .. })),
            format_args!("{item} #{index} is not a Class entry"),
        )
    }

    /// The name and the descriptor that the `NameAndType` entry at `index` indicates.
    fn name_and_type(&self, index: u16) -> Result<(&'a str, &'a str)> {
        match self.constant_pool.get(index as usize) {
            Some(&NameAndType {
                name_index,
                descriptor_index,
            }) => {
                // What is wrong inside the `NameAndType` entry is wrong with that entry.
                let name_and_type = Entry {
                    constant_pool: self.constant_pool,
                    index: index as usize,
                    kind: "NameAndType",
                    section: "4.4.6",
                    since: 45,
                };

                Ok((
                    name_and_type.utf8("name_index", name_index)?,
                    name_and_type.utf8("descriptor_index", descriptor_index)?,
                ))
            }
            _ => Err(self.error(format_args!(
                "name_and_type_index #{index} is not a NameAndType entry"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attributes::BootstrapMethodRecord;

    const JAVA_8: u16 = 52;
    const JAVA_25: u16 = 69;

    fn utf8(value: &str) -> ConstantPool {
        Utf8 {
            value: value.into(),
        }
    }

    /// Format-checks a class file that has nothing but the given constant pool entries, numbered
    /// from one, and a single bootstrap method.
    fn check(
        major_version: u16,
        access_flags: ClassFlags,
        entries: Vec<ConstantPool>,
    ) -> Result<()> {
        let mut constant_pool = vec![Empty];
        constant_pool.extend(entries);

        check_format(&ClassFile::new(
            0xCAFEBABE,
            0,
            major_version,
            constant_pool,
            access_flags,
            0,
            0,
            vec![],
            vec![],
            vec![],
            vec![Attribute::BootstrapMethods {
                bootstrap_methods: vec![BootstrapMethodRecord::new(0, vec![])],
            }],
        ))
    }

    fn check_class(entries: Vec<ConstantPool>) -> Result<()> {
        check(JAVA_25, ClassFlags::ACC_SUPER, entries)
    }

    fn check_module(entries: Vec<ConstantPool>) -> Result<()> {
        check(JAVA_25, ClassFlags::ACC_MODULE, entries)
    }

    fn assert_rejected(result: Result<()>, expected: &str) {
        match result {
            Err(error) => assert_eq!(error.to_string(), format!("Format Error: {expected}")),
            Ok(()) => panic!("Expected a format error: {expected}"),
        }
    }

    /// A reference to `Owner.<name>:<descriptor>` of the given kind in entry #1.
    fn reference(
        kind: fn(u16, u16) -> ConstantPool,
        name: &str,
        descriptor: &str,
    ) -> Vec<ConstantPool> {
        vec![
            kind(2, 4),              //                                      1
            Class { name_index: 3 }, //                         2
            utf8("Owner"),           //                                   3
            NameAndType {
                //                                    4
                name_index: 5,
                descriptor_index: 6,
            },
            utf8(name),       //                                      5
            utf8(descriptor), //                                6
        ]
    }

    fn fieldref(class_index: u16, name_and_type_index: u16) -> ConstantPool {
        Fieldref {
            class_index,
            name_and_type_index,
        }
    }

    fn methodref(class_index: u16, name_and_type_index: u16) -> ConstantPool {
        Methodref {
            class_index,
            name_and_type_index,
        }
    }

    fn interface_methodref(class_index: u16, name_and_type_index: u16) -> ConstantPool {
        InterfaceMethodref {
            class_index,
            name_and_type_index,
        }
    }

    /// A method handle of the given kind in entry #7, on the reference in entry #1.
    fn method_handle(reference_kind: u8, mut reference: Vec<ConstantPool>) -> Vec<ConstantPool> {
        reference.push(MethodHandle {
            reference_kind,
            reference_index: 1,
        });
        reference
    }

    #[test]
    fn test_empty_constant_pool() {
        let class_file = ClassFile::new(
            0xCAFEBABE,
            0,
            JAVA_25,
            vec![],
            ClassFlags::ACC_SUPER,
            0,
            0,
            vec![],
            vec![],
            vec![],
            vec![],
        );

        assert_rejected(
            check_format(&class_file),
            "constant_pool_count is zero (JVMS §4.1)",
        );
    }

    #[test]
    fn test_eight_byte_constant_has_second_entry() {
        assert!(check_class(vec![Long { value: 1 }, Empty]).is_ok());
        assert!(check_class(vec![Double { value: 1.0 }, Empty]).is_ok());

        assert_rejected(
            check_class(vec![Long { value: 1 }]),
            "constant pool entry #1: an 8-byte constant takes up two entries, but this is the last one (JVMS §4.4.5)",
        );
        assert_rejected(
            check_class(vec![Integer { value: 1 }, Double { value: 1.0 }]),
            "constant pool entry #2: an 8-byte constant takes up two entries, but this is the last one (JVMS §4.4.5)",
        );
    }

    #[test]
    fn test_tag_against_class_file_version() {
        let method_type = vec![
            MethodType {
                descriptor_index: 2,
            },
            utf8("()V"),
        ];
        assert!(check(51, ClassFlags::ACC_SUPER, method_type.clone()).is_ok());
        assert_rejected(
            check(50, ClassFlags::ACC_SUPER, method_type),
            "constant pool entry #1 (MethodType): the tag was first defined in version 51.0 of the class file format, but the class file has major version 50 (JVMS §4.4)",
        );

        let invoke_dynamic = || {
            reference(
                |bootstrap_method_attr_index, name_and_type_index| InvokeDynamic {
                    bootstrap_method_attr_index: bootstrap_method_attr_index - 2,
                    name_and_type_index,
                },
                "run",
                "()V",
            )
        };
        assert!(check(51, ClassFlags::ACC_SUPER, invoke_dynamic()).is_ok());
        assert_rejected(
            check(50, ClassFlags::ACC_SUPER, invoke_dynamic()),
            "constant pool entry #1 (InvokeDynamic): the tag was first defined in version 51.0 of the class file format, but the class file has major version 50 (JVMS §4.4)",
        );

        let dynamic = || {
            reference(
                |bootstrap_method_attr_index, name_and_type_index| Dynamic {
                    bootstrap_method_attr_index: bootstrap_method_attr_index - 2,
                    name_and_type_index,
                },
                "constant",
                "I",
            )
        };
        assert!(check(55, ClassFlags::ACC_SUPER, dynamic()).is_ok());
        assert_rejected(
            check(54, ClassFlags::ACC_SUPER, dynamic()),
            "constant pool entry #1 (Dynamic): the tag was first defined in version 55.0 of the class file format, but the class file has major version 54 (JVMS §4.4)",
        );

        let module = vec![Module { name_index: 2 }, utf8("java.base")];
        assert!(check(53, ClassFlags::ACC_MODULE, module.clone()).is_ok());
        assert_rejected(
            check(52, ClassFlags::ACC_MODULE, module),
            "constant pool entry #1 (Module): the tag was first defined in version 53.0 of the class file format, but the class file has major version 52 (JVMS §4.4)",
        );
    }

    #[test]
    fn test_class() {
        assert!(check_class(vec![Class { name_index: 2 }, utf8("java/lang/Object")]).is_ok());
        assert!(check_class(vec![Class { name_index: 2 }, utf8("[[Ljava/lang/Object;")]).is_ok());

        assert_rejected(
            check_class(vec![Class { name_index: 2 }, Integer { value: 1 }]),
            "constant pool entry #1 (Class): name_index #2 is not a Utf8 entry (JVMS §4.4.1)",
        );
        assert_rejected(
            check_class(vec![Class { name_index: 2 }]),
            "constant pool entry #1 (Class): name_index #2 is not a Utf8 entry (JVMS §4.4.1)",
        );
        assert_rejected(
            check_class(vec![Class { name_index: 0 }]),
            "constant pool entry #1 (Class): name_index #0 is not a Utf8 entry (JVMS §4.4.1)",
        );
        assert_rejected(
            check_class(vec![Class { name_index: 2 }, utf8("java.lang.Object")]),
            "constant pool entry #1 (Class): \"java.lang.Object\" is not a valid class or interface name (JVMS §4.4.1)",
        );
    }

    #[test]
    fn test_string() {
        assert!(check_class(vec![String { string_index: 2 }, utf8("")]).is_ok());

        assert_rejected(
            check_class(vec![String { string_index: 2 }, Float { value: 1.0 }]),
            "constant pool entry #1 (String): string_index #2 is not a Utf8 entry (JVMS §4.4.3)",
        );
    }

    #[test]
    fn test_unusable_entry_is_not_referred_to() {
        assert_rejected(
            check_class(vec![String { string_index: 3 }, Long { value: 1 }, Empty]),
            "constant pool entry #1 (String): string_index #3 is not a Utf8 entry (JVMS §4.4.3)",
        );
    }

    #[test]
    fn test_fieldref() {
        assert!(check_class(reference(fieldref, "someText", "Ljava/lang/String;")).is_ok());
        // Only method names are barred from having angle brackets
        assert!(check_class(reference(fieldref, "<init>", "I")).is_ok());

        assert_rejected(
            check_class(reference(fieldref, "someText", "()V")),
            "constant pool entry #1 (Fieldref): \"()V\" is not a field descriptor (JVMS §4.4.2)",
        );
        assert_rejected(
            check_class(reference(fieldref, "someText", "Ljava/lang/String")),
            "constant pool entry #4 (NameAndType): \"Ljava/lang/String\" is not a valid field descriptor (JVMS §4.4.6)",
        );
        assert_rejected(
            check_class(reference(fieldref, "some.Text", "I")),
            "constant pool entry #4 (NameAndType): \"some.Text\" is not a valid field name (JVMS §4.4.6)",
        );
        assert_rejected(
            check_class(reference(
                |_, name_and_type_index| fieldref(3, name_and_type_index),
                "someText",
                "I",
            )),
            "constant pool entry #1 (Fieldref): class_index #3 is not a Class entry (JVMS §4.4.2)",
        );
        assert_rejected(
            check_class(reference(|class_index, _| fieldref(class_index, 5), "someText", "I")),
            "constant pool entry #1 (Fieldref): name_and_type_index #5 is not a NameAndType entry (JVMS §4.4.2)",
        );
    }

    #[test]
    fn test_methodref() {
        assert!(check_class(reference(methodref, "run", "()V")).is_ok());
        assert!(check_class(reference(methodref, "<init>", "(Ljava/lang/String;)V")).is_ok());

        assert_rejected(
            check_class(reference(methodref, "run", "I")),
            "constant pool entry #1 (Methodref): \"I\" is not a method descriptor (JVMS §4.4.2)",
        );
        assert_rejected(
            check_class(reference(methodref, "<clinit>", "()V")),
            "constant pool entry #1 (Methodref): the name \"<clinit>\" begins with '<', but is not <init> (JVMS §4.4.2)",
        );
        assert_rejected(
            check_class(reference(methodref, "<init>", "()I")),
            "constant pool entry #1 (Methodref): <init> must return void, but its descriptor is \"()I\" (JVMS §4.4.2)",
        );
        assert_rejected(
            check_class(reference(methodref, "ru<n", "()V")),
            "constant pool entry #4 (NameAndType): \"ru<n\" is not a valid method name (JVMS §4.4.6)",
        );
        assert_rejected(
            check_class(reference(methodref, "run", "(V)V")),
            "constant pool entry #4 (NameAndType): \"(V)V\" is not a valid method descriptor (JVMS §4.4.6)",
        );
    }

    #[test]
    fn test_interface_methodref() {
        assert!(check_class(reference(interface_methodref, "run", "()V")).is_ok());

        assert_rejected(
            check_class(reference(interface_methodref, "run", "I")),
            "constant pool entry #1 (InterfaceMethodref): \"I\" is not a method descriptor (JVMS §4.4.2)",
        );
        assert_rejected(
            check_class(reference(|_, name_and_type_index| interface_methodref(1, name_and_type_index), "run", "()V")),
            "constant pool entry #1 (InterfaceMethodref): class_index #1 is not a Class entry (JVMS §4.4.2)",
        );
    }

    #[test]
    fn test_name_and_type() {
        let name_and_type = |name: ConstantPool, descriptor: ConstantPool| {
            vec![
                NameAndType {
                    name_index: 2,
                    descriptor_index: 3,
                },
                name,
                descriptor,
            ]
        };

        assert!(check_class(name_and_type(utf8("<clinit>"), utf8("()V"))).is_ok());

        assert_rejected(
            check_class(name_and_type(Integer { value: 1 }, utf8("I"))),
            "constant pool entry #1 (NameAndType): name_index #2 is not a Utf8 entry (JVMS §4.4.6)",
        );
        assert_rejected(
            check_class(name_and_type(utf8("value"), Integer { value: 1 })),
            "constant pool entry #1 (NameAndType): descriptor_index #3 is not a Utf8 entry (JVMS §4.4.6)",
        );
        assert_rejected(
            check_class(name_and_type(utf8(""), utf8("I"))),
            "constant pool entry #1 (NameAndType): \"\" is not a valid field name (JVMS §4.4.6)",
        );
        assert_rejected(
            check_class(name_and_type(utf8("value"), utf8("V"))),
            "constant pool entry #1 (NameAndType): \"V\" is not a valid field descriptor (JVMS §4.4.6)",
        );
    }

    #[test]
    fn test_method_handle_reference_kind() {
        assert_rejected(
            check_class(method_handle(0, reference(fieldref, "someText", "I"))),
            "constant pool entry #7 (MethodHandle): reference_kind 0 is not in the range 1 to 9 (JVMS §4.4.8)",
        );
        assert_rejected(
            check_class(method_handle(10, reference(fieldref, "someText", "I"))),
            "constant pool entry #7 (MethodHandle): reference_kind 10 is not in the range 1 to 9 (JVMS §4.4.8)",
        );
    }

    #[test]
    fn test_method_handle_reference_index() {
        for reference_kind in 1..=4 {
            assert!(check_class(method_handle(
                reference_kind,
                reference(fieldref, "someText", "I")
            ))
            .is_ok());
        }
        assert!(check_class(method_handle(5, reference(methodref, "run", "()V"))).is_ok());
        assert!(check_class(method_handle(
            9,
            reference(interface_methodref, "run", "()V")
        ))
        .is_ok());

        assert_rejected(
            check_class(method_handle(1, reference(methodref, "run", "()V"))),
            "constant pool entry #7 (MethodHandle): reference_index #1 is not a Fieldref entry, as reference_kind 1 requires (JVMS §4.4.8)",
        );
        assert_rejected(
            check_class(method_handle(5, reference(interface_methodref, "run", "()V"))),
            "constant pool entry #7 (MethodHandle): reference_index #1 is not a Methodref entry, as reference_kind 5 requires (JVMS §4.4.8)",
        );
        assert_rejected(
            check_class(method_handle(9, reference(methodref, "run", "()V"))),
            "constant pool entry #7 (MethodHandle): reference_index #1 is not an InterfaceMethodref entry, as reference_kind 9 requires (JVMS §4.4.8)",
        );
        assert_rejected(
            check_class(vec![MethodHandle {
                reference_kind: 1,
                reference_index: 2,
            }]),
            "constant pool entry #1 (MethodHandle): reference_index #2 is not a Fieldref entry, as reference_kind 1 requires (JVMS §4.4.8)",
        );
    }

    #[test]
    fn test_method_handle_on_interface_method_against_class_file_version() {
        for reference_kind in [6, 7] {
            let on_interface_method =
                || method_handle(reference_kind, reference(interface_methodref, "run", "()V"));

            assert!(check(JAVA_8, ClassFlags::ACC_SUPER, on_interface_method()).is_ok());
            assert_rejected(
                check(JAVA_8 - 1, ClassFlags::ACC_SUPER, on_interface_method()),
                &format!("constant pool entry #7 (MethodHandle): reference_index #1 is not a Methodref entry, as reference_kind {reference_kind} requires (JVMS §4.4.8)"),
            );
            assert_rejected(
                check(JAVA_8, ClassFlags::ACC_SUPER, method_handle(reference_kind, reference(fieldref, "someText", "I"))),
                &format!("constant pool entry #7 (MethodHandle): reference_index #1 is not a Methodref or an InterfaceMethodref entry, as reference_kind {reference_kind} requires (JVMS §4.4.8)"),
            );
        }
    }

    #[test]
    fn test_method_handle_method_name() {
        assert!(check_class(method_handle(8, reference(methodref, "<init>", "()V"))).is_ok());

        assert_rejected(
            check_class(method_handle(8, reference(methodref, "run", "()V"))),
            "constant pool entry #7 (MethodHandle): reference_kind 8 must refer to <init>, not to \"run\" (JVMS §4.4.8)",
        );
        assert_rejected(
            check_class(method_handle(5, reference(methodref, "<init>", "()V"))),
            "constant pool entry #7 (MethodHandle): reference_kind 5 must not refer to <init> (JVMS §4.4.8)",
        );
        assert_rejected(
            check_class(method_handle(9, reference(interface_methodref, "<clinit>", "()V"))),
            "constant pool entry #7 (MethodHandle): reference_kind 9 must not refer to <clinit> (JVMS §4.4.8)",
        );
    }

    #[test]
    fn test_method_type() {
        assert!(check_class(vec![
            MethodType {
                descriptor_index: 2
            },
            utf8("(I)J")
        ])
        .is_ok());

        assert_rejected(
            check_class(vec![MethodType { descriptor_index: 2 }, utf8("J")]),
            "constant pool entry #1 (MethodType): \"J\" is not a valid method descriptor (JVMS §4.4.9)",
        );
        assert_rejected(
            check_class(vec![MethodType { descriptor_index: 1 }]),
            "constant pool entry #1 (MethodType): descriptor_index #1 is not a Utf8 entry (JVMS §4.4.9)",
        );
    }

    #[test]
    fn test_dynamic() {
        let dynamic = |bootstrap_method_attr_index, descriptor| {
            vec![
                Dynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index: 2,
                },
                NameAndType {
                    name_index: 3,
                    descriptor_index: 4,
                },
                utf8("constant"),
                utf8(descriptor),
            ]
        };

        assert!(check_class(dynamic(0, "Ljava/lang/Object;")).is_ok());

        assert_rejected(
            check_class(dynamic(0, "()Ljava/lang/Object;")),
            "constant pool entry #1 (Dynamic): \"()Ljava/lang/Object;\" is not a field descriptor (JVMS §4.4.10)",
        );
        assert_rejected(
            check_class(dynamic(1, "Ljava/lang/Object;")),
            "constant pool entry #1 (Dynamic): bootstrap_method_attr_index 1 is not a valid index into the bootstrap_methods array, whose length is 1 (JVMS §4.4.10)",
        );
    }

    #[test]
    fn test_invoke_dynamic() {
        let invoke_dynamic = |bootstrap_method_attr_index, name_and_type_index, descriptor| {
            vec![
                InvokeDynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                },
                NameAndType {
                    name_index: 3,
                    descriptor_index: 4,
                },
                utf8("run"),
                utf8(descriptor),
            ]
        };

        assert!(check_class(invoke_dynamic(0, 2, "()Ljava/lang/Runnable;")).is_ok());

        assert_rejected(
            check_class(invoke_dynamic(0, 2, "Ljava/lang/Runnable;")),
            "constant pool entry #1 (InvokeDynamic): \"Ljava/lang/Runnable;\" is not a method descriptor (JVMS §4.4.10)",
        );
        assert_rejected(
            check_class(invoke_dynamic(0, 3, "()Ljava/lang/Runnable;")),
            "constant pool entry #1 (InvokeDynamic): name_and_type_index #3 is not a NameAndType entry (JVMS §4.4.10)",
        );
        assert_rejected(
            check_class(invoke_dynamic(7, 2, "()Ljava/lang/Runnable;")),
            "constant pool entry #1 (InvokeDynamic): bootstrap_method_attr_index 7 is not a valid index into the bootstrap_methods array, whose length is 1 (JVMS §4.4.10)",
        );
    }

    #[test]
    fn test_module() {
        assert!(check_module(vec![Module { name_index: 2 }, utf8("java.base")]).is_ok());

        assert_rejected(
            check_class(vec![Module { name_index: 2 }, utf8("java.base")]),
            "constant pool entry #1 (Module): permitted only in a class file that declares a module, where ACC_MODULE is set (JVMS §4.4.11)",
        );
        assert_rejected(
            check_module(vec![Module { name_index: 2 }, utf8("java@base")]),
            "constant pool entry #1 (Module): \"java@base\" is not a valid module name (JVMS §4.4.11)",
        );
        assert_rejected(
            check_module(vec![Module { name_index: 1 }]),
            "constant pool entry #1 (Module): name_index #1 is not a Utf8 entry (JVMS §4.4.11)",
        );
    }

    #[test]
    fn test_package() {
        assert!(check_module(vec![Package { name_index: 2 }, utf8("java/lang")]).is_ok());

        assert_rejected(
            check_class(vec![Package { name_index: 2 }, utf8("java/lang")]),
            "constant pool entry #1 (Package): permitted only in a class file that declares a module, where ACC_MODULE is set (JVMS §4.4.12)",
        );
        assert_rejected(
            check_module(vec![Package { name_index: 2 }, utf8("java.lang")]),
            "constant pool entry #1 (Package): \"java.lang\" is not a valid package name (JVMS §4.4.12)",
        );
    }
}
