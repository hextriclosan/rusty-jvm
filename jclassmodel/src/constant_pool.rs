use derive_new::new;
use jclassfile::constant_pool::ConstantPool as Entry;
use std::collections::HashMap;

#[cfg_attr(test, mockall::automock)]
pub trait ConstantPoolLookup {
    fn get_class_name(&self, index: u16) -> Option<String>;
    fn get_integer(&self, index: u16) -> Option<i32>;
    fn get_float(&self, index: u16) -> Option<f32>;
    fn get_double(&self, index: u16) -> Option<f64>;
    fn get_string(&self, index: u16) -> Option<String>;
    fn get_long(&self, index: u16) -> Option<i64>;
    fn get_utf8(&self, index: u16) -> Option<String>;
    fn get_full_field_info(&self, index: u16) -> Option<(String, String, String)>;
    fn get_full_method_info(&self, index: u16) -> Option<(String, String, String)>;
    fn get_full_interfacemethodref_info(&self, index: u16) -> Option<(String, String, String)>;
    fn get_name_and_type(&self, index: u16) -> Option<(String, String)>;
    fn get_invoke_dynamic(&self, index: u16) -> Option<(u16, String, String)>;
    fn get_method_handle(&self, index: u16) -> Option<(u8, String, String, String)>;
    fn get_method_type(&self, index: u16) -> Option<String>;
}

#[derive(Debug)]
pub struct ConstantPool {
    data: HashMap<CPoolType, HashMap<u16, Entry>>,
    raw_cpool: Vec<Entry>,
    loaded_classname: Option<LoadedClassName>,
}

#[derive(Debug, new)]
struct LoadedClassName {
    index: u16,
    name: String,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub(crate) enum CPoolType {
    Empty,
    Utf8,
    Integer,
    Float,
    Long,
    Double,
    Class,
    String,
    Fieldref,
    Methodref,
    InterfaceMethodref,
    NameAndType,
    MethodHandle,
    MethodType,
    Dynamic,
    InvokeDynamic,
    Module,
    Package,
}

impl From<&Entry> for CPoolType {
    fn from(item: &Entry) -> Self {
        match item {
            Entry::Empty => CPoolType::Empty,
            Entry::Utf8 { .. } => CPoolType::Utf8,
            Entry::Integer { .. } => CPoolType::Integer,
            Entry::Float { .. } => CPoolType::Float,
            Entry::Long { .. } => CPoolType::Long,
            Entry::Double { .. } => CPoolType::Double,
            Entry::Class { .. } => CPoolType::Class,
            Entry::String { .. } => CPoolType::String,
            Entry::Fieldref { .. } => CPoolType::Fieldref,
            Entry::Methodref { .. } => CPoolType::Methodref,
            Entry::InterfaceMethodref { .. } => CPoolType::InterfaceMethodref,
            Entry::NameAndType { .. } => CPoolType::NameAndType,
            Entry::MethodHandle { .. } => CPoolType::MethodHandle,
            Entry::MethodType { .. } => CPoolType::MethodType,
            Entry::Dynamic { .. } => CPoolType::Dynamic,
            Entry::InvokeDynamic { .. } => CPoolType::InvokeDynamic,
            Entry::Module { .. } => CPoolType::Module,
            Entry::Package { .. } => CPoolType::Package,
        }
    }
}

impl ConstantPool {
    pub(crate) fn new(cpool: &[Entry]) -> Self {
        Self::new_impl(cpool, None)
    }
    pub(crate) fn new_with_classname(
        cpool: &[Entry],
        loaded_classname_index: u16,
        loaded_classname: String,
    ) -> Self {
        Self::new_impl(
            cpool,
            Some(LoadedClassName::new(
                loaded_classname_index,
                loaded_classname,
            )),
        )
    }

    fn new_impl(cpool: &[Entry], loaded_classname: Option<LoadedClassName>) -> Self {
        let mut data: HashMap<CPoolType, HashMap<u16, Entry>> = HashMap::new();

        for (index, item) in cpool.iter().enumerate() {
            let ctype = item.into();
            let entry = data.entry(ctype).or_default();
            entry.insert(index as u16, item.clone());
        }

        Self {
            data,
            raw_cpool: cpool.to_vec(),
            loaded_classname,
        }
    }

    /// An empty pool, for classes the VM invents rather than loads (primitives, arrays).
    pub fn empty() -> Self {
        Self::new(&[])
    }

