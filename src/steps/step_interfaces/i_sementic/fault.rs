use crate::{errors::soul_error::SoulError};

///could be error or warning
pub enum SoulFault {
    Error(SoulError),
    Warning(SoulError),
}













