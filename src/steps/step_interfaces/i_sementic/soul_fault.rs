use bincode::{Decode, Encode};
use crate::errors::soul_error::SoulError;

#[derive(Debug, Clone, Encode, Decode)]
pub struct SoulFault {
    pub msg: SoulError,
    pub kind: SoulFaultKind,
}

impl SoulFault {

    pub fn new_error(msg: SoulError) -> Self {
        Self{ msg, kind: SoulFaultKind::Error}
    }

    pub fn new_warning(msg: SoulError) -> Self {
        Self{ msg, kind: SoulFaultKind::Warning}
    }

    pub fn new_note(msg: SoulError) -> Self {
        Self{ msg, kind: SoulFaultKind::Note}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
pub enum SoulFaultKind {
    Note,
    Error,
    Warning,
}

