use crate::method_area::method_area::with_method_area;

/*
 * Access modifier flag constants from tables 4.1, 4.4, 4.5, and 4.7 of
 * The Java Virtual Machine Specification
 */
const PUBLIC: u16 = 0x00000001;
const PRIVATE: u16 = 0x00000002;
const PROTECTED: u16 = 0x00000004;
const STATIC: u16 = 0x00000008;
const FINAL: u16 = 0x00000010;
const _SYNCHRONIZED: u16 = 0x00000020;
const _VOLATILE: u16 = 0x00000040;
const _TRANSIENT: u16 = 0x00000080;
const _NATIVE: u16 = 0x00000100;
const INTERFACE: u16 = 0x00000200;
const ABSTRACT: u16 = 0x00000400;
const STRICT: u16 = 0x00000800;

const MODIFIERS: u16 =
    PUBLIC | PROTECTED | PRIVATE | ABSTRACT | STATIC | FINAL | STRICT | INTERFACE;

pub(crate) fn get_modifiers_wrp(args: &[i32]) -> Vec<i32> {
    let modifiers = get_modifiers(args[0]);

    vec![modifiers]
}
fn get_modifiers(reference: i32) -> i32 {
    with_method_area(|method_area| {
        let rc = method_area
            .get_from_reflection_table(reference)
            .expect("error getting method area");
        let access_flags = rc.access_flags();

        access_flags & MODIFIERS
    }) as i32
}