    fn get(&self, ctype: CPoolType, index: u16) -> Option<&Entry> {
        self.data.get(&ctype)?.get(&index)
    }

    fn get_first(&self, ctypes: &[CPoolType], index: u16) -> Option<&Entry> {
        for &ctype in ctypes {
            if let Some(constant_pool) = self.get(ctype, index) {
                return Some(constant_pool);
            }
        }
        None
    }

    /// Number of entries, counting the unused entry 0 and the padding that follows every `Long` and
    /// `Double`, so this matches `constant_pool_count` from the class file.
    pub fn len(&self) -> usize {
        self.raw_cpool.len()
    }

    pub fn is_empty(&self) -> bool {
        self.raw_cpool.is_empty()
    }

    /// The tag of the entry at `index`, or `None` if there is no such entry.
    pub fn tag_at(&self, index: usize) -> Option<ConstantTag> {
        self.raw_cpool.get(index).map(ConstantTag::of)
    }
}

/// The tag byte identifying a constant-pool entry's kind (JVMS Table 4.4-B).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstantTag {
    /// Not a real entry: index 0, or the padding that follows a `Long` or `Double`.
    Empty,
    Utf8,
    Integer,
    Float,
    Long,
    Double,
    Class,
    String,
    Fieldref,
    Methodref,
    InterfaceMethodref,
    NameAndType,
    MethodHandle,
    MethodType,
    Dynamic,
    InvokeDynamic,
    Module,
    Package,
}

impl ConstantTag {
    /// The tag's numeric value as it appears in the class file.
    pub fn as_u8(self) -> u8 {
        match self {
            ConstantTag::Empty => 0,
            ConstantTag::Utf8 => 1,
            ConstantTag::Integer => 3,
            ConstantTag::Float => 4,
            ConstantTag::Long => 5,
            ConstantTag::Double => 6,
            ConstantTag::Class => 7,
            ConstantTag::String => 8,
            ConstantTag::Fieldref => 9,
            ConstantTag::Methodref => 10,
            ConstantTag::InterfaceMethodref => 11,
            ConstantTag::NameAndType => 12,
            ConstantTag::MethodHandle => 15,
            ConstantTag::MethodType => 16,
            ConstantTag::Dynamic => 17,
            ConstantTag::InvokeDynamic => 18,
            ConstantTag::Module => 19,
            ConstantTag::Package => 20,
        }
    }
}

impl ConstantTag {
    fn of(item: &Entry) -> Self {
        match item {
            Entry::Empty => ConstantTag::Empty,
            Entry::Utf8 { .. } => ConstantTag::Utf8,
            Entry::Integer { .. } => ConstantTag::Integer,
            Entry::Float { .. } => ConstantTag::Float,
            Entry::Long { .. } => ConstantTag::Long,
            Entry::Double { .. } => ConstantTag::Double,
            Entry::Class { .. } => ConstantTag::Class,
            Entry::String { .. } => ConstantTag::String,
            Entry::Fieldref { .. } => ConstantTag::Fieldref,
            Entry::Methodref { .. } => ConstantTag::Methodref,
            Entry::InterfaceMethodref { .. } => ConstantTag::InterfaceMethodref,
            Entry::NameAndType { .. } => ConstantTag::NameAndType,
            Entry::MethodHandle { .. } => ConstantTag::MethodHandle,
            Entry::MethodType { .. } => ConstantTag::MethodType,
            Entry::Dynamic { .. } => ConstantTag::Dynamic,
            Entry::InvokeDynamic { .. } => ConstantTag::InvokeDynamic,
            Entry::Module { .. } => ConstantTag::Module,
            Entry::Package { .. } => ConstantTag::Package,
        }
    }
}

impl ConstantPoolLookup for ConstantPool {
    fn get_class_name(&self, index: u16) -> Option<String> {
        // if current classname is requested, we return the name of the loaded class
        // which might be different from the one in the constant pool (e.g. constant pool value `invokedynamic/lambda/LambdaExample$$Lambda` loaded as `invokedynamic/lambda/LambdaExample$$Lambda/0x0000000000000001`)
        if let Some(lcn) = &self.loaded_classname {
            if lcn.index == index {
                return Some(lcn.name.clone());
            }
        }

        let name_index = match *self.get(CPoolType::Class, index)? {
            Entry::Class { name_index } => name_index,
            _ => return None,
        };

        self.get_utf8(name_index)
    }

    fn get_integer(&self, index: u16) -> Option<i32> {
        match self.get(CPoolType::Integer, index)? {
            Entry::Integer { value } => Some(*value),
            _ => None,
        }
    }

