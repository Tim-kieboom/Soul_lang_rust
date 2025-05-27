use super::access_level::AccesLevel;

#[derive(Debug, Clone, PartialEq)]
pub struct MethodeInfo {
    pub name: String,
    // pub args: Vec<u8>,
    pub access: AccesLevel,
}


