use once_cell::sync::Lazy;
use std::collections::HashMap;

pub(crate) static PRIMITIVE_TYPE_BY_CODE: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert("B", "byte");
        map.insert("C", "char");
        map.insert("D", "double");
        map.insert("F", "float");
        map.insert("I", "int");
        map.insert("J", "long");
        map.insert("S", "short");
        map.insert("Z", "boolean");
        map.insert("V", "void");
        map
    });

pub(crate) static PRIMITIVE_CODE_BY_TYPE: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| {
        PRIMITIVE_TYPE_BY_CODE
            .iter()
            .map(|(k, v)| (*v, *k))
            .collect()
    });