    fn get_float(&self, index: u16) -> Option<f32> {
        match self.get(CPoolType::Float, index)? {
            Entry::Float { value } => Some(*value),
            _ => None,
        }
    }

    fn get_double(&self, index: u16) -> Option<f64> {
        match self.get(CPoolType::Double, index)? {
            Entry::Double { value } => Some(*value),
            _ => None,
        }
    }

    fn get_string(&self, index: u16) -> Option<String> {
        let name_index = match self.get(CPoolType::String, index)? {
            Entry::String { string_index } => Some(string_index),
            _ => None,
        }?;

        self.get_utf8(*name_index)
    }

    fn get_long(&self, index: u16) -> Option<i64> {
        match self.get(CPoolType::Long, index)? {
            Entry::Long { value } => Some(*value),
            _ => None,
        }
    }

    fn get_utf8(&self, index: u16) -> Option<String> {
        match self.get(CPoolType::Utf8, index)? {
            Entry::Utf8 { value } => Some(value.clone()),
            _ => None,
        }
    }

    fn get_full_field_info(&self, index: u16) -> Option<(String, String, String)> {
        let (class_index, name_and_type_index) = match self.get(CPoolType::Fieldref, index)? {
            Entry::Fieldref {
                class_index,
                name_and_type_index,
            } => Some((class_index, name_and_type_index)),
            _ => None,
        }?;

        let class_name = self.get_class_name(*class_index)?;
        let (field_name, descriptor_name) = self.get_name_and_type(*name_and_type_index)?;

        Some((class_name, field_name, descriptor_name))
    }

    fn get_full_method_info(&self, index: u16) -> Option<(String, String, String)> {
        let constant_pool = self.get_first(
            &[CPoolType::Methodref, CPoolType::InterfaceMethodref],
            index,
        )?;

        let (class_index, name_and_type_index) = if let Entry::Methodref {
            class_index,
            name_and_type_index,
        }
        | Entry::InterfaceMethodref {
            class_index,
            name_and_type_index,
        } = constant_pool
        {
            (class_index, name_and_type_index)
        } else {
            return None;
        };

        let class_name = self.get_class_name(*class_index)?;
        let (method_name, method_descriptor) = self.get_name_and_type(*name_and_type_index)?;

        Some((class_name, method_name, method_descriptor))
    }

    fn get_full_interfacemethodref_info(&self, index: u16) -> Option<(String, String, String)> {
        let (class_index, name_and_type_index) =
            match self.get(CPoolType::InterfaceMethodref, index)? {
                Entry::InterfaceMethodref {
                    class_index,
                    name_and_type_index,
                } => Some((class_index, name_and_type_index)),
                _ => None,
            }?;

        let class_name = self.get_class_name(*class_index)?;
        let (method_name, method_descriptor) = self.get_name_and_type(*name_and_type_index)?;

        Some((class_name, method_name, method_descriptor))
    }

    fn get_name_and_type(&self, index: u16) -> Option<(String, String)> {
        let (name_index, descriptor_index) = match self.get(CPoolType::NameAndType, index)? {
            Entry::NameAndType {
                name_index,
                descriptor_index,
            } => Some((name_index, descriptor_index)),
            _ => None,
        }?;

        let name = self.get_utf8(*name_index)?;
        let descriptor = self.get_utf8(*descriptor_index)?;

        Some((name, descriptor))
    }

    fn get_invoke_dynamic(&self, index: u16) -> Option<(u16, String, String)> {
        let (bootstrap_method_attr_index, name_and_type_index) =
            match self.get(CPoolType::InvokeDynamic, index)? {
                Entry::InvokeDynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                } => Some((*bootstrap_method_attr_index, *name_and_type_index)),
                _ => None,
            }?;

        let (method_name, method_descriptor) = self.get_name_and_type(name_and_type_index)?;

