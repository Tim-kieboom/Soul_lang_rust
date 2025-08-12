use crate::{errors::soul_error::SoulError};

///could be error or warning
pub enum SoulFault {
    Error(SoulError),
    Warning(SoulError),
    Note(SoulError),
}

impl SoulFault {

    pub fn is_error(&self) -> bool {
        match self {
            SoulFault::Error(_) => true,
            SoulFault::Note(_) |
            SoulFault::Warning(_) => false,
        }
    }

    pub fn get_soul_error(&self) -> &SoulError {
        match self {
            SoulFault::Note(soul_error) => soul_error,
            SoulFault::Error(soul_error) => soul_error,
            SoulFault::Warning(soul_error) => soul_error,
        }
    }

    pub fn consume(self) -> SoulError {
        match self {
            SoulFault::Note(soul_error) => soul_error,
            SoulFault::Error(soul_error) => soul_error,
            SoulFault::Warning(soul_error) => soul_error,
        }
    }
}











