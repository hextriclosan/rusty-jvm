use crate::vm::method_area::primitives_helper::PRIMITIVE_TYPE_BY_CODE;

pub fn decorate(type_name: String) -> String {
    if PRIMITIVE_TYPE_BY_CODE.contains_key(type_name.as_str()) // primitive type B, C, D, F, I, J, S, Z, V
        || (type_name.starts_with('[') && PRIMITIVE_TYPE_BY_CODE.contains_key(&type_name[1..])) // array of primitive types [B, [C, [D, [F, [I, [J, [S, [Z, [V
        || ((type_name.starts_with('L') || type_name.starts_with('[')) && type_name.ends_with(';'))
    {
        type_name
    } else {
        format!("L{};", type_name)
    }
}
