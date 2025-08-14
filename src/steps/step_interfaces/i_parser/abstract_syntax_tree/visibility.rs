use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldAccess {
    /// None = use default (e.g. pub)
    pub get: Option<Visibility>, 
    // None = disallow set
    pub set: Option<Visibility>, 
}

impl FieldAccess {
    pub fn new_public() -> Self {
        Self{ get: Some(Visibility::Public), set: Some(Visibility::Public)}
    }

    pub fn new_private() -> Self {
        Self{ get: Some(Visibility::Private), set: Some(Visibility::Private)}
    }
}

impl Default for FieldAccess  {
    fn default() -> Self {
        Self { get: None, set: None }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
}






















