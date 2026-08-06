use bitflags::bitflags;

bitflags! {
    /// Possible class modifiers (JVMS §4.1, Table 4.1-B `access_flags`).
    ///
    /// These are the flags of the `ClassFile` structure itself, so the set differs from
    /// `java.lang.reflect.Modifier`: `private`, `protected` and `static` are not recorded here for
    /// a nested class, they live in the `InnerClasses` entry that describes it (JVMS §4.7.6).
    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
    pub struct ClassModifier: u16 {
        const Public = 0x0001;
        const Final = 0x0010;
        /// Treated as set by the JVM since Java SE 8, whatever the class file says.
        const Super = 0x0020;
        const Interface = 0x0200;
        const Abstract = 0x0400;
        const Synthetic = 0x1000;
        const Annotation = 0x2000;
        const Enum = 0x4000;
        /// A module declaration (`module-info.class`) rather than a class.
        const Module = 0x8000;
    }
}

bitflags! {
    /// Possible modifiers of a nested class, from the `InnerClasses` attribute (JVMS §4.7.6,
    /// Table 4.7.6-A `inner_class_access_flags`).
    ///
    /// This is where `private`, `protected` and `static` are recorded for a nested class; the
    /// `ClassFile`'s own [`ClassModifier`] cannot express them.
    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
    pub struct NestedClassModifier: u16 {
        const Public = 0x0001;
        const Private = 0x0002;
        const Protected = 0x0004;
        const Static = 0x0008;
        const Final = 0x0010;
        const Interface = 0x0200;
        const Abstract = 0x0400;
        const Synthetic = 0x1000;
        const Annotation = 0x2000;
        const Enum = 0x4000;
    }
}

bitflags! {
    /// Possible modifiers of a formal parameter, from the `MethodParameters` attribute
    /// (JVMS §4.7.24, Table 4.7.24-A `access_flags`).
    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
    pub struct ParameterModifier: u16 {
        const Final = 0x0010;
        /// Not present in the source code: an artifact of the compiler.
        const Synthetic = 0x1000;
        /// Implicitly declared by the source language, such as the outer instance passed to an
        /// inner class constructor.
        const Mandated = 0x8000;
    }
}

bitflags! {
    /// Possible field modifiers (JVMS §4.5, `access_flags`).
    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
    pub struct FieldModifier: u16 {
        const Public = 0x0001;
        const Private = 0x0002;
        const Protected = 0x0004;
        const Static = 0x0008;
        const Final = 0x0010;
        const Volatile = 0x0040;
        const Transient = 0x0080;
        const Synthetic = 0x1000;
        const Enum = 0x4000;
    }
}

bitflags! {
    /// Possible method modifiers (JVMS §4.6, `access_flags`).
    #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
    pub struct MethodModifier: u16 {
        const Public = 0x0001;
        const Private = 0x0002;
        const Protected = 0x0004;
        const Static = 0x0008;
        const Final = 0x0010;
        const Synchronized = 0x0020;
        const Bridge = 0x0040;
        const Varargs = 0x0080;
        const Native = 0x0100;
        const Abstract = 0x0400;
        /// Only meaningful below class file version 61.0; `strictfp` became a no-op in Java 17.
        const Strict = 0x0800;
        const Synthetic = 0x1000;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pinned against the JVMS tables so these sets cannot drift from `jclassfile`'s flags again.
    /// A bit missing here is discarded by `from_bits_truncate` without a trace, which is how
    /// `Super` and `Module` went missing from [`ClassModifier`] in the first place.
    #[test]
    fn class_modifiers_should_match_jvms_table_4_1_b() {
        assert_eq!(0x0001, ClassModifier::Public.bits());
        assert_eq!(0x0010, ClassModifier::Final.bits());
        assert_eq!(0x0020, ClassModifier::Super.bits());
        assert_eq!(0x0200, ClassModifier::Interface.bits());
        assert_eq!(0x0400, ClassModifier::Abstract.bits());
        assert_eq!(0x1000, ClassModifier::Synthetic.bits());
        assert_eq!(0x2000, ClassModifier::Annotation.bits());
        assert_eq!(0x4000, ClassModifier::Enum.bits());
        assert_eq!(0x8000, ClassModifier::Module.bits());
        assert_eq!(0xF631, ClassModifier::all().bits());
    }

    #[test]
    fn field_modifiers_should_match_jvms_table_4_5_a() {
        assert_eq!(0x50DF, FieldModifier::all().bits());
    }

    #[test]
    fn nested_class_modifiers_should_match_jvms_table_4_7_6_a() {
        assert_eq!(0x761F, NestedClassModifier::all().bits());
    }

    #[test]
    fn parameter_modifiers_should_match_jvms_table_4_7_24_a() {
        assert_eq!(0x9010, ParameterModifier::all().bits());
    }

    #[test]
    fn method_modifiers_should_match_jvms_table_4_6_a() {
        assert_eq!(0x1DFF, MethodModifier::all().bits());
    }

    /// The two bits the audit found were being truncated away.
    #[test]
    fn class_modifiers_should_keep_super_and_module() {
        assert_eq!(
            ClassModifier::Public | ClassModifier::Super,
            ClassModifier::from_bits_truncate(0x0021)
        );
        assert_eq!(
            ClassModifier::Module,
            ClassModifier::from_bits_truncate(0x8000)
        );
    }
}
