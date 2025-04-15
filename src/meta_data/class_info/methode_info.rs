use super::access_level::AccesLevel;

#[derive(Debug, Clone)]
pub struct MethodeInfo {
    pub name: String,
    // pub args: Vec<u8>,
    pub access: AccesLevel,
}


