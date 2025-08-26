use bitflags::bitflags;

bitflags! {
    /// Possible class modifiers
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub struct ClassModifier: u16 {
        const Public = 0x0001;
        const Private = 0x0002;
        const Protected = 0x0004;
        const Static = 0x0008;
        const Final = 0x0010;
        const Interface = 0x0200;
        const Abstract = 0x0400;
        const Strict = 0x0800;
        const Synthetic = 0x1000;
        const Annotation = 0x2000;
        const Enum = 0x4000;
    }
}
