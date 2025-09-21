use std::path::PathBuf;
use bincode::{Decode, Encode};
use crate::errors::soul_error::SoulError;

#[derive(Debug, Clone, Encode, Decode)]
pub struct SoulFault {
    pub msg: SoulError,
    pub file: PathBuf,
    pub kind: SoulFaultKind,
}

impl SoulFault {

    pub fn new_error(msg: SoulError, file: PathBuf) -> Self {
        Self{ msg, file, kind: SoulFaultKind::Error}
    }

    pub fn new_warning(msg: SoulError, file: PathBuf) -> Self {
        Self{ msg, file, kind: SoulFaultKind::Warning}
    }

    pub fn new_note(msg: SoulError, file: PathBuf) -> Self {
        Self{ msg, file, kind: SoulFaultKind::Note}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
pub enum SoulFaultKind {
    Note,
    Error,
    Warning,
}

