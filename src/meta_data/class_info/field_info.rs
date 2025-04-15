use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone)]
    pub struct FieldAccess: u8 {
        const EMPTY = 0b0000_0000;
        const PUBLIC_GETTER = 0b0000_0001;
        const PUBLIC_SETTER = 0b0000_0010;
        const HAS_WITH = 0b0000_0100;
        const PUBLIC_WITH = 0b0000_1000;
    }
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub type_name: String,
    pub access: FieldAccess,
}