#[derive(Debug, Clone, PartialEq)]
pub struct FieldAccess {
    /// None = use default (e.g. pub)
    pub get: Option<Visibility>, 
    // None = disallow set
    pub set: Option<Visibility>, 
}

impl Default for FieldAccess  {
    fn default() -> Self {
        Self { get: None, set: None }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
}






















