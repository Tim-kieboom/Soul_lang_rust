use crate::{errors::soul_error::SoulError};

///could be error or warning or note
pub struct SoulFault {
    pub err: SoulError, 
    pub kind: SoulFaultKind,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum SoulFaultKind {
    Error,
    Warning,
    Note,
}

impl SoulFault {

    pub fn new_error(err: SoulError) -> Self {
        Self { err, kind: SoulFaultKind::Error }
    }

    pub fn new_warning(err: SoulError) -> Self {
        Self { err, kind: SoulFaultKind::Warning }
    }

    pub fn new_note(err: SoulError) -> Self {
        Self { err, kind: SoulFaultKind::Note }
    }

    pub fn is_error(&self) -> bool {
        match self.kind {
            SoulFaultKind::Error => true,
            SoulFaultKind::Note |
            SoulFaultKind::Warning => false,
        }
    }
}











