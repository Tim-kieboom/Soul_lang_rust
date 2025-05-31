use std::io::{Error, ErrorKind};
use crate::tokenizer::token::Token;


#[cfg(feature = "throw_result")]
pub fn new_soul_error(token: &Token, msg: &str) -> Error {
    use std::backtrace::Backtrace;
    Error::new(ErrorKind::InvalidData, format!("{}\n\n\n\nat {}:{}; !!error!! {}", Backtrace::capture(), token.line_number, token.line_offset, msg))
}

#[cfg(not(feature = "throw_result"))]
pub fn new_soul_error(token: &Token, msg: &str) -> Error {
    Error::new(ErrorKind::InvalidData, format!("at {}:{}; !!error!! {}", token.line_number, token.line_offset, msg))
}