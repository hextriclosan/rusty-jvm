use crate::method_area::primitives_helper::PRIMITIVE_TYPE_BY_CODE;

pub fn decorate(type_name: String) -> String {
    if PRIMITIVE_TYPE_BY_CODE.contains_key(type_name.as_str())
        || (type_name.starts_with('L') && type_name.ends_with(';'))
    {
        type_name
    } else {
        format!("L{};", type_name)
    }
}
