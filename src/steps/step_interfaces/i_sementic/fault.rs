use crate::{errors::soul_error::SoulError};

///could be error or warning
pub enum SoulFault {
    Error(SoulError),
    Warning(SoulError),
}

impl SoulFault {

    pub fn is_error(&self) -> bool {
        match self {
            SoulFault::Error(_) => true,
            SoulFault::Warning(_) => false,
        }
    }

    pub fn consume(self) -> SoulError {
        match self {
            SoulFault::Error(soul_error) => soul_error,
            SoulFault::Warning(soul_error) => soul_error,
        }
    }
}