        Some((bootstrap_method_attr_index, method_name, method_descriptor))
    }

    fn get_method_handle(&self, index: u16) -> Option<(u8, String, String, String)> {
        let (reference_kind, reference_index) = match self.get(CPoolType::MethodHandle, index)? {
            Entry::MethodHandle {
                reference_kind,
                reference_index,
            } => Some((*reference_kind, *reference_index)),
            _ => None,
        }?;

        let (class_name, name, descriptor) = match reference_kind {
            1..=4 => {
                self.get_full_field_info(reference_index)? // For Fieldref
            }
            5..=9 => {
                self.get_full_method_info(reference_index)? // For Methodref, InterfaceMethodref
            }
            _ => {
                return None; // Unsupported reference kind todo: consider returning an error
            }
        };

        Some((reference_kind, class_name, name, descriptor))
    }

    fn get_method_type(&self, index: u16) -> Option<String> {
        let utf8_index = match self.get(CPoolType::MethodType, index)? {
            Entry::MethodType { descriptor_index } => Some(descriptor_index),
            _ => None,
        }?;

        self.get_utf8(*utf8_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jclassfile::constant_pool::ConstantPool::{
        Class, Double, Empty, Fieldref, Float, Integer, InterfaceMethodref, InvokeDynamic, Long,
        MethodHandle, MethodType, Methodref, NameAndType, String, Utf8,
    };

    #[test]
    fn should_create_internal_map() {
        let cpool = vec![
            Empty, //                                               0
            Class {
                //                                                  1
                name_index: 2,
            },
            Utf8 {
                //                                                  2
                value: "Trivial$1LocalCls".into(),
            },
            Class {
                //                                                  3
                name_index: 4,
            },
            Utf8 {
                //                                                  4
                value: "java/lang/Object".into(),
            },
            Utf8 {
                //                                                  5
                value: "SourceFile".into(),
            },
            Utf8 {
                //                                                  6
                value: "Trivial.java".into(),
            },
            Utf8 {
                //                                                  7
                value: "EnclosingMethod".into(),
            },
            Class {
                //                                                  8
                name_index: 9,
            },
            Utf8 {
                //                                                  9
                value: "Trivial".into(),
            },
            NameAndType {
                //                                                  10
                name_index: 11,
                descriptor_index: 12,
            },
            Utf8 {
                //                                                  11
                value: "run".into(),
            },
            Utf8 {
                //                                                  12
                value: "()V".into(),
            },
            Utf8 {
                //                                                  13
                value: "NestHost".into(),
            },
            Utf8 {
                //                                                  14
                value: "InnerClasses".into(),
            },
            Utf8 {
                //                                                  15
                value: "LocalCls".into(),
            },
        ];
        let actual = ConstantPool::new(&cpool);

        let mut expected = HashMap::new();

        let mut empty = HashMap::new();
        empty.insert(0, Empty);
        expected.insert(CPoolType::Empty, empty);

        let mut class = HashMap::new();
        class.insert(1, Class { name_index: 2 });
        class.insert(3, Class { name_index: 4 });
        class.insert(8, Class { name_index: 9 });
        expected.insert(CPoolType::Class, class);

        let mut name_name_type = HashMap::new();
        name_name_type.insert(
            10,
            NameAndType {
                name_index: 11,
                descriptor_index: 12,
            },
        );
        expected.insert(CPoolType::NameAndType, name_name_type);

        let mut utf8 = HashMap::new();
        utf8.insert(
            2,
            Utf8 {
                value: "Trivial$1LocalCls".to_string(),
            },
        );
        utf8.insert(
            4,
            Utf8 {
                value: "java/lang/Object".to_string(),
            },
        );
        utf8.insert(
            5,
            Utf8 {
                value: "SourceFile".to_string(),
            },
        );
        utf8.insert(
            6,
            Utf8 {
                value: "Trivial.java".to_string(),
            },
        );
        utf8.insert(
            7,
            Utf8 {
                value: "EnclosingMethod".to_string(),
            },
        );
        utf8.insert(
            9,
            Utf8 {
                value: "Trivial".to_string(),
            },
        );
        utf8.insert(
            11,
            Utf8 {
                value: "run".to_string(),
            },
        );
        utf8.insert(
            12,
            Utf8 {
                value: "()V".to_string(),
            },
        );
        utf8.insert(
            13,
            Utf8 {
                value: "NestHost".to_string(),
            },
        );
        utf8.insert(
            14,
            Utf8 {
                value: "InnerClasses".to_string(),
            },
        );
        utf8.insert(
            15,
            Utf8 {
                value: "LocalCls".to_string(),
            },
        );
        expected.insert(CPoolType::Utf8, utf8);

        assert_eq!(expected, actual.data);
    }

    #[test]
    fn should_return_none_when_type_is_not_present() {
        let resolver = ConstantPool::new(&[Empty, Class { name_index: 10 }]);

        let actual = resolver.get(CPoolType::Double, 1);
        assert_eq!(None, actual)
    }

    #[test]
    fn should_return_none_when_index_in_not_matched() {
        let resolver = ConstantPool::new(&[Empty, Class { name_index: 10 }]);

        let actual = resolver.get(CPoolType::Class, 2);
        assert_eq!(None, actual)
    }

    #[test]
    fn should_return_value_when_type_and_index_are_present() {
        let resolver = ConstantPool::new(&[Empty, Class { name_index: 10 }]);

        let actual = resolver.get(CPoolType::Class, 1);
        assert_eq!(Some(Class { name_index: 10 }), actual.cloned())
    }

    #[test]
    fn should_return_class_name() {
        let resolver = ConstantPool::new(&[
            Empty,
            Class { name_index: 2 },
            Utf8 {
                value: "java/lang/Byte".to_string(),
            },
        ]);

        let actual = resolver.get_class_name(1);
        assert_eq!(Some("java/lang/Byte"), actual.as_deref())
    }

    #[test]
    fn should_return_full_field_info() {
        let resolver = ConstantPool::new(&[
            Empty,
            Class { name_index: 2 },
            Utf8 {
                value: "TheClass".to_string(),
            },
            Fieldref {
                class_index: 1,
                name_and_type_index: 4,
            },
            NameAndType {
                name_index: 5,
                descriptor_index: 6,
            },
            Utf8 {
                value: "theField".to_string(),
            },
            Utf8 {
                value: "I".to_string(),
            },
        ]);

        let actual = resolver.get_full_field_info(3);
        assert_eq!(
            Some((
                "TheClass".to_string(),
                "theField".to_string(),
                "I".to_string()
            )),
            actual
        );
    }

    #[test]
    fn should_return_full_method_info() {
        let resolver = ConstantPool::new(&[
            Empty,
            Class { name_index: 2 },
            Utf8 {
                value: "TheClass".to_string(),
            },
            Methodref {
                class_index: 1,
                name_and_type_index: 4,
            },
            NameAndType {
                name_index: 5,
                descriptor_index: 6,
            },
            Utf8 {
                value: "theMethod".to_string(),
            },
            Utf8 {
                value: "()V".to_string(),
            },
        ]);

        let actual = resolver.get_full_method_info(3);
        assert_eq!(
            Some((
                "TheClass".to_string(),
                "theMethod".to_string(),
                "()V".to_string()
            )),
            actual
        );
    }

    #[test]
    fn should_return_full_interfacemethod_info() {
        let resolver = ConstantPool::new(&[
            Empty,
            InterfaceMethodref {
                class_index: 2,
                name_and_type_index: 3,
            },
            Class { name_index: 4 },
            NameAndType {
                name_index: 5,
                descriptor_index: 6,
            },
            Utf8 {
                value: "Interface".to_string(),
            },
            Utf8 {
                value: "sub".to_string(),
            },
            Utf8 {
                value: "(II)I".to_string(),
            },
        ]);

        let actual = resolver.get_full_interfacemethodref_info(1);
        assert_eq!(
            Some((
                "Interface".to_string(),
                "sub".to_string(),
                "(II)I".to_string()
            )),
            actual
        );
    }

    #[test]
    fn should_return_name_and_type() {
        let resolver = ConstantPool::new(&[
            Empty,
            NameAndType {
                name_index: 2,
                descriptor_index: 3,
            },
            Utf8 {
                value: "theField".to_string(),
            },
            Utf8 {
                value: "J".to_string(),
            },
        ]);

        let actual = resolver.get_name_and_type(1);
        assert_eq!(Some(("theField".to_string(), "J".to_string())), actual);
    }

    #[test]
    fn should_return_integer() {
        let resolver = ConstantPool::new(&[Empty, Class { name_index: 2 }, Integer { value: 42 }]);

        let actual = resolver.get_integer(2);
        assert_eq!(Some(42), actual)
    }

    #[test]
    fn should_return_long() {
        let resolver = ConstantPool::new(&[
            Empty,
            Class { name_index: 2 },
            Long {
                value: 9_000_000_000,
            },
        ]);

        let actual = resolver.get_long(2);
        assert_eq!(Some(9_000_000_000), actual)
    }

    #[test]
    fn should_return_float() {
        let resolver = ConstantPool::new(&[Empty, Class { name_index: 2 }, Float { value: 4.25 }]);

        let actual = resolver.get_float(2);
        assert_eq!(Some(4.25), actual)
    }

    #[test]
    fn should_return_double() {
        let resolver = ConstantPool::new(&[
            Empty,
            Class { name_index: 2 },
            Double { value: 4.2217E-105 },
        ]);

        let actual = resolver.get_double(2);
        assert_eq!(Some(4.2217E-105), actual)
    }

    #[test]
    fn should_return_class_as_string() {
        let resolver = ConstantPool::new(&[
            Empty,
            Class { name_index: 2 },
            Utf8 {
                value: "java/lang/Byte".to_string(),
            },
        ]);

        let actual = resolver.get_class_name(1);
        assert_eq!(Some("java/lang/Byte".to_string()), actual)
    }

    #[test]
    fn should_return_current_class_as_string() {
        let resolver = ConstantPool::new_with_classname(
            &[
                Empty,
                Class { name_index: 2 },
                Utf8 {
                    value: "invokedynamic/lambda/LambdaExample$$Lambda".to_string(),
                },
            ],
            1,
            "invokedynamic/lambda/LambdaExample$$Lambda/0x0000000000000001".to_string(),
        );

        let actual = resolver.get_class_name(1);
        assert_eq!(
            Some("invokedynamic/lambda/LambdaExample$$Lambda/0x0000000000000001".to_string()),
            actual
        )
    }

    #[test]
    fn should_return_string_as_utf8() {
        let resolver = ConstantPool::new(&[
            Empty,
            String { string_index: 2 },
            Utf8 {
                value: "int".to_string(),
            },
        ]);

        let actual = resolver.get_string(1);
        assert_eq!(Some("int".to_string()), actual)
    }

    #[test]
    fn should_return_invoke_dynamic() {
        let resolver = ConstantPool::new(&[
            Empty,
            InvokeDynamic {
                bootstrap_method_attr_index: 42,
                name_and_type_index: 2,
            },
            NameAndType {
                name_index: 3,
                descriptor_index: 4,
            },
            Utf8 {
                value: "fn".to_string(),
            },
            Utf8 {
                value: "()V".to_string(),
            },
        ]);

        let actual = resolver.get_invoke_dynamic(1);
        assert_eq!(Some((42, "fn".to_string(), "()V".to_string())), actual);
    }

    #[test]
    fn should_return_method_handle() {
        let resolver = ConstantPool::new(&vec![
            Empty,
            MethodHandle {
                reference_kind: 6,
                reference_index: 2,
            },
            Methodref {
                class_index: 3,
                name_and_type_index: 5,
            },
            Class { name_index: 4 },
            Utf8 {
                value: "SomeClass".to_string(),
            },
            NameAndType {
                name_index: 6,
                descriptor_index: 7,
            },
            Utf8 {
                value: "theMethod".to_string(),
            },
            Utf8 {
                value: "()V".to_string(),
            },
            MethodHandle {
                reference_kind: 3,
                reference_index: 9,
            },
            Fieldref {
                class_index: 3,
                name_and_type_index: 10,
            },
            NameAndType {
                name_index: 11,
                descriptor_index: 12,
            },
            Utf8 {
                value: "theField".to_string(),
            },
            Utf8 {
                value: "I".to_string(),
            },
            MethodHandle {
                reference_kind: 9,
                reference_index: 14,
            },
            InterfaceMethodref {
                class_index: 3,
                name_and_type_index: 15,
            },
            NameAndType {
                name_index: 16,
                descriptor_index: 17,
            },
            Utf8 {
                value: "theInterfaceMethod".to_string(),
            },
            Utf8 {
                value: "()I".to_string(),
            },
        ]);

        let actual_field_info = resolver.get_method_handle(8);
        assert_eq!(
            Some((
                3,
                "SomeClass".to_string(),
                "theField".to_string(),
                "I".to_string()
            )),
            actual_field_info
        );

        let actual_method_info = resolver.get_method_handle(1);
        assert_eq!(
            Some((
                6,
                "SomeClass".to_string(),
                "theMethod".to_string(),
                "()V".to_string()
            )),
            actual_method_info
        );

        let actual_interfacemethod_info = resolver.get_method_handle(13);
        assert_eq!(
            Some((
                9,
                "SomeClass".to_string(),
                "theInterfaceMethod".to_string(),
                "()I".to_string()
            )),
            actual_interfacemethod_info
        );
    }

    #[test]
    fn should_return_method_type() {
        let resolver = ConstantPool::new(&[
            Empty,
            MethodType {
                descriptor_index: 2,
            },
            Utf8 {
                value: "()V".to_string(),
            },
        ]);

        let actual = resolver.get_method_type(1);
        assert_eq!(Some("()V".to_string()), actual)
    }
}
