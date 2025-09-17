use std::path::PathBuf;
use bincode::{Decode, Encode};
use crate::errors::soul_error::SoulError;

#[derive(Debug, Clone, Encode, Decode)]
pub struct SoulFault {
    pub msg: SoulError,
    pub file: PathBuf,
    pub kind: SoulFaultKind,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Encode, Decode)]
pub enum SoulFaultKind {
    Note,
    Error,
    Warning,
}

