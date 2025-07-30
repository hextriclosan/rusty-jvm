use num_enum::TryFromPrimitive;

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
pub enum ReferenceKind {
    REF_getField = 1,
    REF_getStatic = 2,
    REF_putField = 3,
    REF_putStatic = 4,
    REF_invokeVirtual = 5,
    REF_invokeStatic = 6,
    REF_invokeSpecial = 7,
    REF_newInvokeSpecial = 8,
    REF_invokeInterface = 9,
}
