use bitflags::bitflags;

bitflags! {
    pub struct MemberAccess: u8 {
        const EMPTY = 0b0000_0000;
        const PUBLIC_GETTER = 0b0000_0001;
        const PUBLIC_SETTER = 0b0000_0010;
        const HAS_WITH = 0b0000_0100;
        const PUBLIC_WITH = 0b0000_1000;
    }
}
pub struct MemberInfo {
    pub type_name: String,
    pub access: MemberAccess,
}