pub mod run_options;
pub mod cache_file;
pub mod soul_names; 
pub mod run_steps;
pub mod errors;
pub mod utils;
pub mod steps;

use crate::errors::soul_error::{pass_soul_error, Result, SoulErrorKind, SoulSpan};

pub trait MainErrMap<T>{fn main_err_map(self, msg: &str) -> Result<T>;}
impl<T> MainErrMap<T> for Result<T> {
    fn main_err_map(self, msg: &str) -> Result<T> {
        self.map_err(|child| pass_soul_error(SoulErrorKind::NoKind, SoulSpan::new(0, 0, 0), msg, child))
    }
}



