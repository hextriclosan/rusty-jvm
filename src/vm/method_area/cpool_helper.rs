use jclassfile::constant_pool::ConstantPool;
use std::collections::HashMap;

#[cfg_attr(test, mockall::automock)]
pub trait CPoolHelperTrait {
    fn get<'a>(&'a self, ctype: CPoolType, index: u16) -> Option<&'a ConstantPool>;
    fn get_first<'a>(&'a self, ctypes: &[CPoolType], index: u16) -> Option<&'a ConstantPool>;
    fn get_class_name(&self, index: u16) -> Option<String>;
    fn get_integer(&self, index: u16) -> Option<i32>;
    fn get_float(&self, index: u16) -> Option<f32>;
    fn get_double(&self, index: u16) -> Option<f64>;
    fn get_class(&self, index: u16) -> Option<String>;
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
pub struct CPoolHelper {
    data: HashMap<CPoolType, HashMap<u16, ConstantPool>>,
    raw_cpool: Vec<ConstantPool>,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum CPoolType {
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

impl From<&ConstantPool> for CPoolType {
    fn from(item: &ConstantPool) -> Self {
        match item {
            ConstantPool::Empty => CPoolType::Empty,
            ConstantPool::Utf8 { .. } => CPoolType::Utf8,
            ConstantPool::Integer { .. } => CPoolType::Integer,
            ConstantPool::Float { .. } => CPoolType::Float,
            ConstantPool::Long { .. } => CPoolType::Long,
            ConstantPool::Double { .. } => CPoolType::Double,
            ConstantPool::Class { .. } => CPoolType::Class,
            ConstantPool::String { .. } => CPoolType::String,
            ConstantPool::Fieldref { .. } => CPoolType::Fieldref,
            ConstantPool::Methodref { .. } => CPoolType::Methodref,
            ConstantPool::InterfaceMethodref { .. } => CPoolType::InterfaceMethodref,
            ConstantPool::NameAndType { .. } => CPoolType::NameAndType,
            ConstantPool::MethodHandle { .. } => CPoolType::MethodHandle,
            ConstantPool::MethodType { .. } => CPoolType::MethodType,
            ConstantPool::Dynamic { .. } => CPoolType::Dynamic,
            ConstantPool::InvokeDynamic { .. } => CPoolType::InvokeDynamic,
            ConstantPool::Module { .. } => CPoolType::Module,
            ConstantPool::Package { .. } => CPoolType::Package,
        }
    }
}

impl CPoolHelper {
    pub fn new(cpool: &[ConstantPool]) -> Self {
        let mut data: HashMap<CPoolType, HashMap<u16, ConstantPool>> = HashMap::new();

        for (index, item) in cpool.iter().enumerate() {
            let ctype = item.into();
            let entry = data.entry(ctype).or_insert_with(HashMap::new);
            entry.insert(index as u16, item.clone());
        }

        Self {
            data,
            raw_cpool: cpool.to_vec(),
        }
    }

    pub fn raw_cpool(&self) -> &Vec<ConstantPool> {
        &self.raw_cpool
    }
}

impl CPoolHelperTrait for CPoolHelper {
    fn get(&self, ctype: CPoolType, index: u16) -> Option<&ConstantPool> {
        self.data.get(&ctype)?.get(&index)
    }

    fn get_first(&self, ctypes: &[CPoolType], index: u16) -> Option<&ConstantPool> {
        for &ctype in ctypes {
            if let Some(constant_pool) = self.get(ctype, index) {
                return Some(constant_pool);
            }
        }
        None
    }

    fn get_class_name(&self, index: u16) -> Option<String> {
        let name_index = match *self.get(CPoolType::Class, index)? {
            ConstantPool::Class { name_index } => name_index,
            _ => return None,
        };

        self.get_utf8(name_index)
    }

    fn get_integer(&self, index: u16) -> Option<i32> {
        match self.get(CPoolType::Integer, index)? {
            ConstantPool::Integer { value } => Some(*value),
            _ => None,
        }
    }

    fn get_float(&self, index: u16) -> Option<f32> {
        match self.get(CPoolType::Float, index)? {
            ConstantPool::Float { value } => Some(*value),
            _ => None,
        }
    }

    fn get_double(&self, index: u16) -> Option<f64> {
        match self.get(CPoolType::Double, index)? {
            ConstantPool::Double { value } => Some(*value),
            _ => None,
        }
    }

    fn get_class(&self, index: u16) -> Option<String> {
        let name_index = match self.get(CPoolType::Class, index)? {
            ConstantPool::Class { name_index } => Some(name_index),
            _ => None,
        }?;

        self.get_utf8(*name_index)
    }

    fn get_string(&self, index: u16) -> Option<String> {
        let name_index = match self.get(CPoolType::String, index)? {
            ConstantPool::String { string_index } => Some(string_index),
            _ => None,
        }?;

        self.get_utf8(*name_index)
    }

    fn get_long(&self, index: u16) -> Option<i64> {
        match self.get(CPoolType::Long, index)? {
            ConstantPool::Long { value } => Some(*value),
            _ => None,
        }
    }

    fn get_utf8(&self, index: u16) -> Option<String> {
        match self.get(CPoolType::Utf8, index)? {
            ConstantPool::Utf8 { value } => Some(value.clone()),
            _ => None,
        }
    }

    fn get_full_field_info(&self, index: u16) -> Option<(String, String, String)> {
        let (class_index, name_and_type_index) = match self.get(CPoolType::Fieldref, index)? {
            ConstantPool::Fieldref {
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

        let (class_index, name_and_type_index) = if let ConstantPool::Methodref {
            class_index,
            name_and_type_index,
        }
        | ConstantPool::InterfaceMethodref {
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
                ConstantPool::InterfaceMethodref {
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
            ConstantPool::NameAndType {
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
                ConstantPool::InvokeDynamic {
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
            ConstantPool::MethodHandle {
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
            ConstantPool::MethodType { descriptor_index } => Some(descriptor_index),
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
        let actual = CPoolHelper::new(&cpool);

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
        let resolver = CPoolHelper::new(&vec![Empty, Class { name_index: 10 }]);

        let actual = resolver.get(CPoolType::Double, 1);
        assert_eq!(None, actual)
    }

    #[test]
    fn should_return_none_when_index_in_not_matched() {
        let resolver = CPoolHelper::new(&vec![Empty, Class { name_index: 10 }]);

        let actual = resolver.get(CPoolType::Class, 2);
        assert_eq!(None, actual)
    }

    #[test]
    fn should_return_value_when_type_and_index_are_present() {
        let resolver = CPoolHelper::new(&vec![Empty, Class { name_index: 10 }]);

        let actual = resolver.get(CPoolType::Class, 1);
        assert_eq!(Some(Class { name_index: 10 }), actual.cloned())
    }

    #[test]
    fn should_return_class_name() {
        let resolver = CPoolHelper::new(&vec![
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
        let resolver = CPoolHelper::new(&vec![
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
        let resolver = CPoolHelper::new(&vec![
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
        let resolver = CPoolHelper::new(&vec![
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
        let resolver = CPoolHelper::new(&vec![
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
        let resolver =
            CPoolHelper::new(&vec![Empty, Class { name_index: 2 }, Integer { value: 42 }]);

        let actual = resolver.get_integer(2);
        assert_eq!(Some(42), actual)
    }

    #[test]
    fn should_return_long() {
        let resolver = CPoolHelper::new(&vec![
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
        let resolver =
            CPoolHelper::new(&vec![Empty, Class { name_index: 2 }, Float { value: 3.14 }]);

        let actual = resolver.get_float(2);
        assert_eq!(Some(3.14), actual)
    }

    #[test]
    fn should_return_double() {
        let resolver = CPoolHelper::new(&vec![
            Empty,
            Class { name_index: 2 },
            Double { value: 4.2217E-105 },
        ]);

        let actual = resolver.get_double(2);
        assert_eq!(Some(4.2217E-105), actual)
    }

    #[test]
    fn should_return_class_as_string() {
        let resolver = CPoolHelper::new(&vec![
            Empty,
            Class { name_index: 2 },
            Utf8 {
                value: "java/lang/Byte".to_string(),
            },
        ]);

        let actual = resolver.get_class(1);
        assert_eq!(Some("java/lang/Byte".to_string()), actual)
    }

    #[test]
    fn should_return_string_as_utf8() {
        let resolver = CPoolHelper::new(&vec![
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
        let resolver = CPoolHelper::new(&vec![
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
        let resolver = CPoolHelper::new(&vec![
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
        let resolver = CPoolHelper::new(&vec![
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
