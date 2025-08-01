use crate::vm::error::Error;
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

const LOOKUP_CLASS_NAME: &'static str = "java/lang/invoke/MethodHandles$Lookup";

impl ReferenceKind {
    pub fn to_findmethod_signature(
        &self,
    ) -> crate::vm::error::Result<(&'static str, &'static str)> {
        let signature = match &self {
            ReferenceKind::REF_invokeStatic =>
                "findStatic:(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;",
            ReferenceKind::REF_invokeInterface | ReferenceKind::REF_invokeVirtual =>
                "findVirtual:(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/MethodHandle;",
            _ =>
                return Err(Error::new_execution(&format!(
                    "Unsupported yet reference kind for invokedynamic: {self:?}"
                )))
        };

        Ok((LOOKUP_CLASS_NAME, signature))
    }
}
